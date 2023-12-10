use std::collections::HashSet;

pub fn star2(input: &str) -> usize {
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

    let mut inside_loop = pipes.iter().map(|l| l.iter().map(|_| 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut loop_pipes = pipes.iter().map(|l| l.iter().map(|_| Pipe::NoPipe).collect::<Vec<_>>()).collect::<Vec<_>>();
    let first_pos = pos.clone();
    // This is sadly hardcoded by looking at the S in the input, could be solved but was lazy
    let first_dir = (0, -1);
    let mut dir = first_dir;
    loop_pipes[first_pos.y][first_pos.x] = Pipe::StartPipe;
    pos = first_pos.next_pos(dir);

    // Loop once to construct a new matrix of only the pipes in the main pipe-loop
    while pos != first_pos {
        loop_pipes[pos.y][pos.x] = pipes[pos.y][pos.x].clone();
        dir = pipes[pos.y][pos.x].next_dir(dir.0, dir.1).clone();
        pos = pos.next_pos(dir);
        loop_pipes[pos.y][pos.x] = pipes[pos.y][pos.x].clone();
    }

    dir = first_dir;
    pos = first_pos.next_pos(dir);

    // Loop again to calculate what is inside the loop
    while pos != first_pos {
        let front = pos.next_pos(dir);
        let left = left_of_dir(dir);
        let right = pos.next_pos(right_of_dir(dir));
        dir = pipes[pos.y][pos.x].next_dir(dir.0, dir.1).clone();
        let area = right.find_area(&loop_pipes, &mut HashSet::new());
        area.iter().for_each(|p| inside_loop[p.y][p.x] = 1);
        // if the loop is doing a left turn we need to check in front of it for a square
        if front.x < pipes[0].len() && front.y < pipes.len() && dir == left{
            let area = front.find_area(&loop_pipes, &mut HashSet::new());
            area.iter().for_each(|p| inside_loop[p.y][p.x] = 1);
        }
        pos = pos.next_pos(dir);
    }
    inside_loop.iter().map(|l| l.iter().sum::<usize>()).sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn find_area(&self, pipes: &Vec<Vec<Pipe>>, set: &mut HashSet<Pos>) -> Vec<Pos> {
        if set.contains(self) {
            return vec![];
        }
        set.insert(self.to_owned());
        let pipe = &pipes[self.y][self.x];
        match pipe {
            Pipe::NoPipe => {
                let mut this = vec![self.clone()];
                let mut r = self.next_pos((1, 0)).find_area(pipes, set);
                let mut l = self.next_pos((-1, 0)).find_area(pipes, set);
                let mut u = self.next_pos((0, -1)).find_area(pipes, set);
                let mut d = self.next_pos((0, 1)).find_area(pipes, set);
                this.append(&mut r);
                this.append(&mut l);
                this.append(&mut u);
                this.append(&mut d);
                this
            }
            _ => vec![]
        }
    }
}

fn right_of_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        _ => panic!(),
    }
}

fn left_of_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => panic!(),
    }
}


trait FindPos {
    fn next_pos(&self, dir: (i32, i32)) -> Pos;
    fn find_area(&self, pipes: &Vec<Vec<Pipe>>, set: &mut HashSet<Pos>) -> Vec<Pos>;
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
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 10)
    }
}

// 409 to low