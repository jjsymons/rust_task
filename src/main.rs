use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![]; // Should probably change this to create a small csv

    let mut continuation_bool = true;

    while continuation_bool {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A', 'B' or 'C'");
        
        let mut answer = get_input();
        answer.make_ascii_uppercase();

        if answer == "A" {
            println!("A Given");
            task_create(&mut task_vector);
            continuation_bool = continuation_response_question();
            continue
        } if answer == "B" {
            task_delete(&mut task_vector);
            continuation_bool = continuation_response_question();
            continue
        } if answer == "C" {
            // Display tasks
            println!("C Given");
            task_read(&task_vector);
            continuation_bool = continuation_response_question();
            continue
        } else {
            println!("A input of something other than A, B, or C given:\n'{answer}'");
            continue
        }
    }
}

fn task_create(task_vector: &mut Vec<String>) -> &Vec<String> {
    println!("Please give a task you wish to add to your tasks?");
    let response = get_input();
    let task_list: &mut Vec<String> = task_vector;

    task_list.push(response);
    task_list
}

fn task_delete(task_list: &mut Vec<String>) {
    loop {
        let vector_size: i32  = (task_list.len() - 1).try_into().unwrap();

        println!("Please give the number of the task you wish to remove from the list");
        println!("Please give an answer between 1 and {vector_size}");

        let mut answer = get_input().parse::<i32>().unwrap();
        answer -= 1;

        if answer < 0 {
            println!("Error your input was 0");
        } else if answer > vector_size {
            println!("Error you entered a value greater than {vector_size}");
        } else if answer == vector_size {
            task_list.pop();
        } else {
            task_list.remove(answer.try_into().unwrap());
        }
        return
    }
}

fn task_read(task_list: &Vec<String>) {
    // Will Show the task list
    if task_list.len() == 0 {
        println!("Task list is currently empty. Returning");
        return
    }
    let mut index = 1;
    for task in task_list.iter() {
        println!("{index}) {task}");
        index += 1;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get result");

    let input: String = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {println!("ERROR"); return Default::default()},
    };
    return input
}

fn continuation_response_question() -> bool {
    loop {
        println!("Would you like to continue?:\n'Y'/'y' for yes or 'N'/'n' for no.");
        let mut str = get_input();
        str.make_ascii_uppercase();

        if str == "N" {
            return false
        } else if str == "Y" {
            return true
        } else {
            println!("An input other than 'Y'/'y' or 'N'/'n'\nInput was: {str}");
            continue
        }
    }
}

