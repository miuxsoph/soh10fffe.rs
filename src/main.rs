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
    let code: String;

    println!("input program (with .soh10fffe extension):");

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim(); // Trim any whitespace or newline characters

    // Read the file and check for the .soh10fffe extension
    if let Ok(file_content) = std::fs::read_to_string(file_path) {
        code = file_content;
        println!("{}", soh_10fffe(&code));
    } else {
        println!("Failed to read the file or the file does not exist.");
    }
}


