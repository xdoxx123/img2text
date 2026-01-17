use std::{io::Write, path::Path};

use clap::Parser;
use std::fs::File;
use image::{GenericImageView, ImageReader, imageops};

#[derive(Parser,Debug)]
struct Args {
    path:String,
    #[arg(default_value_t=1)]
    x:u32,
    #[arg(default_value_t=1)]
    y:u32,
    #[arg(long)]
    resize:bool
}


fn getchar(pixel:image::Rgba<u8>) -> &'static str {
    let data = pixel.0;
    match data[0] {
        
        10..=50 => {"░"}
        51..=100 => {"▒"}
        101..=150 => {"▓"}
        151..=255 => {"█"}
        _ => {
            " "
        }
    }

}


fn main() -> anyhow::Result<()> {
    if true && false {
        println!("shit")
    }
    let args: Args = Args::parse();
    let path: String = args.path;
    let mut image = ImageReader::open(Path::new(&path))?.decode()?.grayscale();
    println!("{}",args.x);
    let (x,y) = image.dimensions();
    if args.resize {
        println!("resize detected");
        image = image.resize(x*args.x, y*args.y, imageops::FilterType::CatmullRom);
    }else {
        image = image.resize(x/args.x, y/args.y, imageops::FilterType::CatmullRom);
    }
    let mut buf: File = File::create("output.txt")?;
    let (xn,yn) = image.dimensions();
    for rangey in 0..yn {
        for rangex in 0..xn {
            let pixel: image::Rgba<u8> = image.get_pixel(rangex, rangey);
            let shit = getchar(pixel);

                buf.write_all((shit.to_owned()+shit).as_bytes())?;
            
            
        }
        buf.write_all(b"\n")?;
    }




    Ok(())
}
