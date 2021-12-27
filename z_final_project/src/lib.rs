pub fn blur(infile: String, outfile: String, blur_val:f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(blur_val);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, bright_val:i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(bright_val);
    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop(infile: String, outfile: String, x:u32, y:u32, width:u32, height:u32) {
    // img needs to mutable for crop(), otherwise cargo run throws error
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    let mut img2 = img.clone();
    let img2 = img2.crop(x, y, width, height);
    // save
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String, degrees:u32) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let mut img2 = img.clone();
    // There are 3 rotate functions to choose from (all clockwise):
    if degrees == 90 {
        img2 = img2.rotate90();
    } else if degrees == 180 {
        img2 = img2.rotate180();
    } else {
        img2 = img2.rotate270();
    }
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn generate(outfile: String, red:u8, green:u8, blue:u8) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 1024;
    let height = 768;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        // Set the image to some solid color. -- see fractal() for an example
        // Challenge: parse some color data from the command-line, pass it through
        // to this function to use for the solid color.
        *pixel = image::Rgb([red, green, blue]);

        // Use all as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut draw_green = green;
        let mut draw_blue = blue;
        let mut draw_red = red;
        while draw_green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            draw_green += 1;
            draw_blue += 3;
            draw_red += 2;

        }

        *pixel = image::Rgb([draw_red, draw_green, draw_blue]);

    }

    imgbuf.save(outfile).unwrap();

}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z/c + c;
            green += 1;

        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
