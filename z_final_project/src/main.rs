// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use mirage::{
    img_functions::{blur, brighten, crop, rotate, invert, grayscale, generate, fractal},
    super_challenge::{super_challenge}};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    // do not remove the first argument
    let subcommand = &args[0];
    match subcommand.as_str() {
        // blur
        // eg. cargo run --release blur dyson.png blurred.png 2.0
        "blur" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // 3rd argument is blur value in f32
            let blur_val: f32 = args.remove(0).parse().expect("Failed to parse a number");
            blur(infile, outfile, blur_val);
        }
        // brighten
        // eg. cargo run --release brighten dyson.png brighten.png 20
        "brighten" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let bright_val: i32 = args.remove(0).parse().expect("Failed to parse a number");
            brighten(infile, outfile, bright_val);
        }

        // crop
        // eg. cargo run --release crop dyson.png crop.png 0 0 200 200
        "crop" => {
            if args.len() != 7 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // x and y origin of crop
            let x: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let y: u32 = args.remove(0).parse().expect("Failed to parse a number");
            // width and height of crop from origin
            let width: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let height: u32 = args.remove(0).parse().expect("Failed to parse a number");
            crop(infile, outfile, x, y, width, height);
        }

        // rotate
        // eg. cargo run --release rotate dyson.png rotate.png 270
        "rotate" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let degrees: u32 = args.remove(0).parse().expect("Failed to parse a number");
            rotate(infile, outfile, degrees);
        }

        // invert
        // cargo run --release invert dyson.png invert.png
        "invert" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }

        // grayscale
        // cargo run --release grayscale dyson.png gray.png
        "grayscale" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        // fractal
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // generate
        // eg. cargo run --release generate generate.png 100 111 90
        "generate" => {
            if args.len() != 5 {
                print_usage_and_exit();
            }
            let _process_name = args.remove(0);
            let outfile = args.remove(0);
            let red: u8 = args.remove(0).parse().expect("Failed to parse a number");
            let green: u8 = args.remove(0).parse().expect("Failed to parse a number");
            let blue: u8 = args.remove(0).parse().expect("Failed to parse a number");
            generate(outfile, red, green, blue);
        }

        // For everything else...
        // **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
        //
        // Make all of the subcommands stackable!
        //
        // For example, if you run:
        //
        //   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
        //
        // ...then your program would:
        // - read infile.png
        // - apply a blur of 2.5
        // - invert the colors
        // - rotate the image 180 degrees clockwise
        // - brighten the image by 10
        // - and write the result to outfile.png
        //
        // Good luck!
        _ => {
            if args.len() != 9 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let blur = args.remove(0);
            let blur_val: f32 = args.remove(0).parse().expect("Failed to parse a number");
            let invert = args.remove(0);
            let rotate = args.remove(0);
            let degrees: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let brighten = args.remove(0);
            let bright_val: i32 = args.remove(0).parse().expect("Failed to parse a number");
            super_challenge(
                infile,
                outfile,
                blur, 
                blur_val,
                invert,
                rotate,
                degrees,
                brighten,
                bright_val
            );
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    println!("or refer to the example command line arguments in main.rs");
    std::process::exit(-1);
}
