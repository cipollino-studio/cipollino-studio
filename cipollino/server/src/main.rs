
use clap::Parser;
use warp::Filter;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

mod server;
use server::*;

#[derive(clap::Parser)]
#[command(about, long_about = None)]
struct Args {
    #[arg(long, default_value = "8000")]
    port: u16,
    #[arg(long, default_value = "project.cip")]
    path: PathBuf
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let server = Arc::new(Mutex::new(Server::new(args.path)));

    let websocket_server = warp::ws().map(move |socket: warp::ws::Ws| {
        let server = server.clone();
        socket.on_upgrade(move |socket| {
            Server::handle_connection(server, socket)
        })
    });

    let routes = websocket_server;

    let address = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    let address = std::net::SocketAddr::new(address, args.port);

    warp::serve(routes)
        .run(address)
        .await;
}
