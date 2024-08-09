use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![]; // Should probably change this to create a small csv

    loop {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A', 'B' or 'C'");
        let mut answer = get_input();

        answer.make_ascii_uppercase();

        if answer == "A" {
            println!("A Given");
            task_create(&mut task_vector);
        } if answer == "B" {
            task_delete(&task_vector);
            // Delete a task
        } if answer == "C" {
            // Display tasks
            println!("C Given");
            task_read(&task_vector);
        } else {
            println!("A input of something other than A, B, or C given:\n'{answer}'");
            continue
        }
        break
    }
}

fn task_create(task_vector: &mut Vec<String>) -> &Vec<String> {
    println!("Please give a task you wish to add to your tasks?");
    let response = get_input();
    let task_list: &mut Vec<String> = task_vector;

    task_list.push(response);
    task_list
    
}
fn task_delete(task_list: &Vec<String>){
    // Will remove a task from the task list
}
fn task_read(task_list: &Vec<String>){
    // Will Show the task list
}


fn get_input() -> String {
    let mut answer = String::new();
    loop {
        io::stdin() .read_line(&mut answer).expect("Failed to get result");

        let answer: String = match answer.trim().parse() {
            Ok(answer) => answer,
            Err(_) => continue,
        };
    }
    answer
}

