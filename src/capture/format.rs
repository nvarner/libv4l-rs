use std::convert::TryFrom;
use std::{fmt, mem};

use crate::colorspace::Colorspace;
use crate::field_order::FieldOrder;
use crate::fourcc::FourCC;
use crate::v4l_sys::*;
use crate::Quantization;

#[derive(Debug, Copy, Clone)]
/// Streaming format (single-planar)
pub struct Format {
    /// width in pixels
    pub width: u32,
    /// height in pixels
    pub height: u32,
    /// order of fields
    pub field_order: FieldOrder,
    /// pixelformat code
    pub fourcc: FourCC,
    /// bytes per line
    pub stride: u32,
    /// maximum number of bytes required to store an image
    pub size: u32,
    /// colorspace of the pixels
    pub colorspace: Colorspace,
    /// the way colors are mapped
    pub quantization: Quantization,
}

impl Format {
    /// Returns a capture format
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `fourcc` - Four character code (pixelformat)
    ///
    /// # Example
    ///
    /// ```
    /// use v4l::FourCC;
    /// use v4l::capture::Format;
    /// let fmt = Format::new(640, 480, FourCC::new(b"YUYV"));
    /// ```
    pub fn new(width: u32, height: u32, fourcc: FourCC) -> Self {
        Format {
            width,
            height,
            field_order: FieldOrder::Any,
            fourcc,
            stride: 0,
            size: 0,
            colorspace: Colorspace::Default,
            quantization: Quantization::Default,
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "width        : {}", self.width)?;
        writeln!(f, "height       : {}", self.height)?;
        writeln!(f, "field        : {}", self.field_order)?;
        writeln!(f, "fourcc       : {}", self.fourcc)?;
        writeln!(f, "stride       : {}", self.stride)?;
        writeln!(f, "size         : {}", self.size)?;
        writeln!(f, "colorspace   : {}", self.colorspace)?;
        writeln!(f, "quantization : {}", self.quantization)?;
        Ok(())
    }
}

impl From<v4l2_pix_format> for Format {
    fn from(fmt: v4l2_pix_format) -> Self {
        // Assume that the given format is valid
        Format {
            width: fmt.width,
            height: fmt.height,
            field_order: FieldOrder::try_from(fmt.field).expect("Invalid field"),
            fourcc: FourCC::from(fmt.pixelformat),
            stride: fmt.bytesperline,
            size: fmt.sizeimage,
            colorspace: Colorspace::try_from(fmt.colorspace).expect("Invalid colorspace"),
            quantization: Quantization::try_from(fmt.quantization).expect("Invalid quantization"),
        }
    }
}

impl Into<v4l2_pix_format> for Format {
    fn into(self: Format) -> v4l2_pix_format {
        let mut fmt: v4l2_pix_format;
        unsafe {
            fmt = mem::zeroed();
        }

        fmt.width = self.width;
        fmt.height = self.height;
        fmt.field = self.field_order as u32;
        fmt.pixelformat = self.fourcc.into();
        fmt.bytesperline = self.stride;
        fmt.sizeimage = self.size;
        fmt.colorspace = self.colorspace as u32;
        fmt.quantization = self.quantization as u32;
        fmt
    }
}
