use std::time::Instant;

/// A simple struct to measure and log the time elapsed for a given block of code.
/// The timer starts when the struct is created and automatically logs the elapsed time
/// when the struct is dropped.
///
/// # Example
/// ```
/// {
///     let _timer = Timer::new("Example Timer");
///     // Code block whose duration you want to measure
/// }
/// ```
pub struct Timer {
    /// The title or name of the timer, used for identifying the measured block of code.
    title: &'static str,
    /// The start time of the timer, recorded when the Timer instance is created.
    time: Instant,
}

/// Implements the `Drop` trait for `Timer`.
/// The `Drop` trait is automatically called when an instance of `Timer` goes out of scope.
impl Drop for Timer {
    fn drop(&mut self) {
        // Calculate the elapsed time in microseconds and milliseconds.
        let micros: u128 = self.time.elapsed().as_micros();
        let millis: u128 = self.time.elapsed().as_millis();

        // Print the elapsed time with appropriate units.
        // If the elapsed time is less than 10,000 microseconds, print in microseconds (μs).
        // Otherwise, print in milliseconds (ms).
        if micros < 10000 {
            println!("{} ended in {}μs", self.title, micros);
        } else {
            println!("{} ended in {}ms", self.title, millis);
        }
    }
}

impl Timer {
    /// Creates a new `Timer` instance and starts the timer.
    ///
    /// # Arguments
    ///
    /// * `title` - A string slice that holds the name or description of the timer.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Timer` with the current time recorded.
    ///
    /// # Example
    /// ```
    /// let timer = Timer::new("My Timer");
    /// ```
    pub fn new(title: &'static str) -> Timer {
        // Record the current instant when the timer starts.
        let start: Instant = Instant::now();
        Timer { title, time: start }
    }
}
