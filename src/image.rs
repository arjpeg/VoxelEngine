use image::{io::Reader as ImageReader, GenericImageView};

use gl::types::{GLuint, GLvoid};

#[allow(dead_code)]
pub struct Image {
    /// The OpenGL texture ID
    pub id: GLuint,

    /// The width of the image in pixels
    pub width: u32,
    /// The height of the image in pixels
    pub height: u32,
    /// The number of channels in the image
    pub channels: u8,

    /// The raw pixel data
    pub pixels: Vec<u8>,
}

#[allow(dead_code)]
impl Image {
    pub fn new(filename: &'static str) -> Self {
        let img = ImageReader::open(filename)
            .expect(format!("Failed to open image file '{}'", filename).as_str())
            .decode()
            .unwrap();

        // Get the image dimensions
        let (width, height) = img.dimensions();

        // Get the image channels
        let channels = img.color().channel_count();

        // Get the image bytes
        let image_content = img.into_bytes();

        // Get the image pixels
        let pixels = image_content.as_slice().to_vec();

        assert!(image_content == pixels);

        // Create the OpenGL texture
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);

            // Bind the texture
            gl::BindTexture(gl::TEXTURE_2D, id);

            let format = if channels == 3 {
                gl::RGB
            } else if channels == 4 {
                gl::RGBA
            } else {
                panic!("Unsupported number of channels: {}", channels);
            } as i32;

            // Bind the texture to the OpenGL context
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format,
                width as i32,
                height as i32,
                0,
                format as u32,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const GLvoid,
            );

            // Generate the mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Unbind the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // Return the image
        Self {
            id,
            width,
            height,
            channels,
            pixels,
        }
    }

    /// Binds the image to the OpenGL context
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the image from the OpenGL context
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            // Delete the OpenGL texture
            gl::DeleteTextures(1, &self.id);
        }
    }
}
