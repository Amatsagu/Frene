use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Frene", about = "A small, utility tool to help you generate color-schemes & blur your background images on the fly.")]
#[command(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Take png/jpg image and produce blurred version of it
    #[clap(arg_required_else_help = true)]
    Blur {
        file_path: String,
        strength: f32
    },
    /// Analyze png/jpg image and create color-scheme based on most used colors in image
    #[clap(arg_required_else_help = true)]
    Scheme {
        file_path: String,
    },
}