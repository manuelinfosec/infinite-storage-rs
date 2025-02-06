use std::{fs, thread, vec};

use anyhow::{anyhow, Error};
use opencv::core::prelude::*;
use opencv::core::{Mat, MatTraitConst};
use opencv::videoio::{VideoCapture, VideoWriter, CAP_ANY};

use crate::settings::{Data, OutputMode, Settings};
use crate::source::EmbedSource;
use crate::timer::Timer;

/// Reads bytes from a file specified by `path`.
///
/// # Arguments
/// * `path` - The file path to read the bytes from.
///
/// # Returns
/// A vector of bytes if successful, or an error if the file is empty or cannot be read.
pub fn rip_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let byte_data: Vec<u8> = fs::read(path)?;

    if byte_data.is_empty() {
        return Err(anyhow!(
            "Empty files cannot be embedded! File names are not retained, so it's pointless anyway"
        ));
    }
    println!("Bytes ripped successfully");
    println!("Byte length: {}", byte_data.len());

    Ok(byte_data)
}

/// Converts a vector of bytes into a vector of binary bits (`true` for 1, `false` for 0).
///
/// # Arguments
/// * `byte_data` - A vector of bytes to convert.
///
/// # Returns
/// A vector of binary bits or an error if conversion fails.
pub fn rip_binary(byte_data: Vec<u8>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();

    for byte in byte_data {
        // Convert the byte to a binary string representation.
        let mut bits: String = format!("{:b}", byte);
        let missing_0 = 8 - bits.len();

        // Pad with leading zeros to ensure 8 bits.
        for _ in 0..missing_0 {
            bits.insert(0, '0');
        }

        // Convert each bit into a boolean and append to the result.
        for bit in bits.chars() {
            binary_data.push(bit == '1');
        }
    }
    println!("Binary ripped successfully!");
    Ok(binary_data)
}

/// Converts a vector of `u32` integers into binary representation as a vector of `bool`.
///
/// # Arguments
/// * `bytes` - A vector of `u32` integers to convert.
///
/// # Returns
/// A vector of binary bits or an error if conversion fails.
pub fn rip_binary_u32(bytes: Vec<u32>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();

    for byte in bytes {
        // Convert the `u32` value to binary.
        let mut bits = format!("{:b}", byte);
        let missing_0 = 32 - bits.len();

        // Pad with leading zeros to ensure 32 bits.
        for _ in 0..missing_0 {
            bits.insert(0, '0');
        }

        // Convert each bit into a boolean and append to the result.
        for bit in bits.chars() {
            binary_data.push(bit == '1');
        }
    }

    Ok(binary_data)
}

/// Translates binary data (as `bool` vector) back into bytes.
///
/// # Arguments
/// * `binary_data` - A vector of binary bits (`bool`).
///
/// # Returns
/// A vector of bytes.
fn translate_u8(binary_data: Vec<bool>) -> anyhow::Result<Vec<u8>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u8> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 8 {
            // Convert 8 bits into a single byte.
            let byte: u8 = buffer.iter().fold(0u8, |v, b| (v << 1) + (*b as u8));
            byte_data.push(byte);
            buffer.clear();
        }
    }

    Ok(byte_data)
}

/// Translates binary data (as `bool` vector) back into `u32` values.
///
/// # Arguments
/// * `binary_data` - A vector of binary bits (`bool`).
///
/// # Returns
/// A vector of `u32` integers.
fn translate_u32(binary_data: Vec<bool>) -> anyhow::Result<Vec<u32>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u32> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 32 {
            // Convert 32 bits into a single `u32` value.
            let u32_byte = buffer.iter().fold(0u32, |v, b| (v << 1) + (*b as u32));
            byte_data.push(u32_byte);
            buffer.clear();
        }
    }

    Ok(byte_data)
}

/// Writes bytes to a file specified by `path`.
///
/// # Arguments
/// * `path` - The file path to write the bytes to.
/// * `data` - The byte data to write.
///
/// # Returns
/// Nothing if successful, or an error if writing fails.
pub fn write_bytes(path: &str, data: Vec<u8>) -> anyhow::Result<()> {
    fs::write(path, data)?;
    println!("File written successfully");
    Ok(())
}

