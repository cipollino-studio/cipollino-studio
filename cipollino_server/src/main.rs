
use std::path::PathBuf;

use cipollino_project::{serialization::{create_project, open_project}, server::ProjectServer};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cipollino_server")]
#[command(about = "Cipollino Collaboration Server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command 
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Create {
        #[arg(value_name = "PATH")] 
        path: PathBuf,
        #[arg(value_name = "FRAME RATE", default_value_t = 24.0)]
        fps: f32,
        #[arg(value_name = "SAMPLE RATE", default_value_t = 44100.0)]
        sample_rate: f32
    },
    Start {
        #[arg(value_name = "PATH")]
        path: PathBuf,
        #[arg(value_name = "PORT", default_value_t = 2000)]
        port: u32
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Create { path, fps, sample_rate } => {
            create_project(path, fps, sample_rate);
        },
        Command::Start { path, port } => {
            let Some((serializer, curr_key, project)) = open_project(path.clone()) else {
                eprintln!("Could not open project at {}.", path.to_string_lossy());
                return;
            };
            ProjectServer::start(format!("127.0.0.1:{}", port), project, curr_key, serializer).await
        }
    }
}
