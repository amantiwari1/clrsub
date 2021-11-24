use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("clrsub")
        .version("0.0.1_beta")
        .about("your-app-description")
        .author("Marcus Kazmierczak")
        .args(&[Arg::new("filename")
            .about("get the file path.(Note - Support srt only)")
            .short('f')
            .long("filename")
            .takes_value(true)])
        .get_matches();

    if let Some(filename) = matches.value_of("filename") {
        println!("Value for config: {}", filename);

        let newfilename = String::from("clrsub_") + filename;
        let mut file_write = File::create(newfilename).unwrap();

        let file = File::open(filename).expect("failed to open file");

        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let mut is_zero: bool = false;
            let lines = line.unwrap();
            if !lines.is_empty() {
                let ascii = lines.as_bytes()[0];

                if ascii == 48 {
                    is_zero = true
                }

                if !ascii.is_ascii_digit() || is_zero {
                    writeln!(&mut file_write, "{}", lines).unwrap();
                }
            } else {
                writeln!(&mut file_write, "{}", lines).unwrap();
            }
        }
    } else {
        println!("Please Enter file with -f or --filename");
    }
}
