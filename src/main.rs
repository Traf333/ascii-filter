use opencv::{core, highgui, imgproc, prelude::*, videoio};

fn main() -> Result<(), opencv::Error> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open the camera!");
    }

    let window = "Camera Feed";
    // Define ASCII characters from lightest to darkest
    let ascii_chars = " .:-=+*#%@";
    let ascii_len = ascii_chars.len() as f64;

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.empty() {
            continue;
        }

        // Apply a filter: convert to grayscale
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

        // Resize the frame for easier display in the terminal
        let aspect_correction = 1.8; // Adjust based on terminal font
        let rows = 40;
        let cols = (80.0 * aspect_correction) as i32;
        println!("\x1B[2J"); // Clear the terminal
        let mut small_frame = Mat::default();
        imgproc::resize(
            &gray_frame,
            &mut small_frame,
            core::Size {
                width: cols,
                height: rows,
            },
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;

        // Convert each pixel to an ASCII character based on brightness
        let mut ascii_art = String::new();
        for y in 0..rows {
            for x in 0..cols {
                let pixel_value = *small_frame.at_2d::<u8>(y, x)?;
                let char_idx = ((pixel_value as f64 / 255.0) * (ascii_len - 1.0)).round() as usize;
                let ascii_char = ascii_chars.chars().nth(char_idx).unwrap();
                ascii_art.push(ascii_char);
            }
            ascii_art.push('\n');
        }

        // Move cursor to top-left and print the frame
        print!("\x1B[H{}", ascii_art);

        // Small delay for frame rate
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Show the frame
        // highgui::imshow(window, &gray_frame)?;

        // Exit the loop if 'q' is pressed
        if highgui::wait_key(10)? == 'q' as i32 {
            break;
        }
    }

    Ok(())
}