/// Gets the average RGB values of a pixel block in an image.
///
/// # Arguments
/// * `frame` - The source frame containing the image.
/// * `x` - The x-coordinate of the block.
/// * `y` - The y-coordinate of the block.
///
/// # Returns
/// The average RGB values of the block as a vector.
fn get_pixel(frame: &EmbedSource, x: i32, y: i32) -> Option<Vec<u8>> {
    let mut r_list: Vec<u8> = Vec::new();
    let mut g_list: Vec<u8> = Vec::new();
    let mut b_list: Vec<u8> = Vec::new();

    for i in 0..frame.size {
        for j in 0..frame.size {
            let bgr = frame
                .image
                .at_2d::<opencv::core::Vec3b>(y + i, x + j)
                .unwrap();

            r_list.push(bgr[2]);
            g_list.push(bgr[1]);
            b_list.push(bgr[0]);
        }
    }

    let r_average = r_list.iter().map(|&x| x as usize).sum::<usize>() / r_list.len();
    let g_average = g_list.iter().map(|&x| x as usize).sum::<usize>() / g_list.len();
    let b_average = b_list.iter().map(|&x| x as usize).sum::<usize>() / b_list.len();

    Some(vec![r_average as u8, g_average as u8, b_average as u8])
}

/// Etches a pixel block with the specified RGB values into an image.
///
/// # Arguments
/// * `frame` - The source frame containing the image.
/// * `rgb` - The RGB values to etch.
/// * `x` - The x-coordinate of the block.
/// * `y` - The y-coordinate of the block.
///
/// # Returns
/// Nothing if successful, or an error if the operation fails.
fn etch_pixel(frame: &mut EmbedSource, rgb: Vec<u8>, x: i32, y: i32) -> anyhow::Result<()> {
    for i in 0..frame.size {
        for j in 0..frame.size {
            let bgr = frame.image.at_2d_mut::<opencv::core::Vec3b>(y + i, x + j)?;
            bgr[2] = rgb[0];
            bgr[1] = rgb[1];
            bgr[0] = rgb[2];
        }
    }

    Ok(())
}

/// Embeds RGB data (color) into a video frame.
/// Each triplet of values (R, G, B) represents a single pixel.
///
/// # Arguments
/// - `source`: A mutable reference to an `EmbedSource` object, which represents the video frame.
/// - `data`: A vector of `u8` values representing the RGB data to be embedded.
/// - `global_index`: A mutable reference to the current index in the `data` vector.
///
/// # Returns
/// - `Ok(())` if the operation succeeds.
/// - `Err(anyhow::Error)` if the index exceeds the size of the `data` vector.
fn etch_color(
    source: &mut EmbedSource,
    data: &Vec<u8>,
    global_index: &mut usize,
) -> anyhow::Result<()> {
    // Timer object to measure and log the execution time of this function.
    let _timer = Timer::new("Etching frame");

    // Dimensions of the source frame
    let width = source.actual_size.width; // Frame width
    let height = source.actual_size.height; // Frame height
    let size = source.size as usize; // Size of each pixel block to be processed

    // Iterate over each block of pixels in the frame, stepping by the block size.
    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            // Clone the current index to determine which RGB triplet to embed.
            let local_index = global_index.clone();

            // Extract the RGB triplet from the data vector.
            let rgb = vec![
                data[local_index],     // Red channel
                data[local_index + 1], // Green channel
                data[local_index + 2], // Blue channel
            ];

            // Embed the RGB value into the frame at the specified pixel block.
            etch_pixel(source, rgb, x, y).unwrap();

            // Increment the global index to move to the next RGB triplet.
            *global_index += 3;

            // If the index exceeds the length of the data, return an error.
            if *global_index + 2 >= data.len() {
                return Err(Error::msg("Index beyond data"));
            }
        }
    }

    // Return success if all RGB data is embedded without errors.
    return Ok(());
}

