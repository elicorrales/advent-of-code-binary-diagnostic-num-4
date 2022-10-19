#![allow(clippy::all,clippy::pedantic)]

use std::env::args;
use std::fs::{File};
use std::process::exit;
use std::io::{BufReader, BufRead};

fn main() {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
        println!("\nNeed path to inputs file.\n");
        exit(1);
    }

    let path = &args[1];

    //formatting strings
    //One way:
    //let err_msg = format!("\nFile {} was not found.\n", path);
    //Another way (if possible):
    let err_msg = format!("\nFile {path} was not found.\n");

    let file_result = File::open(path);
    let file = match file_result {
        Ok(f) => { f },
        _ => {
            println!("{err_msg}");
            exit(1);
        }
    };

    //READ FILE METHOD 1 (all at once):
    /*
    let contents_as_string = fs::read_to_string(path).unwrap();
    println!("{contents_as_string}");
    */

    //READ FILE METHOD 2 (all at once):
    /*
    let contents_as_vector = fs::read(path).unwrap();
    println!("{:?}", contents_as_vector);
    */

    //READ FILE METHOD 3 (all at once):
    /*
    let mut contents_as_buffer = vec![0; 15000];
    let count = file.read(&mut contents_as_buffer).unwrap();
    println!("{:?}", contents_as_buffer);
    println!("\n{count}\n");
    */

    //if we want to programmatically create a counter array of same size as incoming
    //lines of binary string,
    //the problem/limitation with this one is we have to define the size of it at this time
    //before we have not yet read a single line of the file
    let mut bit_pos_counts_arr = vec![0];
    let mut bits_chars_arr_len = 0;

    //READ FILE METHOD 4 (line-by-line):
    //this is an outer loop that iterates through each/all lines
    let reader = BufReader::new(file);
    for (line_index, line) in reader.lines().enumerate() {
        let binary_string = line.unwrap();
        println!("{line_index}, {binary_string}");

        let bits_chars:Vec<char> = binary_string.chars().collect();
        println!("{:?}", bits_chars);

        if bits_chars_arr_len == 0 {
            bits_chars_arr_len = bits_chars.len();
            bit_pos_counts_arr = vec![0;bits_chars_arr_len];
       
        } else if bits_chars_arr_len != bits_chars.len() {
            let err_msg = format!("\nLength of line {line_index} is different than previous.\n");
            println!("{err_msg}");
            exit(1);
        }

        //FOR LOOP - VERSION 1
        /* 
        for bit_idx in 0..bits_chars.len() {
            if bits_chars[bit_idx] == '1' {
                println!("\tPosition {bit_idx} is a 1.");
            }
        }
        println!();
        */ 

        //FOR LOOP - VERSION 2
        //This is an inner loop that iterates through all positions
        //of a single line
        for (char_idx, bit) in bits_chars.iter().enumerate() {

            if *bit != '1' && *bit != '0' {
                let err_msg = format!("\nLine {line_index} contains wrong chars.\n");
                println!("{err_msg}");
                exit(1);
            }

            if *bit == '1' {
                println!("\t\tPosition {char_idx} is a 1.");
                bit_pos_counts_arr[char_idx] += 1;
            }
        }
        println!("{:?}", bit_pos_counts_arr);



    }
}

