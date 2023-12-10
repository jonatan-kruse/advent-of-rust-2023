pub fn star1(input: &str) -> usize {
    let mut pos  = Pos {x: 0, y: 0};
    let pipes = input.lines().enumerate().map(|(y, l)| l.chars().enumerate().map(|(x, c)| match c {
        '|' => Pipe::UpDown,
        '-' => Pipe::LeftRight,
        'J' => Pipe::UpLeft,
        'L' => Pipe::UpRight,
        '7' => Pipe::DownLeft,
        'F' => Pipe::DownRight,
        'S' => {
            pos = Pos {x, y};
            Pipe::StartPipe
        }
        _ => Pipe::NoPipe,
    }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let first_pos = pos.clone();
    let mut loop_length = 1;
    // hard coded start value from looking at S
    let mut dir = (0, -1);
    pos = pos.next_pos(dir);

    while pos != first_pos {
        dir = pipes[pos.y][pos.x].next_dir(dir.0, dir.1);
        pos = pos.next_pos(dir);
        loop_length += 1;
    }
    loop_length / 2

}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl FindPos for Pos {
    fn next_pos(&self, dir: (i32, i32)) -> Pos {
        let x: i32 = self.x.try_into().unwrap();
        let y: i32 = self.y.try_into().unwrap();
        let next_x: i32 = dir.0 + x;
        let next_y: i32 = dir.1 + y;
        Pos { x: next_x.try_into().unwrap(), y: next_y.try_into().unwrap() }
    }
}

trait FindPos {
    fn next_pos(&self, dir: (i32, i32)) -> Pos;
}

#[derive(Debug, Clone)]
enum Pipe {
    UpDown,
    LeftRight,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    StartPipe,
    NoPipe,
}

impl FindDir for Pipe {
    fn next_dir(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Pipe::UpDown => if x == 0 && y != 0 {(x, y)} else {panic!()},
            Pipe::LeftRight => if x != 0 && y == 0 {(x, y)} else {panic!()},
            Pipe::UpLeft => (-y,-x),
            Pipe::UpRight => (y, x),
            Pipe::DownLeft => (y, x),
            Pipe::DownRight => (-y,-x),
            Pipe::StartPipe => panic!(),
            Pipe::NoPipe => panic!(),
        }
    }
}

trait FindDir {
    fn next_dir(&self, x: i32, y: i32) -> (i32, i32);
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 8)
    }

}
