
use ui::{App, ClientTab, Context};

mod ui;
mod project;

fn main() {

    let mut server = alisa::Server::new("test.project", ()).unwrap();
    let (client_id, welcome_data) = server.add_client();
    let client = alisa::Client::collab(&welcome_data).unwrap();

    pierro::run(App {
        context: Context {
            server
        },
        docking: pierro::DockingState::new(vec![
            ClientTab {
                client_id,
                client,
                outgoing_msgs: Vec::new()
            }
        ])
    });
}
