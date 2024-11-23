# Webcam ASCII Art Filter

A real-time webcam application that converts video feed into ASCII art using OpenCV and Rust. This project captures video from your webcam and displays it in the terminal as ASCII characters, creating an artistic text-based representation of the video feed.

![Convert to GIF Nov 23 2024](https://github.com/user-attachments/assets/1ee9c04a-081c-4ec4-a5a3-9b44b96577a7)


## Prerequisites

- Rust (latest stable version)
- OpenCV 4.x
- A working webcam
- Cargo (Rust's package manager)

## Dependencies

This project relies on the following Rust crates:

- `opencv` - For video capture and image processing
- Other dependencies as specified in `Cargo.toml`

## Installation

1. Make sure you have Rust and Cargo installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install OpenCV dependencies (on macOS):

```bash
brew install opencv
```

3. Clone the repository:

```bash
git clone git@github.com:Traf333/ascii-filter.git
cd ascii-filter
```

## Usage

Run the application:

```bash
cargo run --release
```

To exit the application, press `q` or `Ctrl+C`.

in case of facing issue `dyld: Library not loaded: @rpath/libclang.dylib` please check [OpenCV crate: Troubleshooting section](https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md)

## How It Works

1. The application captures video frames from your webcam using OpenCV
2. Each frame is converted to grayscale
3. The grayscale image is resized to fit your terminal
4. Pixels are mapped to ASCII characters based on their intensity
5. The resulting ASCII art is displayed in real-time

## Configuration

You can modify the following parameters in the code:

- ASCII character set for different levels of intensity
- Output dimensions
- Frame rate
- Video capture device

## Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements or bug fixes.
