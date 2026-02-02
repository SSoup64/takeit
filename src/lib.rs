use std::sync::{ Arc, Mutex };
use std::fmt::{ Debug, Formatter, Result as FmtResult };

/// A syncing type for sending a single object.
/// 
/// The `HandOff` is initialized with a value on creation. The handoff can then
/// be cloned and sent between threads.
/// The first thread to take the value, receives it and takes ownership over the
/// value. Taking after the value was first taken is no allowed.
pub struct HandOff<T>(Arc<Mutex<Option<T>>>);

impl<T> HandOff<T> {
    /// Creates a new HandOff object initialized with a value of type `T`
    ///
    /// # Example
    /// ```
    /// use takeit::HandOff;
    ///
    /// let handoff1 = HandOff::new(10);
    /// let handoff2 = HandOff::new(String::from("Hello, World!"));
    /// ```
    pub fn new(val: T) -> Self {
        Self(Arc::new(Mutex::new(Some(val))))
    }
    
    /// Returns the value of the `HandOff` by moving it.
    ///
    /// # Errors
    /// If the value was already "taken" earlier, it returns `None`.
    ///
    /// # Example
    /// ```
    /// use takeit::HandOff;
    ///
    /// let handoff = HandOff::new(1337);
    /// let handoff_clone = handoff.clone();
    ///
    /// assert_eq!(handoff.take(), Some(1337));
    /// assert_eq!(handoff_clone.take(), None);
    /// ```
    pub fn take(self) -> Option<T> {
        if let Ok(mut locked_value) = self.0.lock() {
            locked_value.take()
        } else {
            None
        }
    }
}

impl<T> Clone for HandOff<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Debug> Debug for HandOff<T> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut builder = fmt.debug_struct("HandOff");

        let locked_value = self.0.lock();
        match locked_value {
            Ok(val) => {
                builder.field("value", &val);
            },
            _ => {
                builder.field("value", &None::<T>);
            }
        }

        builder.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(PartialEq, Debug)]
    struct Foo {
        val: i32,
    }

    #[test]
    fn test_single_thread() {
        let handoff = HandOff::new(19);
        let handoff_clone = handoff.clone();
        
        assert_eq!(handoff.take(), Some(19));
        assert_eq!(handoff_clone.take(), None);
    }
    
    #[test]
    fn test_non_clonable() {
        let handoff = HandOff::new(Foo { val: 10 });
        let handoff_clone = handoff.clone();

        assert_eq!(handoff.take(), Some(Foo { val: 10 }))
    }
    
    #[test]
    fn test_threads() {
        let handoff = HandOff::new(Foo { val: 42 });

        let my_thread = std::thread::spawn(move || {
            assert_eq!(handoff.clone().take(), Some(Foo { val: 42 }))
        });
    }
}
