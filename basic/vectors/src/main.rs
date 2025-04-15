fn main() {
    // Create an empty vector for treasures
    let mut treasures = Vec::new();

    // Add some treasures to vector
    treasures.push(String::from("Gold coins"));
    treasures.push(String::from("Emerald"));

    println!("Crabby's treasures: {:?}", treasures);

    let mut items = vec!["Gold", "Silver", "Ruby Gem"];

    items.remove(1);
    items.push("Diamond");

    println!("Items: {:?}", items);
    println!("Items length: {}", items.len());
    println!("Items capacity: {}", items.capacity());
}
