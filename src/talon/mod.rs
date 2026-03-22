//! The "Talon" State Engine
//!
//! Strict Model-View-Update (MVU) architecture with Async Command support.

use std::fmt::Debug;
use crate::oracle::SemanticNode;
use futures::future::BoxFuture;

/// A Command represents an asynchronous side-effect that eventually produces a message.
pub type Cmd<M> = BoxFuture<'static, Option<M>>;

/// Represents the global state of the application.
pub trait Model: Debug + Sized {
    /// The type of messages this model processes.
    type Message: Debug + Send + 'static;

    /// Updates the model based on a message and optionally returns a side-effect Command.
    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>>;

    /// Generates a "View" representation for the terminal.
    fn view(&self) -> String;

    /// Generates a semantic tree for AI accessibility.
    fn semantic_view(&self) -> SemanticNode;
}

/// A dispatcher that bridges the Renderer and the State Engine.
pub struct Program<M: Model> {
    pub model: M,
    msg_tx: tokio::sync::mpsc::UnboundedSender<M::Message>,
    msg_rx: tokio::sync::mpsc::UnboundedReceiver<M::Message>,
}

impl<M: Model> Program<M> {
    pub fn new(initial_model: M) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self { 
            model: initial_model,
            msg_tx: tx,
            msg_rx: rx,
        }
    }

    /// Dispatch a message directly.
    pub fn dispatch(&self, msg: M::Message) {
        let _ = self.msg_tx.send(msg);
    }

    /// Process pending messages and run commands.
    pub async fn update(&mut self) -> bool {
        let mut changed = false;
        while let Ok(msg) = self.msg_rx.try_recv() {
            if let Some(cmd) = self.model.update(msg) {
                let tx = self.msg_tx.clone();
                tokio::spawn(async move {
                    if let Some(next_msg) = cmd.await {
                        let _ = tx.send(next_msg);
                    }
                });
            }
            changed = true;
        }
        changed
    }

    pub fn model(&self) -> &M {
        &self.model
    }

    /// For AI usage: get the current UI as a semantic JSON tree.
    pub fn oracle_json(&self) -> String {
        let node = self.model.semantic_view();
        serde_json::to_string_pretty(&node).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct Counter {
        count: i32,
    }

    #[derive(Debug)]
    enum CounterMsg {
        Increment,
    }

    impl Model for Counter {
        type Message = CounterMsg;

        fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
            match msg {
                CounterMsg::Increment => self.count += 1,
            }
            None
        }

        fn view(&self) -> String {
            format!("Count: {}", self.count)
        }

        fn semantic_view(&self) -> SemanticNode {
            SemanticNode::new("counter").with_content(&self.view())
        }
    }

    #[tokio::test]
    async fn test_counter_program() {
        let mut prog = Program::new(Counter::default());
        prog.dispatch(CounterMsg::Increment);
        prog.update().await;
        assert_eq!(prog.model().count, 1);
        let json = prog.oracle_json();
        assert!(json.contains("Count: 1"));
    }
}
