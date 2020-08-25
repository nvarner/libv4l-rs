extern crate clap;
extern crate v4l;

use clap::{App, Arg};
use std::io::Write;
use v4l::prelude::*;

fn main() {
    let matches = App::new("v4l device")
        .version("0.2")
        .author("Nathan Varner <nathanmvarner@protonmail.com>")
        .about("Video4Linux device example")
        .arg(
            Arg::with_name("capture_device")
                .short("c")
                .long("capture-device")
                .value_name("INDEX or PATH")
                .help("Capture device node path or index (default: 0)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output-device")
                .short("o")
                .long("output-device")
                .value_name("INDEX or PATH")
                .help("Output device node path or index (default: 1)")
                .takes_value(true),
        )
        .get_matches();

    // Determine which capture device to use
    let mut capture_path: String = matches
        .value_of("capture-device")
        .unwrap_or("/dev/video0")
        .to_string();
    if capture_path.parse::<u64>().is_ok() {
        capture_path = format!("/dev/video{}", capture_path);
    }
    println!("Using capture device: {}", capture_path);
    let capture_dev =
        CaptureDevice::with_path(capture_path).expect("Failed to open capture device");

    // Determine which output device to use
    let mut output_path: String = matches
        .value_of("output-device")
        .unwrap_or("/dev/video1")
        .to_string();
    if output_path.parse::<u64>().is_ok() {
        output_path = format!("/dev/video{}", output_path);
    }
    println!("Using output device: {}", output_path);
    let mut output_dev =
        OutputDevice::with_path(output_path).expect("Failed to open output device");

    // Set the output's format to the same as the capture's
    let format = capture_dev.format().unwrap();
    output_dev
        .set_format(&format)
        .expect("Failed to set format of output device");

    // Setup a buffer stream, grab a frame, and write it to the output
    let mut stream =
        MmapStream::with_buffers(&capture_dev, 1).expect("Failed to create buffer stream");

    loop {
        let buf = stream.next().expect("Failed to capture buffer");
        output_dev
            .write_all(buf.data())
            .expect("Failed to write to output device");
    }
}
