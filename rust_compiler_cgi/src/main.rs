/**
 * This implementation was written to be used for web programming, so , and _ are not included
 * Error reports are bloat
 */
use std::env;
use std::path::Path;
use std::fs;

fn main() 
{
    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists()
    {
        println!("File {} not found!", filename);
        return;
    }
    let code = fs::read_to_string(filename)
        .expect("Error corrupted while reading file");
    let tokens = code.split("");
    let mut result = String::from("#include<stdio.h>\nint main() { char stack[30000] = {0}; int pointer = 0;");
    for token in tokens
    {
        match token
        {
            "+" => result.push_str("stack[pointer]++;"),
            "-" => result.push_str("stack[pointer]--;"),
            ">" => result.push_str("pointer++;"),
            "<" => result.push_str("pointer--;"),
            "." => result.push_str("putchar(stack[pointer]);"),
            "[" => result.push_str("while (stack[pointer]) {"),
            "]" => result.push_str("}"),
            _ => result.push_str("")
        }
    }
    result.push_str("}");
    println!("{}", result); // wip
}
