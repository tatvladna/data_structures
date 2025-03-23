use std::collections::HashSet;

fn main() {
    let mut unique_ids = HashSet::new();

    unique_ids.insert(1);
    unique_ids.insert(2);
    unique_ids.insert(3);
    unique_ids.insert(2);

    println!("Уникальные идентификаторы: {:?}", unique_ids);


    // Операции над множествами 
    let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Объединение: {:?}", union); 

    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Пересечение: {:?}", intersection);


    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Разность: {:?}", difference); 
}