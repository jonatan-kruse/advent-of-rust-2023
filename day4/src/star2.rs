pub fn star2() {
    println!(
        "star1: {}",
        (include_str!("i2").lines().collect::<Vec<&str>>().len())
    )
}
