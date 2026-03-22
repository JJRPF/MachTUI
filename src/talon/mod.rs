//! The "Talon" State Engine
//!
//! Strict Model-View-Update (MVU) architecture.

use std::fmt::Debug;
use crate::oracle::SemanticNode;

/// Represents the global state of the application.
pub trait Model: Debug + Sized {
    /// The type of messages this model processes.
    type Message: Debug;

    /// Updates the model based on a message.
    fn update(&mut self, msg: Self::Message);

    /// Generates a "View" representation for the terminal.
    fn view(&self) -> String;

    /// Generates a semantic tree for AI accessibility.
    fn semantic_view(&self) -> SemanticNode;
}

/// A dispatcher that bridges the Renderer and the State Engine.
pub struct Program<M: Model> {
    pub model: M,
}

impl<M: Model> Program<M> {
    pub fn new(initial_model: M) -> Self {
        Self { model: initial_model }
    }

    pub fn dispatch(&mut self, msg: M::Message) {
        self.model.update(msg);
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
        Decrement,
    }

    impl Model for Counter {
        type Message = CounterMsg;

        fn update(&mut self, msg: Self::Message) {
            match msg {
                CounterMsg::Increment => self.count += 1,
                CounterMsg::Decrement => self.count -= 1,
            }
        }

        fn view(&self) -> String {
            format!("Count: {}", self.count)
        }

        fn semantic_view(&self) -> SemanticNode {
            SemanticNode::new("counter").with_content(&self.view())
        }
    }

    #[test]
    fn test_counter_program() {
        let mut prog = Program::new(Counter::default());
        prog.dispatch(CounterMsg::Increment);
        assert_eq!(prog.model().count, 1);
        let json = prog.oracle_json();
        assert!(json.contains("Count: 1"));
    }
}
