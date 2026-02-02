use std::sync::{ Arc, Mutex };

#[derive(Debug, Clone)]
pub struct HandOff<T>(Arc<Mutex<Option<T>>>);

impl<T> HandOff<T> {
    fn new(val: T) -> Self {
        HandOff(Arc::new(Mutex::new(Some(val))))
    }

    fn take(self) -> Option<T> {
        if let Ok(mut locked_value) = self.0.lock() {
            locked_value.take()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread() {
        let handoff = HandOff::new(19);
        let handoff_clone = handoff.clone();
        
        assert_eq!(handoff.take(), Some(19));
        assert_eq!(handoff_clone.take(), None);
    }
}
