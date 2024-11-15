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
        let (cols, rows) = (80, 40); // Adjust for your terminal size
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
        for y in 0..rows {
            let mut line = String::new();
            for x in 0..cols {
                let pixel_value = *small_frame.at_2d::<u8>(y, x)?;
                let char_idx = ((pixel_value as f64 / 255.0) * (ascii_len - 1.0)).round() as usize;
                line.push(ascii_chars.chars().nth(char_idx).unwrap());
            }
            println!("{}", line); // Print each line for the frame
        }

        // Clear the terminal (ANSI escape code for Unix-based systems)
        print!("\x1B[2J\x1B[1;1H");

        // Add delay (adjust based on your processing speed)
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
