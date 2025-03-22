use std::collections::VecDeque;
use std::sync:: {Arc, Mutex};
use std::thread;
use std::time::Duration;
fn queue() {
    // cоздаем очередь, защищенную мьютексом
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    // флаг для указания завершения работы производителя
    let producer_finished = Arc::new(Mutex::new(false));

    // клонируем очередь и флаг для использования в потоках
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);
    let producer_finished_consumer = Arc::clone(&producer_finished);

    // поток-производитель
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            let mut queue = queue_producer.lock().unwrap();
            queue.push_back(i); // добавляем задачу в очередь
            println!("Производитель добавил задачу: {}", i);
            drop(queue); // освобождаем мьютекс
            thread::sleep(Duration::from_secs(1)); // задержка
        }
        // устанавливаем флаг завершения работы производителя
        let mut finished = producer_finished.lock().unwrap();
        *finished = true;
        println!("Производитель завершил работу.");
    });

    // поток-потребитель
    let consumer = thread::spawn(move || {
        loop {
            let task = {
                let mut queue = queue_consumer.lock().unwrap();
                queue.pop_front() // извлекаем задасу из очереди
            }; // Мьютекс автоматически освобождается здесь

            if let Some(task) = task {
                println!("Потребитель обработал задачу: {}", task);
                thread::sleep(Duration::from_secs(1));
            } else {
                // обязательно проверяем, завершил ли производитель работу
                let finished = producer_finished_consumer.lock().unwrap();
                if *finished {
                    // Если производитель завершил работу и очередь пуста, завершаем потребитель
                    println!("Очередь пуста. Потребитель завершает работу.");
                    break;
                }
                // Если очередь пуста, но производитель еще работает, ждем
                println!("Очередь пуста, ждем новые задачи...");
                thread::sleep(Duration::from_millis(500)); // Небольшая задержка
            }
        }
    });

    // Ждем завершения потоков
    producer.join().unwrap();
    consumer.join().unwrap();

    println!("Очередь закрыта.");

}

fn main() {
    queue();
}