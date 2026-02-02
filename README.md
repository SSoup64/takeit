# takeit
**A tiny Rust crate for safely transferring ownership exactly once across threads.**

*takeit* provides a `HandOff` primitive: a thread-safe, cloneable container designed for one-time data transfers.

It allows you to share access to a value across multiple threads and ensures that exactly one thread takes ownership of the value, even if the inner type T does not implement Clone.
