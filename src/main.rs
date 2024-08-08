use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    loop {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A', 'B' or 'C'");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to get result");

        let answer: String = match answer.trim().parse() {
            Ok(answer) => answer,
            Err(_) => continue,
        };

        if answer == "A" {
            println!("A Given");
        } if answer == "B" {
            println!("B Given");
        } if answer == "C" {
            println!("C Given");
        }
        break
        
        
    }
}
