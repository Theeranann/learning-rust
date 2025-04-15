fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure!".to_string())
    }
}

fn open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        Err("You found a monster!".to_string())
    } else {
        Ok("The door is safe".to_string())
    }
}

fn main() {
    let chest_result = match open_chest(false) {
        Some(treasure) => treasure,
        None => "The chest is empty".to_string(),
    };

    let door_result = match open_door(false) {
        Ok(safe) => safe,
        Err(monster) => panic!("{}", monster),
    };

    println!("{}", chest_result);
    println!("{}", door_result);
}
