/// Represents the output mode of the data.
/// `Binary` for binary output, `Color` for color data.
pub enum OutputMode {
    Binary, // Binary mode for representing data as bits (e.g., `Vec<bool>`).
    Color,  // Color mode for representing data as bytes (e.g., `Vec<u8>`).
}

/// A struct to hold data and its corresponding output mode.
/// - `bytes`: A vector of bytes (`u8`) used to store color or other non-binary data.
/// - `binary`: A vector of booleans (`bool`) used to represent binary data.
/// - `out_mode`: Specifies the mode (`Binary` or `Color`) to indicate the type of data stored.
pub struct Data {
    pub bytes: Vec<u8>,    // Stores data in byte format (e.g., for color or raw data).
    pub binary: Vec<bool>, // Stores data in binary format (a series of true/false values).
    pub out_mode: OutputMode, // Indicates the output mode of the data (`Binary` or `Color`).
}

impl Data {
    /// Creates a new instance of `Data` with an empty state and a specified `OutputMode`.
    ///
    /// # Arguments
    /// - `out_mode`: The desired output mode of the `Data` (`Binary` or `Color`).
    ///
    /// # Returns
    /// A new `Data` instance with:
    /// - An empty `bytes` vector.
    /// - An empty `binary` vector.
    /// - The specified `out_mode`.
    ///
    /// # Example
    /// ```
    /// let data = Data::new_out_mode(OutputMode::Binary);
    /// ```
    pub fn new_out_mode(out_mode: OutputMode) -> Data {
        Data {
            bytes: Vec::new(),  // Initializes an empty vector for `bytes`.
            binary: Vec::new(), // Initializes an empty vector for `binary`.
            out_mode,           // Sets the `out_mode` to the provided value.
        }
    }

    /// Creates a new `Data` instance from a given binary vector.
    ///
    /// # Arguments
    /// - `binary`: A vector of booleans representing binary data.
    ///
    /// # Returns
    /// A new `Data` instance with:
    /// - An empty `bytes` vector.
    /// - The provided `binary` vector.
    /// - The `OutputMode` set to `Binary`.
    ///
    /// # Example
    /// ```
    /// let binary_data = vec![true, false, true];
    /// let data = Data::from_binary(binary_data);
    /// ```
    pub fn from_binary(binary: Vec<bool>) -> Data {
        Data {
            bytes: Vec::new(),            // Initializes an empty vector for `bytes`.
            binary,                       // Sets `binary` to the provided binary vector.
            out_mode: OutputMode::Binary, // Sets the output mode to `Binary`.
        }
    }

    /// Creates a new `Data` instance from a given byte vector.
    ///
    /// # Arguments
    /// - `bytes`: A vector of bytes representing color or non-binary data.
    ///
    /// # Returns
    /// A new `Data` instance with:
    /// - The provided `bytes` vector.
    /// - An empty `binary` vector.
    /// - The `OutputMode` set to `Color`.
    ///
    /// # Example
    /// ```
    /// let color_data = vec![255, 128, 64];
    /// let data = Data::from_color(color_data);
    /// ```
    pub fn from_color(bytes: Vec<u8>) -> Data {
        Data {
            bytes,                       // Sets `bytes` to the provided byte vector.
            binary: Vec::new(),          // Initializes an empty vector for `binary`.
            out_mode: OutputMode::Color, // Sets the output mode to `Color`.
        }
    }
}

/// Represents the configuration settings.
/// This struct is designed to encapsulate various parameters such as size, threading,
/// frames per second (FPS), and dimensions (width and height) for a customizable setup.
pub struct Settings {
    /// Size of the block or data unit used in the operation.
    /// Example use case: In video encoding, this might represent the block size in pixels.
    pub size: i32,

    /// Number of threads to be used for parallel processing.
    /// A higher thread count can improve performance on multi-core systems.
    pub thread: usize,

    /// Frames per second (FPS) setting for output, affecting video playback smoothness.
    /// Higher FPS values result in smoother playback but may increase processing load.
    pub fps: f64,

    /// Width of the output or input frame, measured in pixels.
    /// Example use case: Setting the resolution width for a video frame.
    pub width: i32,

    /// Height of the output or input frame, measured in pixels.
    /// Example use case: Setting the resolution height for a video frame.
    pub height: i32,
}

impl Settings {
    /// Creates a new instance of the `Settings` struct with the specified parameters.
    ///
    /// # Arguments
    /// - `size` (i32): The block or data unit size.
    /// - `thread` (usize): The number of threads to use for processing.
    /// - `fps` (f64): Frames per second setting.
    /// - `width` (i32): The width of the frame or resolution in pixels.
    /// - `height` (i32): The height of the frame or resolution in pixels.
    ///
    /// # Returns
    /// A new `Settings` instance initialized with the provided values.
    ///
    /// # Example
    /// ```
    /// let settings = Settings::new(16, 4, 30.0, 1920, 1080);
    /// println!(
    ///     "Settings - Size: {}, Threads: {}, FPS: {}, Width: {}, Height: {}",
    ///     settings.size, settings.thread, settings.fps, settings.width, settings.height
    /// );
    /// ```
    ///
    /// # Use Cases
    /// - Configuring video encoding settings with specific resolution and FPS.
    /// - Setting up parameters for multi-threaded data processing.
    pub fn new(size: i32, thread: usize, fps: f64, width: i32, height: i32) -> Self {
        Settings {
            size,   // Block or data unit size.
            thread, // Number of threads for parallel processing.
            fps,    // Frames per second for output.
            width,  // Width of the frame or resolution.
            height, // Height of the frame or resolution.
        }
    }
}
