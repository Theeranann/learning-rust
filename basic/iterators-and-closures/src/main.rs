fn main() {
    let treasures = vec![100, 200, 300, 400];

    println!("Normal price : {:?}", treasures);
    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();

    println!("Doubled price : {:?}", doubled_treasures);
}
