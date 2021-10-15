/**
 * Attempt to create a programming language
 * 
 * Error reports are waste of time.
 * You need only 8 characters to write solid software.
 * I think that only way to write comments is anywhere in your code.
 * Next project is going to be self-compiler. 
 * Yes i have no life how did you noticed?
 * I started with this thing because i got B from ICT lesson.
 * But man i've created interpreted language in rust.
 *
 * "I had to do it in C" counter: 452425547582
 * *duckduckgoing* "how to global variables in rust" counter: 45742542578
 * *wrong shortcut in vim* counter: 5865847
 * 
 * -TENMAJKL
 */
use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct StackContext 
{
    stack: Vec<u8>,
    current_element: i32,
    is_parsing_loop: bool,
    loop_statement: String
}

fn parse(token:&str, mut current_element:i32, stack: &mut Vec<u8>, mut is_parsing_loop: bool, mut loop_statement: String) -> StackContext
{
    if token == "]"
    {
        if is_parsing_loop
        {
            is_parsing_loop = false;
            let mut loop_tokens;
            let ls = &loop_statement;
            let mut l_stack_ctx:StackContext;
            while stack[current_element as usize] != 0
            {
                loop_tokens = loop_statement.split("");
                for loop_token in loop_tokens
                {
                    l_stack_ctx = parse(loop_token, current_element, stack, is_parsing_loop, ls.to_string());
                    *stack = l_stack_ctx.stack;                    
                    current_element = l_stack_ctx.current_element;
                }
            }
        }
    }
    if is_parsing_loop
    {
        loop_statement.push_str(token);
        let stack = stack.to_vec();
        let ctx = StackContext { stack, current_element, is_parsing_loop, loop_statement}; 
        return ctx;
    }
    if token == ">"
    {
        current_element+= 1;
        if stack.get(current_element as usize) == None
        {
            stack.push(0);
        }
        //println!("{}", current_element);
    }
    if token == "<"
    {
        if current_element == 0
        {
            println!("Can't move cursor backwards, because its on first element!");
            process::exit(0x0100);
        }
        current_element-= 1;
    }
    if token == "+"
    {
        stack[current_element as usize]+= 1;
    }
    if token == "-"
    {
        stack[current_element as usize]-= 1;
    }
    if token == "."
    {
        println!("{}", stack[current_element as usize] as char);  
    }
    if token == "_"
    {
        println!("{}", stack[current_element as usize]);
    }
    if token == "["
    {
        is_parsing_loop = true;
    }
    let stack = stack.to_vec();
    let ctx = StackContext { stack, current_element, is_parsing_loop, loop_statement}; 
    return ctx;

    
}


fn main() {
    let mut stack: Vec<u8> = Vec::new();
    stack.push(0);
    let mut current_element = 0;
    let mut args = env::args();
    let mut is_parsing_loop = false;
    let mut loop_statement = String::from(""); 
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists()
    {
        println!("File {} not found!", filename);
        return;
    }
    let code = fs::read_to_string(filename)
        .expect("Error corrupted while reading file");
    let tokens = code.split("");
    let mut stack_cxt: StackContext;
    for token in tokens
    {
        stack_cxt = parse(token, current_element, &mut stack, is_parsing_loop, loop_statement);
        stack = stack_cxt.stack;
        current_element = stack_cxt.current_element;
        is_parsing_loop = stack_cxt.is_parsing_loop;
        loop_statement = stack_cxt.loop_statement;
    }
}
