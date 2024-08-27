use std::io;

fn main() {
    println!("Hello, welcome to your task reminder!");

    let mut task_vector: Vec<String> = vec![]; // Should probably change this to create a small csv

    loop {
        println!("Would you like to add a task (A), delete a task (B), read your current tasks (C) or (E) to Exit?:\nPlease enter:\n'A', 'B', 'C' or 'E'");
        
        let mut answer = get_input_string();
        answer.make_ascii_uppercase();

        if answer == "A" {
            task_create(&mut task_vector);
            continue
        } if answer == "B" {
            task_delete(&mut task_vector);
            continue
        } if answer == "C" {
            // Display tasks
            task_read(&task_vector);
            continue
        } if answer == "E" {
            // Exits
            break
        } else {
            println!("A input of something other than A, B, C or E given:\n'{answer}'");
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
        let vector_size = task_list.len();
        if vector_size < 1 {
            println!("Task list is empty, you will need to add tasks first, returning.");
            return 
        }
        println!("Please give the number of the task you wish to remove from the list between 1 and {vector_size}.");
        let input = get_input_integer();

        if input < 0 {
            println!("Value is less than 0. Please enter a value greater than 0.");
            continue
        }

        let mut answer: usize = input.try_into().unwrap();
        answer -= 1;

        if answer > (vector_size - 1) {
            println!("Error you entered a value greater than {vector_size}.");
        } else if answer == (vector_size - 1)  {
            task_list.pop();
        } else {
            task_list.remove(answer);
        }
        return
    }
}

fn task_read(_task_list: &Vec<String>) {
    // Will show the task list to the user 
    if _task_list.len() == 0 {
        println!("Task list is empty, you will need to add tasks first, returning.");
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
fn get_input_integer() -> isize {
    // Function to get input from user when needed
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get result");

        let _input: isize = match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => {println!("ERROR, Please give a new numberic input: "); continue},
            };
    } 
}

