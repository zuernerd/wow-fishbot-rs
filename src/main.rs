use std::{env::current_dir, time::{Duration, Instant}};
use xcap::{image::ImageBuffer, Window};
use opencv::{boxed_ref::BoxedRef, core::{Point, Rect, Mat, CV_8UC4}, highgui, imgproc, imgcodecs, Result};
use opencv::core::min_max_loc;
use opencv::core;
use opencv::prelude::MatTraitConst;
use enigo::*;
use rand::Rng;
use xdotool::{self, option_vec};
use xdotool::OptionVec;

use std::fs;
use std::path::Path;
use std::error::Error;
use std::thread::sleep;

/// Loads fishing bobber template images from the ./template directory
/// These templates are used for computer vision to detect the fishing bobber on screen
fn load_templates() -> Result<Vec<Mat>, Box<dyn Error>> {
    let mut templates: Vec<Mat> = Vec::new();

    let path = Path::new("./template");
    for entry in fs::read_dir(path)?{
            let entry = entry?;
            println!("{:?}", entry.path().display());

            // Load the template image in color
            let template = imgcodecs::imread(&entry.path().display().to_string(), 1 as i32)?;

            // Convert to grayscale for better template matching
            let mut template_gray = Mat::default();
            imgproc::cvt_color(&template, &mut template_gray, imgproc::COLOR_BGR2GRAY, 0)?;

            // Apply Canny edge detection to focus on edges rather than colors
            // This makes template matching more robust to lighting changes
            let mut template_canny = Mat::default();
            imgproc::canny(&template_gray, &mut template_canny, 50 as f64, 100 as f64, 3, false)?;
            templates.push(template_canny);
    }
    Ok(templates)
}

