use crate::vec2i::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum Facing {
    N,
    E,
    S,
    W,
    None
}

impl Facing {
    pub fn from_char(c: &char) -> Facing {
        match c {
            'N' => Facing::N,
            'E' => Facing::E,
            'S' => Facing::S,
            'W' => Facing::W,
            _ => Facing::None
        }
    }
}

#[derive(Debug)]
pub enum TurnDirection {
    Left,
    Right,
    None
}

impl TurnDirection {
    pub fn from_char(c: &char) -> TurnDirection{
        match c {
            'L' => TurnDirection::Left,
            'R' => TurnDirection::Right,
            _ => TurnDirection::None
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Translate(Facing, isize),
    Forward(isize),
    Turn(TurnDirection, isize),
    None
}

#[derive(Debug)]
pub struct Waypoint {
    pub rel_pos: Vec2
}

impl Waypoint {
    pub fn new() -> Waypoint {
        Waypoint {
            rel_pos: Vec2 {x: 10, y: 1}
        }
    }

    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Translate(Facing::N, amount) => self.rel_pos.y += amount,
            Instruction::Translate(Facing::E, amount) => self.rel_pos.x += amount,
            Instruction::Translate(Facing::S, amount) => self.rel_pos.y -= amount,
            Instruction::Translate(Facing::W, amount) => self.rel_pos.x -= amount,
            Instruction::Turn(TurnDirection::Left, amount) => self.turn_left(*amount),
            Instruction::Turn(TurnDirection::Right, amount) => self.turn_right(*amount),
            _ => {}
        };
    }

    pub fn turn_left(&mut self, amount: isize) {
        let times = amount / 90;
        for _ in 0 .. times {
            let new_x = self.rel_pos.y * -1;
            self.rel_pos.y = self.rel_pos.x;
            self.rel_pos.x = new_x;
        }
    }

    pub fn turn_right(&mut self, amount: isize) {
        let times = amount / 90;
        for _ in 0 .. times {
            let new_y = self.rel_pos.x * -1;
            self.rel_pos.x = self.rel_pos.y;
            self.rel_pos.y = new_y;
        }
    }
}

#[derive(Debug)]
pub struct Ship {
    pub pos: Vec2,
    pub waypoint: Waypoint
}

impl Ship {
    pub fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            waypoint: Waypoint::new()
        }
    }

    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                let direction = self.waypoint.rel_pos;
                println!("{:?}, {}", direction, amount);
                self.pos += direction * (*amount);
            },
            _ => self.waypoint.run(instruction)
        }
    }
}