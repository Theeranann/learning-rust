use std::collections::HashMap;

fn main() {

    let mut treasures = HashMap::new();

    treasures.insert("Silver Coins", 100);
    treasures.insert("Ruby", 3);

    if let Some(ruby) = treasures.get_mut("Ruby") {
        *ruby += 5;
    }

    println!("Treasures: {:?}", treasures);
}
