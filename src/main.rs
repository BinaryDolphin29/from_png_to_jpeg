extern crate image;
extern crate chrono;

use image::GenericImageView;
use std::{env, fs::File, path::Path, thread::sleep, time::Duration};
use chrono::Local;

fn main() {
    let infile = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("引数にファイルを指定してください。")
    };

    let p = &Path::new(&infile);
    let opened = image::open(p);
    match opened {
        Ok(img) => {
            let now = Local::now().format("%m-%d_%H;%M");
            let out_file_name = format!("{}.jpeg",  now);
            let outfile = &mut File::create(&Path::new(&format!("{}/{}",  p.parent().unwrap().display(), out_file_name))).unwrap();

            img.write_to(outfile, image::ImageFormat::Jpeg).unwrap();

            let (w, h) = img.dimensions();
            println!("解像度: {}x{}", w, h);
            println!("ファイル名: {}", out_file_name);
        }

        Err(err) => {
            println!("エラー: {}", err);
        }
    }

    sleep(Duration::from_secs(8));
}
