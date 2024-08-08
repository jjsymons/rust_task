use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![];

    loop {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A', 'B' or 'C'");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to get result");

        let mut answer: String = match answer.trim().parse() {
            Ok(answer) => answer,
            Err(_) => continue,
        };

        answer.make_ascii_uppercase();

        if answer == "A" {
            println!("A Given");
            // Add task
        } if answer == "B" {
            println!("B Given");
            // Delete a task
        } if answer == "C" {
            // Display tasks
            println!("C Given");
        } else {
            println!("A input of something other than A, B, or C given:\n'{answer}'");
            continue
        }
        break
    }
}

fn task_delete() {
    // Will add a task in the form of a string to the vector
    "Nothing"
}
fn task_delete(){
    // Will remove a task from the task list
}
fn task_display(){
    // Will Show the task list
}

