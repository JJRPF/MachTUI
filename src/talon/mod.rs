//! The "Talon" State Engine
//!
//! Strict Model-View-Update (MVU) architecture.

use std::fmt::Debug;

/// Represents the global state of the application.
pub trait Model: Debug + Sized {
    /// The type of messages this model processes.
    type Message: Debug;

    /// Updates the model based on a message.
    fn update(&mut self, msg: Self::Message);

    /// Generates a "View" representation (to be refined as we build Mach components).
    fn view(&self) -> String; 
}

/// A dispatcher that bridges the Renderer and the State Engine.
pub struct Program<M: Model> {
    model: M,
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
    }

    #[test]
    fn test_counter_program() {
        let mut prog = Program::new(Counter::default());
        prog.dispatch(CounterMsg::Increment);
        prog.dispatch(CounterMsg::Increment);
        prog.dispatch(CounterMsg::Decrement);
        
        assert_eq!(prog.model().count, 1);
        assert_eq!(prog.model().view(), "Count: 1");
    }
}
