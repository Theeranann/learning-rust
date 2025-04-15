fn main() {
    println!("Hello, James!");
    let x = 5;
    let y = 10.0;

    let sum = x + y as i32;
    println!("sum : {}", sum);

    // str data type
    let msg = String::from("Hello String String::from");
    let msg2 = "Hello String .to_string()".to_string();
    let msg3 = "Hello &str";
    let msg4 = format!("format! String : {}, {}", x, y);

    println!("{},{},{},{}", msg, msg2, msg3, msg4);

    let crabby_energy = 100;

    if crabby_energy > 75 {
        println!("Full energy and ready to go");
    } else if crabby_energy > 50 {
        println!("a bit tried but can keep going")
    } else {
        println!("Need to rest..")
    }

    let enemy = "slime";

    match enemy {
        "goblin" => println!("attacks with claws!"),
        "slime" => println!("uses a water blast!"),
        "dragon" => println!("runs for cover!"),
        _ => println!("doesn't know what to do.."),
    }

    let mut apples = 0;

    loop {
        apples += 1;
        println!("collects an apples total : {}", apples);

        if apples == 5 {
            break;
        }
    }

    let weather = "rainy";

    match weather {
        "sunny" => println!("cross the river by swimming!"),
        "rainy" => println!("build a bridge to stay dry"),
        "stormy" => println!("wait for better weather"),
        _ => println!("doesn't know what to do.."),
    }

    let enemy = "troll";

    match enemy {
        "goblin" => println!("uses rusty sword to attack"),
        "troll" => println!("sets a trap!"),
        "dragon" => println!("run for cover!"),
        _ => println!("confused know what to do.."),
    }

    let mut wood = 0;
    loop {
        wood += 1;
        println!("collects a wood total : {}", wood);

        if wood == 10 {
            println!("finished the boat!");
            break;
        }
    }

    let result = crabby_tasks("gathering coins", 12);
    println!("{}", result);

    let task = crabby_tasks("task", 60);
    println!("{}", task);

    fn crabby_tasks(task: &str, time: i32) -> String {
        format!("Crabby has successfully done {} in {} minutes!", task, time)
    }
}
