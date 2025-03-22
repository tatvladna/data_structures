use std::collections::LinkedList;

fn linked_list() {
    let mut tasks1 = LinkedList::new();
    tasks1.push_back("Сделать домашку");
    tasks1.push_back("Сходить на пары");
    tasks1.push_back("Купить молоко");
    println!("Список задач: ");
    

    // реализуем очередь с помощью связанного списка
    while let Some(t) = tasks1.pop_front() {
        println!("Выполняем задачу: {}", t);
    } 

}


fn main() {
    linked_list();
}