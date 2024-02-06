use std::env;
mod compute;
mod parser;

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() != 9 && input.len() != 7 {
        println!("Enter six or eight arguments for the min and max on the x and y axis respectively, the two halves of the equation, then optionally the x scale and y scale.");
        println!("Defaults for the axis are -10, 10, -10, 10, 1, 1");
        println!("Equations must be in prefix notation, seperated by spaces");
    } else {
        let x_min_in = str::parse::<f64>(&input[1]);
        let x_max_in = str::parse::<f64>(&input[2]);
        let y_min_in = str::parse::<f64>(&input[3]);
        let y_max_in = str::parse::<f64>(&input[4]);
        let left_side = &input[5];
        let right_side = &input[6];
        let mut x_scale = 1.0;
        let mut y_scale = 1.0;

        if input.len() == 9 {
            let x_scale_in = str::parse::<f64>(&input[7]);
            let y_scale_in = str::parse::<f64>(&input[8]);

            
            match x_scale_in {
                Ok(_f64) => x_scale = x_scale_in.unwrap(),
                Err(_) => {
                    println!("invalid x scale, using default"); 
                },
            };
            match y_scale_in {
                Ok(_f64) => y_scale = y_scale_in.unwrap(),
                Err(_) => {
                    println!("invalid y scale, using default"); 
                },
            };
        }

        let x_min = match x_min_in {
            Ok(_f64) => (x_min_in.unwrap() / x_scale).round() as i64,
            Err(_) => {
                println!("invalid x min, using default"); 
                -10
            },
        };
        
        let x_max = match x_max_in {
            Ok(_f64) => (x_max_in.unwrap() / x_scale).round() as i64,
            Err(_) => {
                println!("invalid x max, using default"); 
                10
            },
        };

        let y_min = match y_min_in {
            Ok(_f64) => (y_min_in.unwrap() / y_scale).round() as i64,
            Err(_) => {
                println!("invalid y min, using default"); 
                -10
            },
        };
        
        let y_max = match y_max_in {
            Ok(_f64) => (y_max_in.unwrap() / y_scale).round() as i64,
            Err(_) => {
                println!("invalid y max, using default"); 
                10
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
        
        for i in y_min..(y_max + 1) {
            for j in x_min..(x_max + 1) {
                if parser::point_check(left_side, right_side, j as f64 * x_scale, i as f64 * y_scale, x_scale.max(y_scale)) {
                    graph[(y_max - i) as usize][(j - x_min) as usize] = '•';
                }
            }
        }
        let mut output: String = String::new();
        for row in graph {
            for char in row {
                output.push(char);
            }
            output.push('\n');
        }

        println!("{}", output);
    }
}
