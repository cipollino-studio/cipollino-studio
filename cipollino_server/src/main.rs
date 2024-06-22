
use cipollino_project::{project::Project, server::ProjectServer};

#[tokio::main]
async fn main() {
    ProjectServer::start("127.0.0.1:2000".to_owned(), Project::new(24.0, 44100.0), 2).await
}
