pub fn star1() {
    println!(
        "star1: {}",
        (include_str!("i1").lines().collect::<Vec<&str>>().len())
    )
}