/// Captures a screenshot of the specified window and converts it to OpenCV format
fn capture_window(window: &Window) -> Result<Mat, Box<dyn Error>> {
    // Capture the window content as an image
    let cap = window.capture_image().unwrap();
    let (width, heigth) = cap.dimensions();
    let image_buffer = cap.clone().into_raw();

    // Convert the raw image data to OpenCV Mat format (RGBA format)
    let frame = unsafe{Mat::new_rows_cols_with_data_unsafe(heigth as i32, width as i32, CV_8UC4, image_buffer.as_ptr() as *mut _, 0)?};

    // Convert to grayscale for image processing
    let mut frame_gray = Mat::default();
    imgproc::cvt_color(&frame, &mut frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;

    Ok(frame_gray)
}

/// Finds the fishing bobber in the current frame using template matching
fn find_bobber(frame_gray: &Mat, templates: &Vec<Mat>) -> Result<Point, Box<dyn Error>> {
    // Apply Canny edge detection to the current frame (same as on templates)
    let mut frame_canny = Mat::default();
    imgproc::canny(&frame_gray, &mut frame_canny, 50 as f64, 100 as f64, 3, false)?;
 
    // Perform template matching to find the bobber location
    let mut frame_lure_location = Mat::default();
    imgproc::match_template(&frame_canny, &templates[0], &mut frame_lure_location, imgproc::TM_CCOEFF_NORMED, &Mat::default())?;

    // Find the location with the highest match confidence
    let mut max_val = 0.0;
    let mut lure_location = Point::new(0, 0);
    min_max_loc(&frame_lure_location, None, Some(&mut max_val), None, Some(&mut lure_location), &Mat::default())?;
    println!("Bobber dectetion: max_val = {:?}  lure_location = {:?} ", max_val, lure_location);
    Ok(lure_location)
}

/// Detects if a fish has "splashed" by comparing two consecutive frames
/// A splash is detected as significant movement/change in the bobber area
fn detect_splash(prev_frame: &Mat, current_frame: &Mat, rect: Rect) -> Result<bool, Box<dyn Error>> {
    // Calculate the absolute difference between frames
    let mut diff_frame = Mat::default();
    core::absdiff(prev_frame, current_frame, &mut diff_frame)?;

    // Extract the region of interest (ROI) around the bobber
    let mut roi = Mat::roi(&diff_frame, rect)?;

    //highgui::imshow("ROI", &roi)?;

    // Apply threshold to convert differences to binary (black/white)
    let mut thresh_frame = Mat::default();
    imgproc::threshold(&roi, &mut thresh_frame, 50.0, 255.0, imgproc::THRESH_BINARY)?;

    // Count non-zero pixels (white pixels indicating movement)
    let non_zero_count = core::count_non_zero(&thresh_frame)?;

    // If enough pixels changed, consider it a splash
    let splash_detected = non_zero_count > 250; // Todo: Adjust this threshold based on experimentation

    print!("{:?} ", non_zero_count);

    Ok(splash_detected)
}

/// Continuously monitors for a fish splash within the specified timeout period
fn wait_for_splash(
    window: &Window, 
    lure_location_rect: Rect, 
    timeout: Duration
) -> Result<bool, Box<dyn std::error::Error>> {
    // Capture initial frame for comparison
    let mut prev_frame_gray = capture_window(&window)?;
    let start_time = Instant::now();

    // Keep checking for splashes until timeout
    while Instant::now().duration_since(start_time) < timeout {
        let current_frame_gray = capture_window(&window)?;

        // Check if a splash occurred
        if detect_splash(&prev_frame_gray, &current_frame_gray, lure_location_rect)? {
            println!("Splash detected!");
            return Ok(true);
        }

        // Update previous frame for next comparison
        prev_frame_gray = current_frame_gray;

        // Small delay between checks to avoid excessive CPU usage
        sleep(Duration::from_millis(50));
    }

    Ok(false) // Timeout occurred without detecting a splash
}

/// Moves the mouse cursor to the center of the specified rectangle
fn move_mouse_to_rect_center(enigo: &mut Enigo, rect: Rect) -> Result<(), Box<dyn Error>> {
    let x = rect.x + rect.width / 2;
    let y = rect.y + rect.height / 2 + 69; // +69 offset for window positioning on gnome
    enigo.move_mouse(x, y, Coordinate::Abs)?;

    Ok(())
}

/// Simulates pressing the fishing key (F4) to cast the fishing line
fn cast_fishing(enigo: &mut Enigo) -> Result<(), Box<dyn Error>> {
    enigo.key(Key::F4, Direction::Press).unwrap();
    random_delay(150, 350);
    enigo.key(Key::F4, Direction::Release).unwrap();

    Ok(())
}

/// Introduces a random delay to make the bot behavior less predictable
/// This might helps avoid detection by anti-cheat systems
fn random_delay(min_delay: u64, max_delay: u64) {
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(min_delay..=max_delay);

    sleep(Duration::from_millis(delay_ms))
}

fn main() -> Result<(), Box<dyn Error>>  {
    // Find the World of Warcraft window
    let window = Window::all().expect("Could not iterate windows")
        .into_iter().find(|x| x.title() == "World of Warcraft")
        .expect("Could not find World of Warcraft windows");

    let mut enigo = Enigo::new(&Settings::default()).unwrap(); 
    
    let templates = load_templates()?;

    let timeout = Duration::from_secs(29);

    // Activate window. TODO: automatically find window ID
    xdotool::desktop::activate_window("33554451", OptionVec::new());

    std::thread::sleep(std::time::Duration::from_millis(1000)); 
    println!("Starting!");


    loop {
        println!("Cast fishing...");
        cast_fishing(&mut enigo)?;
  
        random_delay(1800, 2200);
    
        let frame_gray = capture_window(&window)?;
        let lure_location = find_bobber(&frame_gray, &templates)?;
        let lure_location_rect = Rect::new(lure_location.x, lure_location.y, templates[0].cols(), templates[0].rows());
    
        move_mouse_to_rect_center(&mut enigo, lure_location_rect)?;
    
        //imgproc::rectangle(&mut frame_gray, lure_location_rect, core::Scalar::new(255.0, 0.0, 255.0, 0.0), 2, imgproc::LINE_8, 0)?;
    
        random_delay(400, 600); // wait so the splash detector is not disturbed by the moving cursor

        //highgui::imshow("prev_frame_gray", &prev_frame_gray)?;

        let splash_detected = wait_for_splash(&window, lure_location_rect, timeout)?;

        if splash_detected {
            random_delay(50, 189);
            enigo.button(Button::Right, Direction::Press).unwrap();
            random_delay(184, 483);
            enigo.button(Button::Right, Direction::Release).unwrap();
        } else {
            println!("Timeout occured while waiting for splash")
        }
        
        random_delay(684, 4833);
    }

    //highgui::imshow("Captured Image", &frame_gray)?;
    
    //highgui::wait_key(10000)?;
    Ok(())
}