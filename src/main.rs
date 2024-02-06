use std::env;

use compute::NumberNode;

use crate::parser::generate_tree;
mod compute;
mod parser;

fn draw_iter(left_side: &NumberNode, right_side: &NumberNode, x: f64, y: f64, x_scale: f64, y_scale: f64, iterations: i64) -> bool {
    let x_diff = x_scale / iterations as f64;
    let y_diff = y_scale / iterations as f64;

    let x = x - (x_scale / 2.0);
    let y = y - (y_scale / 2.0);

    let err = ((x*x_scale).powi(2) + (y*y_scale).powi(2)).sqrt() * x_scale.max(y_scale);

    let mut correct = false;
    for i in 0..iterations {
        if compute::point_check(Some(left_side), Some(right_side), x + (i as f64*x_diff), y + (i as f64*y_diff), err) {
            correct = true;
            break;
        }
    }

    correct
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() != 10 && input.len() != 8 {
        println!("Enter seven or nine arguments for the min and max on the x and y axis respectively, the two halves of the equation, number of sub-iterations, then optionally the x scale and y scale.");
        println!("Defaults for the axis are -10, 10, -10, 10, 1, 1");
        println!("Equations must be in prefix notation, seperated by spaces");
    } else {
        let x_min_in = str::parse::<f64>(&input[1]);
        let x_max_in = str::parse::<f64>(&input[2]);
        let y_min_in = str::parse::<f64>(&input[3]);
        let y_max_in = str::parse::<f64>(&input[4]);
        let left_side_string = &input[5];
        let right_side_string = &input[6];
        let iterations_in = str::parse::<i64>(&input[7]);
        let mut x_scale = 1.0;
        let mut y_scale = 1.0;

        if input.len() == 10 {
            let x_scale_in = str::parse::<f64>(&input[8]);
            let y_scale_in = str::parse::<f64>(&input[9]);

            
            match x_scale_in {
                Ok(_) => x_scale = x_scale_in.unwrap(),
                Err(_) => {
                    println!("invalid x scale, using default"); 
                },
            };
            match y_scale_in {
                Ok(_) => y_scale = y_scale_in.unwrap(),
                Err(_) => {
                    println!("invalid y scale, using default"); 
                },
            };
        }

        let x_min = match x_min_in {
            Ok(_) => (x_min_in.unwrap() / x_scale).round() as i64,
            Err(_) => {
                println!("invalid x min, using default"); 
                -10
            },
        };
        
        let x_max = match x_max_in {
            Ok(_) => (x_max_in.unwrap() / x_scale).round() as i64,
            Err(_) => {
                println!("invalid x max, using default"); 
                10
            },
        };

        let y_min = match y_min_in {
            Ok(_) => (y_min_in.unwrap() / y_scale).round() as i64,
            Err(_) => {
                println!("invalid y min, using default"); 
                -10
            },
        };
        
        let y_max = match y_max_in {
            Ok(_) => (y_max_in.unwrap() / y_scale).round() as i64,
            Err(_) => {
                println!("invalid y max, using default"); 
                10
            },
        };

        let iterations = match iterations_in {
            Ok(_) => iterations_in.unwrap(),
            Err(_) => {
                println!("invalid iterations, using default"); 
                3
            },
        };

        let mut graph: Vec<Vec<char>> = Vec::new();

        //coordinate axis
        for i in (y_min..y_max + 1).rev() {
            let mut row: Vec<char> = Vec::new();
            for j in x_min..x_max + 1 {
                if i != 0 {
                    if j != 0 {
                        row.push(' ');
                    } else {
                        row.push('|');
                    }
                } else if j == 0 {
                    row.push('+');
                } else {
                    row.push('-');
                }
            }
            graph.push(row);
        }

        let left_side = generate_tree(left_side_string);
        let right_side = generate_tree(right_side_string);
        
        if left_side.is_none() || right_side.is_none() {
            if left_side.is_none() {
                eprintln!("first expression couldn't be resolved");
            }
            if right_side.is_none() {
                eprintln!("second expression couldn't be resolved");
            }
        }
        else {
            for i in y_min..(y_max + 1) {
                for j in x_min..(x_max + 1) {
                    if draw_iter(left_side.as_ref().unwrap(), right_side.as_ref().unwrap(), j as f64 * x_scale, i as f64 * y_scale,  x_scale, y_scale, iterations) {
                        graph[(y_max - i) as usize][(j - x_min) as usize] = '•';
                    }
                }

                let mut output: String = String::new();
                for char in &graph[(y_max - i) as usize] {
                    output.push(*char);
                }
        
                println!("{}", output);

            }
        }
    }
}
