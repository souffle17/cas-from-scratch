use std::io::{self, Write};

use simple_cas::{compute::NumberNode, parser::generate_tree, draw, simplify};

fn memory_edit(slot_1: &mut (String, Option<NumberNode>), slot_2: &mut (String, Option<NumberNode>)) {

    let mut input: String = "".to_string();
                        
    print!("Enter expression: ");
    let _ = io::stdout().flush();

    input.clear();
    let _ = io::stdin().read_line(&mut input).is_ok();
    input = input.trim_end().to_string();

    let temp = input.clone();

    print!("Select memory slot: ");
    let _ = io::stdout().flush();

    input.clear();
    let _ = io::stdin().read_line(&mut input).is_ok();
    input = input.trim_end().to_string();

    match str::parse::<i32>(&input) {
        Ok(j) => {
            match j {
                1 => {
                    slot_1.0 = temp.clone(); 
                    slot_1.1 = generate_tree(&slot_1.0);
                },
                2 => {
                    slot_2.0 = temp.clone(); 
                    slot_2.1 = generate_tree(&slot_2.0);
                }
                _ => println!("Slot not found")
            }
        }
        Err(_) => println!("Slot not found")
    }

}

fn print_memory(slot_1: &(String, Option<NumberNode>), slot_2: &(String, Option<NumberNode>)) {
    let mut input: String = "".to_string();

    print!("Select a memory slot: ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();
    input = input.trim_end().to_string();

    match str::parse::<i32>(&input) {
        Ok(j) => {
            match j {
                1 => {
                    dbg!(&slot_1.1);
                },
                2 => {
                    dbg!(&slot_2.1);
                }
                _ => println!("Slot not found")
            }
        }
        Err(_) => println!("Slot not found")
    }
}

fn compute_number(slot_1: Option<&NumberNode>, slot_2: Option<&NumberNode>) {
    let mut input: String = "".to_string();
    let _ = io::stdout().flush();

    print!("Select an expression: ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();
    input = input.trim_end().to_string();

    let func = match str::parse::<i32>(&input) {
        Ok(j) => {
            match j {
                1 => {
                    slot_1
                },
                2 => {
                    slot_2
                }
                _ => {println!("Slot not found"); None},
            }
        }
        Err(_) => {println!("Slot not found"); None},
    };

    if func.is_none() {
        println!("Invalid expression");
    }
    else {
        let func = func.unwrap();

        print!("Enter x: ");
        let _ = io::stdout().flush();

        let _ = io::stdin().read_line(&mut input).is_ok();
        input = input.trim_end().to_string();

        let x = str::parse::<f64>(&input);
        if x.is_err() {
            println!("Not a number");
            return;
        }
        
        print!("Enter y: ");
        let _ = io::stdout().flush();

        let _ = io::stdin().read_line(&mut input).is_ok();
        input = input.trim_end().to_string();

        let y = str::parse::<f64>(&input);
        if y.is_err() {
            println!("Not a number");
            return;
        }
        let _ = io::stdout().flush();

        println!("{}", func.resolve(x.unwrap(), y.unwrap()));

    }
}
fn simplify_slot(slot_1: Option<&NumberNode>, slot_2: Option<&NumberNode>) {
    let mut input: String = "".to_string();
    let _ = io::stdout().flush();

    print!("Select an expression: ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();
    input = input.trim_end().to_string();

    let func = match str::parse::<i32>(&input) {
        Ok(j) => {
            match j {
                1 => {
                    slot_1
                },
                2 => {
                    slot_2
                }
                _ => {println!("Slot not found"); None},
            }
        }
        Err(_) => {println!("Slot not found"); None},
    };

}
fn main() {
    let mut input: String = "".to_string();
    let mut slot_1: (String, Option<NumberNode>) = ("".to_string(), None);
    let mut slot_2: (String, Option<NumberNode>) = ("".to_string(), None);
    loop {
        input.clear();
        println!("Expressions in memory: ");
        println!("1: {}", &slot_1.0);
        println!("2: {}", &slot_2.0);
        println!();
        println!("0. Quit");
        println!("1. Enter an expression");
        println!("2. Print expression tree");
        println!("3. Compute value from expression");
        println!("4. Graph expressions as equation");
        println!("5. Simplify an expression");
        print!("Pick an operation by number: ");
        let _ = io::stdout().flush();


        let _ = io::stdin().read_line(&mut input).is_ok();

        input = input.trim_end().to_string();

        match str::parse::<i32>(&input) {
            Ok(i) => {
                match i {
                    0 => break,
                    1 => memory_edit(&mut slot_1, &mut slot_2),
                    2 => print_memory(&slot_1, &slot_2),
                    3 => compute_number(slot_1.1.as_ref(), slot_2.1.as_ref()),
                    4 => draw::draw_prompt(slot_1.1.as_ref(), slot_2.1.as_ref()),
                    5 => simplify::simplify(),
                    _ => println!("Invalid operation")
                }
            }
            Err(_) => println!("Invalid operation")
        }
        
        println!("_______________________________________________________________________________________\n");
    }
    
}