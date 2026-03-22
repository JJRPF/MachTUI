//! Oracle Demo: Headless mode for AI consumption.

use machtui::oracle::SemanticNode;
use machtui::talon::{Cmd, Model, Program};

#[derive(Debug)]
struct MockUI {
    is_admin: bool,
    status: String,
}

#[derive(Debug)]
enum Msg {
    Login,
}

impl Model for MockUI {
    type Message = Msg;
    fn update(&mut self, _msg: Self::Message) -> Option<Cmd<Self::Message>> {
        self.is_admin = true;
        None
    }
    fn view(&self) -> String {
        format!("Admin: {}", self.is_admin)
    }
    fn semantic_view(&self) -> SemanticNode {
        let mut root = SemanticNode::new("window");
        root.add_child(SemanticNode::new("status_bar").with_content(&self.status));
        root.add_child(SemanticNode::new("button").with_content("Login"));
        root
    }
}

#[tokio::main]
async fn main() {
    let ui = MockUI {
        is_admin: false,
        status: "Ready".to_string(),
    };
    let mut prog = Program::new(ui);

    println!("--- MachTUI Oracle (AI Perspective) ---");
    println!("Oracle Semantic Tree Output:");
    println!("{}", prog.oracle_json());

    println!("\nSending 'Login' message to Program...");
    prog.dispatch(Msg::Login);
    prog.update().await;

    println!("\nUpdated Oracle Tree:");
    println!("{}", prog.oracle_json());
}
