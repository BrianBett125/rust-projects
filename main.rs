use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------- STRUCTS ----------------------

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    timestamp: u64,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

// ---------------------- IMPLEMENTATION ----------------------

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let timestamp = current_time();
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            timestamp,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✅ Task added successfully.");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks available.");
            return;
        }

        println!("📋 Task List:");
        for task in &self.tasks {
            println!(
                "[{}] {} - {} (Created: {})",
                task.id,
                task.title,
                if task.completed { "✔ Done" } else { "❌ Pending" },
                task.timestamp
            );
        }
    }

    fn complete_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task {} marked as complete!", id);
                return;
            }
        }
        println!("⚠️ Task not found.");
    }

    fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < original_len {
            println!("🗑️ Task deleted.");
        } else {
            println!("⚠️ Task not found.");
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        for task in &self.tasks {
            let line = format!(
                "{},{},{},{}\n",
                task.id, task.title, task.completed, task.timestamp
            );
            file.write_all(line.as_bytes()).unwrap();
        }

        println!("💾 Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("⚠️ No previous file found.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let task = Task {
                    id: parts[0].parse().unwrap_or(0),
                    title: parts[1].to_string(),
                    completed: parts[2].parse().unwrap_or(false),
                    timestamp: parts[3].parse().unwrap_or(0),
                };
                self.tasks.push(task);
            }
        }

        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        println!("📂 Tasks loaded.");
    }

    fn analytics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("\n📊 Analytics:");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);

        // Word frequency analysis
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for task in &self.tasks {
            for word in task.title.split_whitespace() {
                let word = word.to_lowercase();
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        println!("\n🔤 Most common words:");
        for (word, count) in word_count.iter().take(5) {
            println!("{} → {}", word, count);
        }
    }
}

// ---------------------- UTILITIES ----------------------

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let log_entry = format!("[{}] {}\n", current_time(), action);
    file.write_all(log_entry.as_bytes()).unwrap();
}

// ---------------------- MAIN LOOP ----------------------

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.db";

    manager.load_from_file(filename);

    loop {
        println!("\n==== TASK MANAGER ====");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Analytics");
        println!("6. Save & Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
                log_action("Added task");
            }
            "2" => {
                manager.list_tasks();
                log_action("Listed tasks");
            }
            "3" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.complete_task(id);
                    log_action("Completed task");
                }
            }
            "4" => {
                println!("Enter task ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete_task(id);
                    log_action("Deleted task");
                }
            }
            "5" => {
                manager.analytics();
                log_action("Viewed analytics");
            }
            "6" => {
                manager.save_to_file(filename);
                log_action("Exited program");
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
