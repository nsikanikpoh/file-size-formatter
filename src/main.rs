
use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String
}

impl Sizes {
    fn new(size: u64) -> Self {
        let _size_f64 = size as f64;
        let bytes = format!("{} bytes", size);
        let kilobytes = format!("{:.2} kilobytes", size / 1024);
        let megabytes = format!("{:.2} megabytes", size / 1_048_576);
        let gigabytes = format!("{:.2} gigabytes", size / 1_073_741_824);
        let terabytes = format!("{:.2} gigabytes", size / 1_099_511_627_776);

        Self {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
            terabytes,
        }
    }
}


fn  main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let user_input = args[1].as_str();
        let size_data: Vec<&str> = user_input.split(" ").collect();
        if size_data.len() != 2 {
            panic!("Usage: file_size_formatter <size unit>");
        }
        let size: u64 = size_data[0].parse().expect("Invalid size value");
        let unit = size_data[1].to_lowercase();

        let bytes: u64 = match unit.as_str() {
            "bytes" => 1,
            "kb" | "kilobytes" => 1024,
            "mb" | "megabytes" => 1_048_576,
            "gb" | "gigabytes" => 1_073_741_824,
            "tb" | "terabytes" => 1_099_511_627_776,
            _ => 0, 
        };


        if bytes == 0 {
            panic!("Unknown size unit: {}", unit);
        }
        let bytes = size * bytes; 
        let size_obj = Sizes::new(bytes);
        println!("{:?}", size_obj);
    } else {
        panic!("No size data provided!");
    }
}