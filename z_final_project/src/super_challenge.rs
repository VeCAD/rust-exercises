pub fn super_challenge(infile: String,
    outfile: String,
    blur: String, 
    blur_val:f32,
    invert: String,
    rotate: String,
    degrees:u32,
    brighten:String,
    bright_val:i32,
) {

    let img = image::open(infile).expect("Failed to open INFILE.");
    let mut img2 = img.clone();
    
    if blur == "blur" {
        img2 = img2.blur(blur_val);
    }
    if invert == "invert"{
        img2.invert();
    }
    if rotate == "rotate"{
        if degrees == 90 {
            img2 = img2.rotate90();
        } else if degrees == 180 {
            img2 = img2.rotate180();
        } else {
            img2 = img2.rotate270();
        }
    }
    if brighten == "brighten"{
        img2 = img2.brighten(bright_val);
    }

    img2.save(outfile).expect("Failed writing OUTFILE.");
}