/// Embeds binary data (black-and-white) into a video frame. Each boolean value
/// in the `data` vector corresponds to a pixel where `true` represents white
/// (255 brightness) and `false` represents black (0 brightness).
///
/// # Arguments
/// - `source`: A mutable reference to an `EmbedSource` object, which represents
///   the video frame where the data is being embedded.
/// - `data`: A vector of boolean values that represents the binary data to embed
///   (1 = white, 0 = black).
/// - `global_index`: A mutable reference to the current index within the `data`
///   vector that indicates the next bit to be embedded.
///
/// # Returns
/// - `Ok(())` if the data was successfully embedded.
/// - `Err(anyhow::Error)` if an error occurs, such as exceeding the bounds of `data`.
fn etch_bw(
    source: &mut EmbedSource, // Frame source to embed data into
    data: &Vec<bool>,         // Binary data to embed (true = white, false = black)
    global_index: &mut usize, // Current index in the data vector
) -> anyhow::Result<()> {
    // Timer to track and log the execution time of the etching operation
    let _timer = Timer::new("Etching frame");

    // Dimensions of the source frame (width and height)
    let width = source.actual_size.width; // Frame width
    let height = source.actual_size.height; // Frame height

    // The size of each pixel block (used to step through the frame in increments)
    let size = source.size as usize; // Size of pixel blocks for etching

    // Iterate over the frame's pixels in steps, ensuring that we process each pixel block
    for y in (0..height).step_by(size) {
        // Loop through the vertical pixels
        for x in (0..width).step_by(size) {
            // Loop through the horizontal pixels
            // Clone the global index to determine which bit of data to use
            let local_index = global_index.clone();

            // Determine the brightness for the current pixel
            // 255 (white) for `true` (1) and 0 (black) for `false` (0)
            let brightness = if data[local_index] == true {
                255 // White pixel (bit is 1)
            } else {
                0 // Black pixel (bit is 0)
            };

            // Create an RGB value where R, G, and B channels have the same brightness
            let rgb = vec![brightness, brightness, brightness];

            // Call the `etch_pixel` function to embed the RGB value at the specified (x, y) position
            // This will place the current pixel in the frame.
            etch_pixel(source, rgb, x, y).unwrap(); // Using unwrap here assumes success; consider proper error handling

            // Increment the global index to move to the next bit in the data vector
            *global_index += 1;

            // If the global index exceeds the length of the data, return an error
            if *global_index >= data.len() {
                return Err(Error::msg("Index beyond data"));
            }
        }
    }

    // Return Ok if the function successfully completes without errors
    return Ok(());
}

/// Reads black-and-white (binary) data from a source image by sampling pixel values
/// at intervals defined by the specified block size.
///
/// # Arguments
/// * `source` - A reference to an `EmbedSource` containing the image and related metadata.
/// * `current_frame` - The index of the current frame being processed.
/// * `final_frame` - The index of the last frame to process. Used to determine if this is the final frame.
/// * `final_bit` - The number of bits to retain in the final frame. Only used if `current_frame == final_frame`.
///
/// # Returns
/// * `anyhow::Result<Vec<bool>>` - A vector of boolean values representing the binary data
///   extracted from the image. Returns an error if something goes wrong during processing.
fn read_bw(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_bit: i32,
) -> anyhow::Result<Vec<bool>> {
    // Extract the width and height of the source image.
    let width: i32 = source.actual_size.width;
    let height = source.actual_size.height;

    // Block size determines the step size for sampling pixels in both x and y directions.
    let size = source.size as usize;

    // Initialize an empty vector to store the binary data extracted from the image.
    let mut binary_data: Vec<bool> = Vec::new();

    // Iterate over the image's pixels using a step size equal to the block size.
    // This effectively divides the image into a grid and samples one pixel per block.
    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            // Retrieve the RGB value of the pixel at (x, y).
            let rgb = get_pixel(&source, x, y);

            // If the pixel is out of bounds or cannot be retrieved, skip to the next iteration.
            if rgb.is_none() {
                continue;
            } else {
                // Unwrap the RGB value (since it's guaranteed to exist at this point).
                let rgb = rgb.unwrap();

                // Convert the red channel's value to a boolean.
                // If the red channel's value is >= 127, it's considered `true` (white).
                // Otherwise, it's considered `false` (black).
                if rgb[0] >= 127 {
                    binary_data.push(true);
                } else {
                    binary_data.push(false);
                }
            }
        }
    }

    // If this is the final frame, truncate the binary data to the specified length (`final_bit`).
    if current_frame == final_frame {
        // Slice the binary data to retain only the first `final_bit` elements.
        let slice = binary_data[0..final_bit as usize].to_vec();
        return Ok(slice); // Return the truncated binary data.
    }

    // Return the full binary data for non-final frames.
    Ok(binary_data)
}

