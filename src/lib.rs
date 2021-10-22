#![warn(missing_docs)]

//! Encode and decode Octo cartridges or "Octocarts", CHIP-8 game cartridges for the
//! [Octo](https://github.com/JohnEarnest/Octo) environment.
//!
//! Use cases:
//!
//! * Decoding: You can extract the program source code and runtime settings from an
//!   Octocart file. The source code can be assembled into CHIP-8 bytecode with Octo or
//!   [`decasm`]. The runtime settings can be given to a CHIP-8 interpreter like Octo or
//!   [`deca`], or saved as JSON for the [CHIP-8
//!   Archive](https://github.com/JohnEarnest/chip8Archive), as an `.octo.rc` file for C-Octo or
//!   [`termin-8`], etc.
//! * Encoding: TODO
//!
//! Octo cartridge files are GIF89a images with a payload steganographically
//! embedded in one or more animation frames. Data is stored in the least significant
//! bits of colors, 1 from the red/blue channels and 2 from the green channel,
//! allowing us to pack a hidden byte into every 2 successive pixels.
//!
//! The payload consists of a 32-bit length, followed by a sequence of ASCII bytes
//! consisting of the JSON-encoded options dictionary and source text.
//!
//! An Octo cartridge contains the source code of an Octo program, and a set of
//! options for the Octo runtime on how to run the program.
//!
//! * To compile/assemble the source code, check out the [`decasm`] crate.
//! * To interpret an assembled program, check out the [`deca`] crate (backend) or
//!   a program like [`termin-8`] (frontend and graphics).

use octopt::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::u8;
use thiserror::Error;

/// Representation of the payload in the Octo cartridge.
#[derive(Serialize, Deserialize)]
pub struct OctoCart {
    /// The source code of the `.8o` file used to generated the Octocart, as a string of ASCII characters
    program: String,
    /// Representation of the Octo runtime settings required to run this program correctly
    options: OctoOptions,
}

/// Represents the types of errors that can occur during decoding of an Octocart.
#[derive(Error, Debug)]
pub enum Error {
    /// IO error while reading Octocart file
    #[error("Failed to open file")]
    IoError(#[from] std::io::Error),
    /// Decoding error while reading decoding payload from Octocart
    #[error("Failed to decode file")]
    DecodingError(#[from] gif::DecodingError),
    /// Decoding error while deserializing data from payload
    #[error("Failed to parse payload")]
    ParsingError(#[from] serde_json::Error),
}

impl FromStr for OctoCart {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octo_cart = serde_json::from_str(s)?;
        Ok(octo_cart)
    }
}

impl fmt::Display for OctoCart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(string) => write!(f, "{}", string),
            _ => Err(fmt::Error),
        }
    }
}

/// Read and decode Octocart from a file path
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<OctoCart, Error> {
    let file = File::open(path)?;
    let string = decode_octocart(file)?;
    let cart = OctoCart::from_str(&string)?;
    Ok(cart)
}

/// Decodes an Octocart, and returns the decoded JSON payload as a string.
///
/// Example
/// ```no_run
/// let file = std::fs::File::open("test_octocart.gif").unwrap();
/// let payload: String = decart::decode_octocart(file).unwrap();
/// ```
/// You can deserialize this string as an [`OctoCart`]:
/// ```no_run
/// # let payload = "{\"tickrate\":7,\"maxSize\":3215,\"screenRotation\":0,\"fontStyle\":\"octo\",\"touchInputMode\":\"none\",\"fillColor\"#FFCC00\",\"fillColor2\":\"#FF6600\",\"blendColor\":\"#662200\",\"backgroundColor\"\"#996600\",\"buzzColor\":\"#FFAA00\",\"quietColor\":\"#000000\",\"shiftQuirks\":0,\"loadStoreQuirks\":0,\"jumpQuirks\":0,\"logicQuirks\":true,\"clipQuirks\":true,\"vBlankQuirks\":true}";
/// # use std::str::FromStr;
/// use decart::OctoCart;
/// let cart: OctoCart = OctoCart::from_str(payload).unwrap();
/// ```
/// Note that you can also deserialize from a file directly with [`from_file`]:
/// ```no_run
/// use decart::*;
/// let cart: OctoCart = from_file("test_octocart.gif").unwrap();
/// ```
pub fn decode_octocart<R: Read>(input: R) -> Result<String, gif::DecodingError> {
    let mut decoder = gif::DecodeOptions::new().read_info(input)?;
    let palette = &decoder.global_palette().unwrap().to_vec();
    let mut size: u32 = 0;
    let mut first_frame = true;
    let mut json_string = String::new();

    'frame_loop: while let Ok(Some(frame)) = decoder.read_next_frame() {
        let palette = frame.palette.as_ref().unwrap_or(palette);
        if first_frame {
            size = ((byte(&frame.buffer, palette, 0) as u32) << 24)
                | ((byte(&frame.buffer, palette, 2) as u32) << 16)
                | ((byte(&frame.buffer, palette, 4) as u32) << 8)
                | byte(&frame.buffer, palette, 6) as u32;
            json_string = String::with_capacity(size as usize);
        }
        for pixel in (0..frame.buffer.len()).step_by(2) {
            if size == 0 {
                break 'frame_loop;
            }
            if first_frame {
                if pixel < 8 {
                    continue;
                } else {
                    first_frame = false;
                }
            }
            json_string.push(byte(&frame.buffer, palette, pixel) as char);
            size -= 1;
        }
    }
    Ok(json_string)
}

fn nybble((r, g, b): (u8, u8, u8)) -> u8 {
    ((r << 3) & 8) | ((g << 1) & 6) | b & 1
}

fn byte(buffer: &[u8], palette: &[u8], i: usize) -> u8 {
    (nybble(pixel_to_color(buffer[i], palette)) << 4)
        | nybble(pixel_to_color(buffer[i + 1], palette))
}

fn pixel_to_color(pixel: u8, palette: &[u8]) -> (u8, u8, u8) {
    (
        palette[(pixel * 3) as usize],
        palette[(pixel * 3) as usize + 1],
        palette[(pixel * 3) as usize + 2],
    )
}
