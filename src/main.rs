#![allow(unused_variables)]
use std::io::{self, BufRead};

fn bin_list(text: &str, _encoding: &str) -> Vec<i32> {
    let bin_nums_8: Vec<i32> = text
        .as_bytes()
        .iter()
        .flat_map(|&x| {
            let bits = format!("{:08b}", x)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            bits.into_iter()
        })
        .collect();

    bin_nums_8
}



fn bin_to_text(binary_list: Vec<i32>, _encoding: &str) -> Result<String, &'static str> {
    if binary_list.len() % 8 != 0 {
        println!("The output bit buffer is not a multiple of 8, so excess bits will be discarded.");
    }
    let byte_list: Vec<u8> = binary_list
        .chunks(8)
        .map(|chunk| {
            let s: String = chunk.iter().map(|&x| x.to_string()).collect();
            u8::from_str_radix(&s, 2).unwrap()
        })
        .collect(); // Use collect() to allocate the iterator

    match String::from_utf8(byte_list) {
        Ok(s) => Ok(s),
        Err(_) => Err("Unable to decode bytes"),
    }
}

fn soh_10fffe(code: &str) -> String {
    let tape_length = 50000;
    let mut output = String::new();
    let mut output_buffer: Vec<i32> = Vec::new();
    let mut bit_tape = vec![0; tape_length];
    let mut pointer = 0;
    let mut current_char = 0;

    let mut code_chars = code.chars();

    while let Some(instruction) = code_chars.next() {
        current_char += 1;

        match instruction {
            '\u{0001}' => {
                println!("Program is requesting user input:");
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input).unwrap();
                let encoding = if bit_tape[pointer] == 0 { "latin-1" } else { "utf-8" };
                let bit_input = bin_list(&input, encoding);
                bit_tape.splice(pointer..pointer + bit_input.len(), bit_input);
            }
            '\u{10FFFE}' => {
                let encoding = if bit_tape[pointer] == 0 { "latin-1" } else { "utf-8" };
                match bin_to_text(output_buffer.clone(), encoding) {
                    Ok(s) => output.push_str(&s),
                    Err(_) => println!("Unable to decode bytes"),
                }
            }
            '1' => output_buffer.push(bit_tape[pointer]),
            '0' => {
                let popped = output_buffer.pop();
                if let Some(p) = popped {
                    bit_tape[pointer] = p;
                }
            }
            '>' => pointer = (pointer + 1) % tape_length,
            '<' => pointer = (pointer + tape_length - 1) % tape_length,
            '-' => bit_tape[pointer] = 1 - bit_tape[pointer],
            '?' => {
                let move_char = bit_tape[pointer] as usize;
                current_char += move_char;
            }
            _ => {}
        }
    }

    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let mut file_path = String::new();

        println!("Enter the file name (including the .soh10fffe extension):");
        std::io::stdin().read_line(&mut file_path).expect("Failed to read line");

        let file_path = file_path.trim();

        let file_content = match std::fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("File not found or cannot be opened.");
                return;
            }
        };
        let result = soh_10fffe(&file_content);
        println!("{}", result);
    } else if args.len() == 2 && args[1] == "all" {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        let mut files: Vec<_> = std::fs::read_dir(current_dir)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .map(|dir_entry| dir_entry.path())
            .filter(|path| {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        return ext.to_lowercase() == "soh10fffe";
                    }
                }
                false
            })
            .collect();

        files.sort();

        for file in files {
            let file_content = std::fs::read_to_string(&file).expect("Failed to read file");
            println!("Running program from file: {}", file.display());
            let result = soh_10fffe(&file_content);
            println!("Result:\n{}", result);
        }
    } else {
        eprintln!("Invalid argument. To run all .soh10fffe files, use `cargo run all`. Otherwise, don't include any arguments.");
    }
}