/// Reads color data (RGB bytes) from a source image by sampling pixel values
/// at intervals defined by the specified block size.
///
/// # Arguments
/// * `source` - A reference to an `EmbedSource` containing the image and related metadata.
/// * `current_frame` - The index of the current frame being processed.
/// * `final_frame` - The index of the last frame to process. Used to determine if this is the final frame.
/// * `final_byte` - The number of bytes to retain in the final frame. Only used if `current_frame == final_frame`.
///
/// # Returns
/// * `anyhow::Result<Vec<u8>>` - A vector of `u8` values representing the RGB data
///   extracted from the image. Returns an error if something goes wrong during processing.
fn read_color(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_byte: i32,
) -> anyhow::Result<Vec<u8>> {
    // Get the width and height of the source image.
    let width = source.actual_size.width;
    let height = source.actual_size.height;

    // Block size determines the step size for sampling pixels in both x and y directions.
    let size = source.size as usize;

    // Initialize an empty vector to store the byte data extracted from the image.
    let mut byte_data: Vec<u8> = Vec::new();

    // Iterate over the image's pixels using a step size equal to the block size.
    // This effectively divides the image into a grid and samples one pixel per block.
    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            // Retrieve the RGB value of the pixel at (x, y).
            let rgb = get_pixel(&source, x, y);

            // If the pixel is out of bounds or cannot be retrieved, skip to the next iteration.
            if rgb.is_none() {
                continue;
            } else {
                // Unwrap the RGB value (since it's guaranteed to exist at this point).
                let rgb = rgb.unwrap();

                // Push the R, G, and B channels of the pixel into the byte data vector.
                byte_data.push(rgb[0]); // Red channel
                byte_data.push(rgb[1]); // Green channel
                byte_data.push(rgb[2]); // Blue channel
            }
        }
    }

    // If this is the final frame, truncate the byte data to the specified length (`final_byte`).
    if current_frame == final_frame {
        // Slice the byte data to retain only the first `final_byte` elements.
        let slice = byte_data[0..final_byte as usize].to_vec();
        return Ok(slice); // Return the truncated byte data.
    }

    // Return the full byte data for non-final frames.
    Ok(byte_data)
}

