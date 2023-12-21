use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    EAST, WEST, NORTH, SOUTH
}

#[derive(Debug)]
struct Crucible {
    x: i16,
    y: i16,
    current_direction: Direction,
    losses: Box<Vec<Vec<u8>>>,
    steps: u8,
    current_loss: u32
}

impl Eq for Crucible {}

impl PartialEq<Self> for Crucible {
    fn eq(&self, other: &Self) -> bool {
        self.current_loss == other.current_loss
    }
}

impl PartialOrd<Self> for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.current_loss.partial_cmp(&other.current_loss)
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        self.current_loss.cmp(&other.current_loss)
    }
}

impl Crucible {

    fn move_(&self, dx: i8, dy: i8, new_direction: Direction, new_forward: u8) -> Option<Crucible> {
        let (new_x, new_y) = (self.x + dx as i16, self.y + dy as i16);
        if new_x >= 0 && new_y >= 0 && new_x < self.losses.len() as i16 && new_y < self.losses[0].len() as i16 {
            let d_loss = self.losses[new_x as usize][new_y as usize];
            Some(Crucible{x: new_x,
                y: new_y,
                current_direction: new_direction,
                losses: self.losses.clone(),
                steps: new_forward,
                current_loss: self.current_loss + d_loss as u32})
        } else {
         None
        }
    }

    fn forward(&self) -> Option<Crucible> {
        let (dx, dy) = match self.current_direction {
            Direction::EAST  => (0, 1),
            Direction::WEST  => (0, -1),
            Direction::NORTH  => (-1, 0),
            Direction::SOUTH  => (1, 0)
        };
        self.move_(dx, dy, self.current_direction, self.steps + 1)

    }

    fn go_left(&self) -> Option<Crucible> {
        let (dx, dy, new_direction) = match self.current_direction{
            Direction::EAST  => (-1, 0 , Direction::NORTH),
            Direction::WEST  => (1, 0 , Direction::SOUTH),
            Direction::NORTH  => (0, -1 , Direction::WEST),
            Direction::SOUTH  => (0, 1 , Direction::EAST)
        };
        self.move_(dx, dy, new_direction, 1)
    }

    fn go_right(&self) -> Option<Crucible> {
        let (dx, dy, new_direction) = match self.current_direction{
            Direction::EAST  => (1, 0 , Direction::SOUTH),
            Direction::WEST  => (-1, 0 , Direction::NORTH),
            Direction::NORTH  => (0, 1 , Direction::EAST),
            Direction::SOUTH  => (0, -1 , Direction::WEST)

        };
        self.move_(dx, dy, new_direction, 1)
    }
}

fn main() {
    let unwrapped_file = read_to_string("./src/input.txt").unwrap();
    let losses = unwrapped_file.lines()
        .map(|x| x.chars().map(|c| c as u8  - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let (nr, nc) = (losses.len(), losses[0].len());
    let boxed_losses = Box::new(losses);
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(
        Crucible{x: 0, y: 0, current_direction: Direction::EAST, losses: boxed_losses.clone(), steps: 0, current_loss: 0}
    ));
    pq.push(Reverse(
        Crucible{x: 0, y: 0, current_direction: Direction::SOUTH, losses: boxed_losses.clone(), steps: 0, current_loss: 0}
    ));
    let mut visited = HashSet::<(i16, i16, Direction, u8)>::new();
    let mut res = u32::MAX;
    while !pq.is_empty() {
        if let Some(Reverse(crucible)) = pq.pop() {
            if !visited.contains(&(crucible.x, crucible.y, crucible.current_direction, crucible.steps)) {
                visited.insert((crucible.x, crucible.y, crucible.current_direction, crucible.steps));
                //println!("{}, {}, {}", crucible.x, crucible.y, crucible.current_loss);
                if crucible.x as usize  == nr - 1 && crucible.y as usize  == nc - 1 {
                    res = res.min(crucible.current_loss);
                } else {
                    if let Some(forward_crucible) = crucible.forward() {
                        if forward_crucible.steps < 4 {
                            pq.push(Reverse(forward_crucible));
                        }
                    }
                    if let Some(left_crucible) = crucible.go_left() {
                        pq.push(Reverse(left_crucible))
                    }
                    if let Some(right_crucible) = crucible.go_right() {
                        pq.push(Reverse(right_crucible))
                    }
                }
            }
        } else {
            break;
        }
    }
    println!("Min loss is {}", res);

}