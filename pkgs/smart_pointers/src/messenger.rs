pub trait Messenger {
    /// The trait method takes `&self`, so implementors only receive
    /// an immutable reference. If an implementation needs to mutate
    /// its internal state, it must use interior mutability
    /// (e.g., RefCell).
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Error: exceeded quota!");
        } else if percentage >= 0.75 {
            self.messenger
                .send("Warning: high level of usage at {percentage*100}%!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    /// MockMessenger needs to record sent messages.
    /// However, `Messenger::send` only takes `&self`,
    /// so we cannot mutate `sent_msgs` directly.
    /// RefCell is used to enable interior mutability.
    struct MockMessenger {
        sent_msgs: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_msgs: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            println!("The message is {msg}");
            self.sent_msgs.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn send_one_msg_when_over_75() {
        let messenger = MockMessenger::new();
        let mut limit_trscker = LimitTracker::new(&messenger, 100);
        limit_trscker.set_value(80);
        assert_eq!(messenger.sent_msgs.borrow().len(), 1);
    }
}
