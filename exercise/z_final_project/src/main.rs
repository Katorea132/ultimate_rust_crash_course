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

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit("");
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0).parse::<f32>().unwrap();
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile, amount);
        },
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0).parse::<i32>().unwrap();
            brighten(infile, outfile, amount);
        },
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x = args.remove(0).parse::<u32>().unwrap();
            let y = args.remove(0).parse::<u32>().unwrap();
            let height = args.remove(0).parse::<u32>().unwrap();
            let width = args.remove(0).parse::<u32>().unwrap();
            crop(infile, outfile, x, y, width, height);
        },
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let rotation = args.remove(0);
            rotate(infile, outfile, &rotation);
        },
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        },
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit(&subcommand);
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        },
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit("");
            }
            let outfile = args.remove(0);
            fractal(outfile);
        },
        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit("");
        }
    }
}

fn print_usage_and_exit(command:  &str) {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    match command {
        "blur" | "brighten" => println!("{command} INFILE OUTFILE AMOUNT"),
        "crop" => println!("{command} INFILE OUTFILE X Y WIDHT HEIGHT"),
        "rotate" => println!("{command} INFILE OUTFILE ROTATION\nROTATION shall be either 90, 180 or 270"),
        "invert" | "grayscale" => println!("{command} INFILE OUTFILE"),
        _ => println!("Remember the command to us pls.")
        
    }
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, amount: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amount: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(amount);
    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop(x, y, width, height);
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.")
    
}

fn rotate(infile: String, outfile: String, rotation: &str) {
    
    let img = image::open(infile).expect("Failed to open INFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let img2 = match rotation {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => {
            print_usage_and_exit("rotate");
            img
        }
    };

    img2.save(outfile).expect("Failed writing OUTFILE.")
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    
}

fn invert(infile: String, outfile: String) {
    
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();
    
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
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
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

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
