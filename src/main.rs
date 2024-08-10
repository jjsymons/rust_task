use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![]; // Should probably change this to create a small csv

    let mut continuation_bool = true;

    while continuation_bool {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A'/'a', 'B'/'b' or 'C'/'c'");
        
        let mut answer = get_input();
        answer.make_ascii_uppercase();

        if answer == "A" {
            task_create(&mut task_vector);
            continuation_bool = continuation_response_question();
            continue
        } if answer == "B" {
            task_delete(&mut task_vector);
            continuation_bool = continuation_response_question();
            continue
        } if answer == "C" {
            // Display tasks
            task_read(&task_vector);
            continuation_bool = continuation_response_question();
            continue
        } else {
            println!("A input of something other than A, B, or C given:\n'{answer}'");
            continue
        }
    }
    println!("Exiting");
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
        let mut answer = get_input().parse::<i32>().unwrap();
        let vector_size: i32  = task_list.len().try_into().unwrap();

        if vector_size.len() < 1 {
            println!("Task list is empty, returning.")
            return 
        }

        println!("Please give the number of the task you wish to remove from the list");
        println!("Please give an answer between 1 and {vector_size}");

        answer -= 1;

        if answer < 0 {
            println!("Error your input was 0");
        } else if answer > (vector_size -1) {
            println!("Error you entered a value greater than {vector_size}");
        } else if answer == (vector_size -1)  {
            task_list.pop();
        } else {
            task_list.remove(answer.try_into().unwrap());
        } // Need else for non-int inputs.
        return
    }
}

fn task_read(task_list: &Vec<String>) {
    // Will show the task list to the user 
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
    // Function to get input from user when needed
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
    // function to see if the program should exit, returning a bool to a while loop in the main
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

