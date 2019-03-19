/*!

Measuring invocation time on the Web can be done with `console.time`
and `console.timeEnd`.

This API wraps both the `time` and `timeEnd` calls into a single type
named `ConsoleTimer`, ensuring both are called.

## Scoped measurement

```no_run
use gloo_console_timer::ConsoleTimer;

let value = ConsoleTimer::scope("foo", || {
    // Place code to be measured here
    // Optionally return a value.
});
```

 */

#![deny(missing_docs, missing_debug_implementations)]

use web_sys::console;

/// A console time measurement.
///
/// See `ConsoleTimer::scope` for starting a labeled time measurement
/// of code wrapped in a closure.
#[derive(Debug)]
pub struct ConsoleTimer<'a> {
    label: &'a str,
}

impl<'a> ConsoleTimer<'a> {
    /// Starts a console time measurement. The measurement
    /// ends when the constructed `ConsoleTimer` object is dropped.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_console_timer::ConsoleTimer;
    ///
    /// let _timer = ConsoleTimer::new("foo");
    /// ```
    pub fn new(label: &'a str) -> ConsoleTimer<'a> {
        console::time_with_label(label);
        ConsoleTimer { label }
    }

    /// Starts a scoped console time measurement
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_console_timer::ConsoleTimer;
    ///
    /// let value = ConsoleTimer::scope("foo", || {
    ///     // Code to measure here
    /// });
    /// ```
    pub fn scope<F, T>(label: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _timer = ConsoleTimer::new(label);
        f()
    }
}

impl<'a> Drop for ConsoleTimer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.label);
    }
}
