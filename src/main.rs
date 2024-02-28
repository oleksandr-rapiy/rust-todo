use rust_todo::{ToDo, ToDoList};

fn main() {
    let mut todo_list = ToDoList::new();

    let todo_1 = ToDo::create(String::from("Task 1"));

    todo_list.add(todo_1);
    todo_list.display();

    let todo_2 = ToDo::create(String::from("Task 2"));
    let todo_3 = ToDo::create(String::from("Task 3"));
    let todo_4 = ToDo::create(String::from("Task 4"));

    todo_list.add(todo_2);
    todo_list.add(todo_3);
    todo_list.add(todo_4);
    
    todo_list.display();

    todo_list.remove(1);

    todo_list.display();

    todo_list.clean();

    todo_list.display();
}