use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Task {
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut task_queue = BinaryHeap::new();

    // Добавляем задачи с разными приоритетами
    task_queue.push(Task {
        priority: 3,
        description: "Низкий приоритет".to_string(),
    });
    task_queue.push(Task {
        priority: 1,
        description: "Высокий приоритет".to_string(),
    });
    task_queue.push(Task {
        priority: 2,
        description: "Средний приоритет".to_string(),
    });

    // Выполняем задачи в порядке приоритета
    while let Some(task) = task_queue.pop() {
        println!("Выполняем задачу: {:?}", task);
    }
}