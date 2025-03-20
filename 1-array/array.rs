use std::collections::HashMap;
// use memoize::memoize;

fn array() {

    // В rust array можно представить кортежем (неоднороднеы данные)/массивом (однородные данные): фиксированная длина
    let my_tuple = (1, 3, 5, "10", "23", ["1", "2", "3"]);
    println!("{:?}", my_tuple);

    let my_array: [i32; 5] =  [10, 23, 5, 52, 45];
    let sum_array: i32= my_array.iter().sum();
    println!("Сумма всех элементов массива: {}", sum_array);


    let array_days: [&str; 7] = ["Понедельник", "Вторник", "Среда", "Четверг", "Пятница", "Суббота", "Воскресенье"];
    println!("Список дней недели: {:?}", array_days);


    // tuple
    let color = (255, 0, 134);
    let (r, g, b) = color;
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);

    match max_min(&my_array) {
        Some((min, max)) => println!("Минимальное и максимальное число в массиве: {}, {}", min, max),
        None => println!("Массив пуст."),
    }

    // кэширование
    let mut cache: HashMap<i32, [i32; 3]> = HashMap::new();

    let mut color1: [i32; 3] = [201, 12, 254];
    let mut color2: [i32; 3] = [252, 17, 223];
    let mut color3: [i32; 3] = [159, 251, 203];

    change_color(&mut color1);
    change_color(&mut color2);
    change_color(&mut color3);

    cache.insert(1, color1);
    cache.insert(2, color2);
    cache.insert(3, color3);
    println!("Кэшированные цвета: {:?}", cache);

    // встроенная мемоизация
    let new_days_array = change_str_array(&array_days);
    println!("Измененный массив дней недели: {:?}", new_days_array); // результат взят из кэша

}

// #[memoize]
fn change_str_array<const N: usize>(array: &[&str; N]) -> [String; N] {
    let mut result: [String; N] = std::array::from_fn(|_| String::new());
    for (i, val) in array.iter().enumerate() {
        result[i] = format!("{}-{}", i+1, val.to_uppercase());
    }
    result
}


fn change_color(array: &mut [i32; 3]) {
    for i in 0..array.len() {
        array[i] = (array[i] + 10) % 256; // циклическое переполнение
    }
}

fn max_min(array: &[i32]) -> Option<(i32, i32)> {

    if array.is_empty() {
        return None;
    }
    // работаем со ссылкой
    let min = *array.iter().min().unwrap();
    let max = *array.iter().max().unwrap();
    return Some((min, max));
}

fn main() {
    array();
}