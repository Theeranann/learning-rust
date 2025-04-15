fn main() {
    let treasures = ["Gold", "Silver", "Ruby Gem", "Emerald"];

    let mut energy = 5;
    let target = treasures[2];
    // let target = String::from("Ruby Gem");

    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are out of energy. Game over!");
            break;
        } else if treasure == &target {
            println!("You found the {} ! You win!", target);
            break;
        }
        println!("you found the {} not {} keep searching", treasure, target);
        energy -= 1;
    }
}
