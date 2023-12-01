mod star1;
mod star2;

fn main() {
    let input = include_str!("./i");
    let answer1 = star1::star1(input);
    let answer2 = star2::star2(input);
    dbg!(answer1);
    dbg!(answer2);
}
