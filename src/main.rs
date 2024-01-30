use std::env;

fn func(x: i64) -> i64 {
    let out = (10.0 * (x as f64 / 5.0).sin()).round() as i64;

    out
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() != 5 {
        println!("Enter four arguments for the min and max on the x and y axis respectively");
    }
    else {
        let x_min = str::parse::<i64>(&input[1]).unwrap();
        let x_max = str::parse::<i64>(&input[2]).unwrap();
        let y_min = str::parse::<i64>(&input[3]).unwrap();
        let y_max = str::parse::<i64>(&input[4]).unwrap();

        
        let mut graph: Vec<Vec<char>> = Vec::new();

        //coordinate axis
        for i in (y_min..y_max + 1).rev() {
            let mut row: Vec<char> = Vec::new();
            for j in x_min..x_max + 1 {
                if i != 0 {
                    if j != 0 {
                        row.push(' ');
                    }
                    else {
                        row.push('|');
                    }
                }
                else {
                    if j == 0 {
                        row.push('+');
                    }
                    else {
                        row.push('-');
                    }
                }
            }
            graph.push(row);
        }


        for i in 0..(x_max-x_min) {
            let y = func(x_min + i);
            if y <= y_max && y >= y_min {
                graph[(y_max - y) as usize][i as usize] = '•';
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
