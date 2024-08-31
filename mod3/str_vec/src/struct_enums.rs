use std::env;

#[derive(Debug)]
struct FileSize {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl FileSize {
    fn bytes(size: u64) -> Self {
        FileSize {
            bytes: size,
            kilobytes: size as f64 / 1024.0,
            megabytes: size as f64 / (1024.0 * 1024.0),
            gigabytes: size as f64 / (1024.0 * 1024.0 * 1024.0),
        }
    }
}

fn format_size(size: u64) -> String {
    let filesize = FileSize::bytes(size);
    format!(
        "{} bytes, {} KB, {} MB, {} GB",
        filesize.bytes, filesize.kilobytes, filesize.megabytes, filesize.gigabytes
    )
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file size");
        return;
    }
    let file_size: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please provide a valid file size");
            return;
        }
    };
    let file_format_size = format_size(file_size);
    println!("File size is {}", file_format_size);
}
