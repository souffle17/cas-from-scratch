use std::env;

fn func(x: f64) -> f64 {
    let out: f64 = 1.0 / x;

    out
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() != 7 && input.len() != 5 {
        println!("Enter four or six arguments for the min and max on the x and y axis respectively, then optionally the x scale and y scale");
    } else {
        let x_min_in = str::parse::<f64>(&input[1]);
        let x_max_in = str::parse::<f64>(&input[2]);
        let y_min_in = str::parse::<f64>(&input[3]);
        let y_max_in = str::parse::<f64>(&input[4]);
        let mut x_scale = 1.0;
        let mut y_scale = 1.0;
        if input.len() == 7 {
            let x_scale_in = str::parse::<f64>(&input[5]);
            let y_scale_in = str::parse::<f64>(&input[6]);

            if y_scale_in.is_err() || x_scale_in.is_err() {
                panic!("Not a number")
            }

            x_scale = x_scale_in.unwrap();
            y_scale = y_scale_in.unwrap();
        }

        if x_min_in.is_err() || x_max_in.is_err() || y_min_in.is_err() || y_max_in.is_err() {
            panic!("Not a number")
        }

        let mut graph: Vec<Vec<char>> = Vec::new();

        let x_min = (x_min_in.unwrap() as f64 / x_scale).round() as i64;
        let x_max = (x_max_in.unwrap() as f64 / x_scale).round() as i64;
        let y_min = (y_min_in.unwrap() as f64 / y_scale).round() as i64;
        let y_max = (y_max_in.unwrap() as f64 / y_scale).round() as i64;

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

        for i in 0..(x_max - x_min + 1) {
            let y = (func((x_min + i) as f64 * x_scale) / y_scale).round() as i64;
            if y <= y_max && y >= y_min {
                graph[((y_max - y) as f64).round() as usize][i as usize] = '•';
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
