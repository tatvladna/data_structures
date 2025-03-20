use std::collections::VecDeque;

fn list() {
    let mut tasks1 = Vec::new();
    tasks1.push("Сделать домашку");
    tasks1.push("Сходить на пары");
    tasks1.push("Купить молокоо");
    println!("Список задач: ");

    // реалиация стэка
    while let Some(t) = tasks1.pop() {
        println!("Выполняем задачу: {}", t);
    }

    // реализация очереди
    let mut tasks2 = VecDeque::new();
    tasks2.push_back("Погулять");
    tasks2.push_back("Поиграть");
    tasks2.push_back("Поговорить с мамой");

    while let Some(t) = tasks2.pop_front() {
        println!("Выполняем задачу: {}", t);
    }

    // реализация истории операции через стэк
    let mut history = Vec::new();
    history.push("Добавлена задача: Сделать домашку");
    history.push("Добавлена задача: Сходить на пары");
    history.push("Добавлена задача: Купить молоко");

    println!("История операции: ");
    for (i, h) in history.iter().enumerate() {
        println!("{}-{}",i+1, h);
    }

    // удалим последнюю операцию
    history.pop();
    println!("История операции после удаления последнего пункта: ");
    for (i, h) in history.iter().enumerate() {
        println!("{}-{}", i+1, h);
    }

}

fn main() {
    list();
}