/// Generates etching instructions for encoding data into an image source.
/// Depending on the output mode (Color or Binary), this function computes the frame
/// and pixel positions where the data embedding ends.
///
/// # Arguments
/// * `settings` - Configuration settings for the etching process.
/// * `data` - The data to be embedded into the image source.
///
/// # Returns
/// * `EmbedSource` containing the embedded instructions as an image.
fn etch_instructions(settings: &Settings, data: &Data) -> anyhow::Result<EmbedSource> {
    // Size of the instruction block in pixels
    let instruction_size = 5;

    // List of 32-bit instructions to store embedding metadata
    let mut u32_instructions: Vec<u32> = Vec::new();

    // Calculate the number of pixels in a single frame
    let frame_size = (settings.height * settings.width) as usize;

    // Determine the final frame and pixel position based on the output mode
    match data.out_mode {
        OutputMode::Color => {
            // Color mode marker: `u32::MAX`
            u32_instructions.push(u32::MAX);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.bytes.len() % frame_data_size;
            let mut final_frame = data.bytes.len() / frame_data_size;

            // Handle edge case: increment frame if data length perfectly matches frame size
            if data.bytes.len() % frame_size != 0 {
                final_frame += 1;
            }

            // Debugging final frame position
            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
        OutputMode::Binary => {
            // Binary mode marker: `u32::MIN`
            u32_instructions.push(u32::MIN);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.binary.len() % frame_data_size;
            let mut final_frame = data.binary.len() / frame_data_size;

            // Handle edge case: increment frame if data length perfectly matches frame size
            if data.binary.len() % frame_size != 0 {
                final_frame += 1;
            }

            // Debugging final frame position
            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
    }

    // Include the pixel block size in instructions
    u32_instructions.push(settings.size as u32);
    // End marker for size readability; this marker might be required for compatibility
    u32_instructions.push(u32::MAX);

    // Convert instructions into binary format for embedding
    let instruction_data = rip_binary_u32(u32_instructions)?;

    // Create a new image source to store the instructions
    let mut source = EmbedSource::new(instruction_size, settings.width, settings.height);
    let mut index = 0;

    // Attempt to etch instructions onto the source; handle potential errors
    match etch_bw(&mut source, &instruction_data, &mut index) {
        Ok(_) => {}
        Err(_) => {
            println!("Instructions written");
        }
    }

    // Optional display code (commented out for typical use cases)
    // Uncomment for debugging visualization
    // highgui::named_window("window", WINDOW_FULLSCREEN)?;
    // highgui::imshow("window", &source.image)?;
    // highgui::wait_key(10000000)?;

    // Save the resulting image for verification
    // imwrite("src/out/test1.png", &source.image, &Vector::new())?;

    Ok(source)
}

/// Reads and parses embedding instructions from the given `EmbedSource`.
///
/// This function extracts metadata necessary for decoding the embedded data,
/// such as output mode, final frame, and byte positions, and constructs appropriate settings.
///
/// # Arguments
/// * `source` - The source from which instructions are read.
/// * `threads` - The number of threads for parallel processing.
///
/// # Returns
/// * A tuple containing:
///   - `OutputMode` indicating the type of output (Color or Binary)
///   - `final_frame` indicating the last frame with embedded data
///   - `final_byte` indicating the last byte position in the final frame
///   - `Settings` instance with configuration for the decoding process
fn read_instructions(
    source: &EmbedSource,
    threads: usize,
) -> anyhow::Result<(OutputMode, i32, i32, Settings)> {
    // Read binary data from the first frame of the source
    // This retrieves the raw binary encoding of the instructions
    let binary_data = read_bw(source, 0, 1, 0)?;
    
    // Convert the binary data into a vector of 32-bit unsigned integers
    let u32_data = translate_u32(binary_data)?;

    // Extract and interpret the output mode from the first instruction value
    let out_mode = match u32_data[0] {
        u32::MAX => OutputMode::Color,  // Color mode marker
        _ => OutputMode::Binary,       // Default to Binary mode
    };

    // Extract the final frame index for the embedded data
    let final_frame = u32_data[1] as i32;
    
    // Extract the byte position within the final frame
    let final_byte = u32_data[2] as i32;
    
    // Extract the pixel block size for encoding
    let size = u32_data[3] as i32;

    // Retrieve source dimensions (height and width)
    let height = source.frame_size.height;
    let width = source.frame_size.width;

    // Create the settings object for decoding, using the extracted size and provided thread count
    let settings = Settings::new(size, threads, 1337, width, height);

    // Return the parsed instructions and settings
    Ok((out_mode, final_frame, final_byte, settings))
}


/// Embeds data into a video file using multi-threaded frame generation.
///
/// # Arguments
/// * `path` - Path to the output video file.
/// * `data` - Data to embed in the video.
/// * `settings` - Configuration for the embedding process.
///
/// # Returns
/// * `anyhow::Result<()>` - Ok on success or an error on failure.
pub fn etch(path: &str, data: Data, settings: Settings) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching video");

    let mut spool = Vec::new();

    match data.out_mode {
        OutputMode::Color => {
            let length = data.bytes.len();

            // Compute sizes for frame data and chunk data for threads
            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize * 3;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            // Divide data into chunks and spawn threads for parallel processing
            for chunk in data.bytes.chunks(chunk_data_size) {
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    // Generate frames and push to the frame list
                    loop {
                        let mut source = EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_color(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_) => {
                                frames.push(source);
                                println!("Embedding thread complete!");
                                break;
                            }
                        }
                    }

                    frames
                });

                spool.push(thread);
            }
        }
        OutputMode::Binary => {
            let length = data.binary.len();

            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            for chunk in data.binary.chunks(chunk_data_size) {
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    loop {
                        let mut source = EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_bw(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_) => {
                                frames.push(source);
                                println!("Embedding thread complete!");
                                break;
                            }
                        }
                    }

                    frames
                });

                spool.push(thread);
            }
        }
    }

    let mut complete_frames = Vec::new();

    // Generate the instructional frame and add it to the frame list
    let instructional_frame = etch_instructions(&settings, &data)?;
    complete_frames.push(instructional_frame);

    // Collect all frames from the threads
    for thread in spool {
        let frame_chunk = thread.join().unwrap();
        complete_frames.extend(frame_chunk);
    }

    // Attempt to use a lossless codec (PNG)
    let fourcc = VideoWriter::fourcc('p', 'n', 'g', ' ')?;

    // Determine frame size based on the first frame in the list
    let frame_size = complete_frames[1].frame_size;
    let video = VideoWriter::new(path, fourcc, settings.fps, frame_size, true);

    // Fallback to an alternative codec if PNG fails
    let mut video = match video {
        Ok(v) => v,
        Err(_) => {
            let fourcc = VideoWriter::fourcc('a', 'v', 'c', '1')?;
            VideoWriter::new(path, fourcc, settings.fps, frame_size, true)
                .expect("Both PNG and AVC1 codecs failed. Please raise an issue on GitHub.")
        }
    };

    // Write all frames to the video
    for frame in complete_frames {
        let image = frame.image;
        video.write(&image)?;
    }
    video.release()?;

    println!("Video embedded successfully at {}", path);

    Ok(())
}

