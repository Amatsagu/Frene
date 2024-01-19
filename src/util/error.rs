use std::fmt;


pub enum FreneError {
    ImageError(image::ImageError),
    IOError(std::io::Error)
}

impl fmt::Display for FreneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageError(e) => e.fmt(f),
            Self::IOError(e) => e.fmt(f),
        }
    }
}

impl fmt::Debug for FreneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error at {}:{}", file!(), line!())?;
        write!(f, "Error message: ")?;
        match self {
            Self::ImageError(e) => e.fmt(f),
            Self::IOError(e) => e.fmt(f),
        }
    }
}

impl From<image::ImageError> for FreneError {
    fn from(err: image::ImageError) -> Self {
        Self::ImageError(err)
    }
}

impl From<std::io::Error> for FreneError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}
