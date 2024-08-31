enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_size(size: u64) -> String {
    let file_size = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1000_000_000..=9999_999_999 => FileSize::Megabytes(size / 1000_000),
        _ => FileSize::Gigabytes(size / 1000_000_000),
    };

    match file_size {
        FileSize::Bytes(s) => format!("File size is {} bytes", s),
        FileSize::Kilobytes(s) => format!("File size is {} kilobytes", s),
        FileSize::Megabytes(s) => format!("File size is {} megabytes", s),
        FileSize::Gigabytes(s) => format!("File size is {} gigabytes", s),
    }
}

pub fn run() {
    let file_size = 1000;
    let result = format_size(file_size);
    println!("{}", result);
}