/// Reads embedded data from a video file.
///
/// # Arguments
/// * `path` - Path to the input video file.
/// * `threads` - Number of threads to use for decoding.
///
/// # Returns
/// * `anyhow::Result<Vec<u8>>` - Returns the embedded byte data or an error.
pub fn read(path: &str, threads: usize) -> anyhow::Result<Vec<u8>> {
    let _timer = Timer::new("Dislodging frame");
    const INSTRUCTION_SIZE: usize = 5;

    // Open the video file
    let mut video = VideoCapture::from_file(path, CAP_ANY)
        .expect("Could not open video path");
    let mut frame = Mat::default();

    // Read the first frame for instructions
    video.read(&mut frame)?;
    let instruction_source = EmbedSource::from(
        frame.clone(), 
        INSTRUCTION_SIZE, 
        true
    ).expect("Couldn't create instructions");

    let (out_mode, final_frame, final_byte, settings) =
        read_instructions(&instruction_source, threads)?;

    let mut byte_data = Vec::new();
    let mut current_frame = 1;

    // Loop through video frames and extract embedded data
    while video.read(&mut frame)? && frame.cols() > 0 {
        if current_frame % 20 == 0 {
            println!("On frame: {}", current_frame);
        }

        let source = EmbedSource::from(
            frame.clone(), 
            settings.size, 
            false
        ).expect("Reading frame failed");

        // Read and decode frame data based on the output mode
        let frame_data = match out_mode {
            OutputMode::Color => read_color(&source, current_frame, i32::MAX, final_byte)
                .expect("Failed to read color frame"),
            OutputMode::Binary => {
                let binary_data = read_bw(&source, current_frame, final_frame, final_byte)
                    .expect("Failed to read binary frame");
                translate_u8(binary_data).expect("Failed to translate binary data")
            }
        };

        byte_data.extend(frame_data);
        current_frame += 1;
    }

    println!("Video read successfully");
    Ok(byte_data)
}
