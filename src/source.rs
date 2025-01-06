use opencv::core::prelude::*;
use opencv::core::{Mat, Size, Size_, CV_8UC3};

/// A struct representing an embedding source, which includes an image and its associated sizes.
/// The struct provides methods for creating an embedding source from scratch or from an existing image.
pub struct EmbedSource {
    /// The image (matrix) for embedding purposes.
    pub image: Mat,
    /// The size of the embedding block.
    pub size: i32,
    /// The original frame size of the image.
    pub frame_size: Size,
    /// The adjusted size of the image where both dimensions are multiples of `size`.
    pub actual_size: Size,
}

impl EmbedSource {
    /// Creates a new `EmbedSource` with a blank image of the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the embedding block.
    /// * `width` - The width of the original image or frame.
    /// * `height` - The height of the original image or frame.
    ///
    /// # Returns
    ///
    /// A new instance of `EmbedSource` with initialized fields.
    ///
    /// # Panics
    ///
    /// Panics if the creation of a new OpenCV `Mat` fails.
    pub fn new(size: i32, width: i32, height: i32) -> EmbedSource {
        // Original size of the frame
        let frame_size: Size_<i32> = Size::new(width, height);

        // Adjust the width to be a multiple of the embedding size
        let actual_width: i32 = width - (width % size);

        // Adjust the height to be a multiple of the embedding size
        let actual_height: i32 = height - (height % size);

        // Create the adjusted size using the new width and height
        let actual_size: Size_<i32> = Size::new(actual_width, actual_height);

        unsafe {
            // Create a new blank image (matrix) with the specified dimensions and type
            let image: Mat = Mat::new_rows_cols(frame_size.height, frame_size.width, CV_8UC3)
                .expect("Failed to create new Mat");

            EmbedSource {
                image,
                size,
                frame_size,
                actual_size,
            }
        }
    }

    /// Creates a new `EmbedSource` from an existing image.
    ///
    /// # Arguments
    ///
    /// * `image` - An OpenCV `Mat` containing the source image.
    /// * `size` - The size of the embedding block.
    /// * `instuction` - A boolean that determines whether to allow images where the height
    ///   is not divisible by the embedding size.
    ///
    /// # Returns
    ///
    /// * `Ok(EmbedSource)` if the image is successfully processed.
    /// * `Err(String)` if the image's dimensions are incompatible and `instuction` is `false`.
    pub fn from(image: Mat, size: i32, instuction: bool) -> Result<EmbedSource, String> {
        // Extract the dimensions of the provided image
        let width: i32 = image.cols();
        let height: i32 = image.rows();

        // Create the original size of the image (frame size)
        let frame_size: Size_<i32> = Size::new(width, height);

        // If the height is not a multiple of the embedding size and `instuction` is false,
        // return an error message
        if height % size != 0 && !instuction {
            return Err("Image size is not a multiple of the embedding size".to_string());
        }

        // Adjust the width to be a multiple of the embedding size
        let adjusted_width: i32 = width - (width % size);

        // Adjust the height to be a multiple of the embedding size
        let adjusted_height: i32 = height - (height % size);

        // Create the adjusted size using the new width and height
        let actual_size: Size_<i32> = Size::new(adjusted_width, adjusted_height);

        // Return the new `EmbedSource` instance
        Ok(EmbedSource {
            image,
            size,
            frame_size,
            actual_size,
        })
    }
}
