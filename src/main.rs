use std::io;

fn main() {
    println!("This is the Calculator App 1.0 with Rust. Type the equation
              that you seek to resolve below. You can use +,-,*,/,%,^,(,) 
              operators to create your equations. Note that there is no 
              support for other non-numeric characters.");

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
               .expect("Something wrong happened with the input");
    
        //we don't want the useless chars. Keep period for decimals
        let filtered_input = user_input.chars()
                            .filter(|&ch| ch.is_numeric() || ch == '+' || ch == '-' ||
                                                ch == '*' || ch == '/' || ch == '^' ||
                                                ch == '%' || ch == '(' || ch == ')' || ch == '.')
                            .collect::<String>();

        let characters_to_space = ['+', '-', '*', '/', '^', '%', '(', ')'];

        //vec! creates a vector with specific elements inside
        let spaced_input = filtered_input.chars()
                                                .flat_map(|c| {
                                                    if characters_to_space.contains(&c) {
                                                        vec![' ', c, ' ']
                                                    } else {
                                                        vec! [ c ]
                                                    }
                                                }).collect::<String>();

        println!("{spaced_input}");

        let parts_equation: Vec<&str> = spaced_input.split_whitespace().collect();


        let mut subset_equation: f64 = 0.0;
        //space is default
        let mut current_operator: char = ' ';
        for part in parts_equation {
            if let Ok(num_val) = part.parse::<f64>() {
                println!("Number: {}", num_val);
                if subset_equation == 0.0 {
                    subset_equation = num_val;
                } else if current_operator != ' ' {
                    if current_operator == '+' {
                        subset_equation += num_val;
                    } else if current_operator == '-' {
                        subset_equation -= num_val;
                    } else if current_operator == '*' {
                        subset_equation *= num_val;
                    } else if current_operator == '/' {
                        subset_equation /= num_val;
                    } else if current_operator == '^' {
                        subset_equation = subset_equation.powf(num_val as f64);
                    }
                }
                
            } else {
                println!("Operator: {}", part);
                let filter_operator = part.chars().filter(|c| !c.is_whitespace())
                                                .next().unwrap();
                current_operator = filter_operator;
            }
        }

        println!("{subset_equation}");
    }
}