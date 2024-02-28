use rand::Rng;

pub struct ToDo {
    id: i32,
    title: String,
    status: ToDoStatus,
}

#[derive(Debug)]
enum ToDoStatus {
    Done,
    Pending,
    New,
}

pub struct ToDoList {
    todo_list: Vec<ToDo>,
}

impl ToDo {
    pub fn create(title: String) -> ToDo {
        ToDo {
            id: rand::thread_rng().gen_range(1..1000),
            title,
            status: ToDoStatus::New,
        }
    }
}

impl ToDoList {
    pub fn new() -> ToDoList {
        let mut todo_list = ToDoList {
            todo_list: Vec::new(),
        };

        todo_list.add(ToDo {
            id: 1,
            title: String::from("Default task"),
            status: ToDoStatus::New,
        });

        return todo_list;
    }

    pub fn add(&mut self, todo: ToDo) {
        self.todo_list.push(todo);
    }

    pub fn find(&mut self, id: i32) -> &mut ToDo {
        let todo = self
            .todo_list
            .iter_mut()
            .find(|todo| todo.id == id)
            .expect("Not found todo item");

        return todo;
    }

    pub fn remove(&mut self, id: i32) {
        let position = self.todo_list.iter().position(|x| x.id == id).unwrap_or(0);
        self.todo_list.remove(position);
    }

    pub fn complete_task(&mut self, id: i32) {
        let todo = self.find(id);
        todo.status = ToDoStatus::Done;
    }

    pub fn start_task(&mut self, id: i32) {
        let todo = self.find(id);

        todo.status = ToDoStatus::Pending;
    }

    pub fn clean(&mut self) {
        self.todo_list.clear();
    }

    pub fn display(&self) {
        if self.todo_list.is_empty() {
            println!("==================");
            println!("The are no tasks");
            println!("==================\n");
            return;
        }

        println!("==================");
        for item in self.todo_list.iter() {
            println!("{} - {}: {:?}", item.id, item.title, item.status);
        }
        println!("==================\n");
    }
}
