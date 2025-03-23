use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct SocialNetwork {
    users: HashMap<u32, HashSet<u32>>,
}

// реализуем социальную сеть
impl SocialNetwork {
    fn new() -> Self {
        SocialNetwork {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, user_id: u32) {
        self.users.entry(user_id).or_insert(HashSet::new());
    }

    // добавим связь между пользователями
    fn add_friendship(&mut self, user1: u32, user2: u32) {
        self.users.entry(user1).or_insert(HashSet::new()).insert(user2);
        self.users.entry(user2).or_insert(HashSet::new()).insert(user1);
    }

    // получаем друзей пользователя
    fn get_friends(&self, user_id: u32) -> Option<&HashSet<u32>> {
        self.users.get(&user_id)
    }
}


// реализуем рекомендательную систему
#[derive(Debug)]
struct RecommendationSystem {
    graph: HashMap<u32, HashSet<u32>>, // граф в виде списка смежности
}

impl RecommendationSystem {
    fn new() -> Self {
        RecommendationSystem {
            graph: HashMap::new(),
        }
    }

    // добавляем связь между товарами
    fn add_connection(&mut self, item1: u32, item2: u32) {
        self.graph.entry(item1).or_insert(HashSet::new()).insert(item2);
        self.graph.entry(item2).or_insert(HashSet::new()).insert(item1);
    }

    // получаем рекомендации для товара
    fn get_recommendations(&self, item: u32) -> Option<&HashSet<u32>> {
        self.graph.get(&item)
    }
}

fn main() {
    let mut network = SocialNetwork::new();

    network.add_user(1);
    network.add_user(2);
    network.add_user(3);

    network.add_friendship(1, 2);
    network.add_friendship(1, 3);

    println!("Друзья пользователя 1: {:?}", network.get_friends(1));

    // рекомендательные системы
    let mut system = RecommendationSystem::new();

    system.add_connection(1, 2);
    system.add_connection(1, 3);
    system.add_connection(2, 4);

    println!("Рекомендации для товара 1: {:?}", system.get_recommendations(1));
    println!("Рекомендации для товара 2: {:?}", system.get_recommendations(2));

}