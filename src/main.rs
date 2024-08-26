use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![]; // Should probably change this to create a small csv

    let mut continuation_bool = true;

    while continuation_bool {
        println!("Would you like to add a task (A), delete a task (B) or read your current tasks (C)?:\nPlease enter:\n'A'/'a', 'B'/'b' or 'C'/'c'");
        
        let mut answer = get_input_string();
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
    let response = get_input_string();
    let task_list: &mut Vec<String> = task_vector;

    task_list.push(response);
    task_list
}

fn task_delete(task_list: &mut Vec<String>) {
    loop {
        println!("Please give the number of the task you wish to remove from the list");

        let mut answer = get_input_integer();

        let vector_size = task_list.len();

        if vector_size < 1 {
            println!("Task list is empty, returning.");
            return 
        }

        println!("Please give an answer between 1 and {vector_size}");
        answer -= 1;

        if answer < 0 {
            println!("Error your input was 0");
        } else if answer > (vector_size -1) {
            println!("Error you entered a value greater than {vector_size}");
        } else if answer == (vector_size -1)  {
            task_list.pop();
        } else if answer > 0 {
            task_list.remove(answer);
        } else {
            println!("ERROR: You entered a non-numberical value, please try again.");
            continue
        }
        return
    }
}

fn task_read(_task_list: &Vec<String>) {
    // Will show the task list to the user 
    if _task_list.len() == 0 {
        println!("Task list is currently empty. Returning");
        return
    }
    let mut index = 1;
    for task in _task_list.iter() {
        println!("{index}) {task}");
        index += 1;
    }
}

fn get_input_string() -> String {
    // Function to get input from user when needed
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get result");

    let input: String = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {println!("ERROR"); return Default::default()},
    };
    input
}
fn get_input_integer() -> usize {
    // Function to get input from user when needed
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get result");

    let input: usize = match input.trim().parse() {
        Ok(input) => return input,
        Err(_) => {println!("ERROR, Please give a new input: "); continue},
        };
    } 
}

fn continuation_response_question() -> bool {
    // function to see if the program should exit, returning a bool to a while loop in the main
    loop {
        println!("Would you like to continue?:\n'Y'/'y' for yes or 'N'/'n' for no.");
        let mut str = get_input_string();
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

