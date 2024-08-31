#[derive(Debug, PartialEq)]
enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug, PartialEq)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

pub fn run() {
    let disk_type = DiskType::SSD;

    //     if disk_type == DiskType::SSD {
    //         println!("Disk type is SSD");
    //     } else {
    //         println!("Disk type is HDD");
    //     }
    // }
    match disk_type {
        DiskType::SSD => println!("Disk type is SSD"),
        DiskType::HDD => println!("Disk type is HDD"),
    }

    let disk_size = DiskSize::GB(256);
    println!("Disk size is {:?}", disk_size);
}
