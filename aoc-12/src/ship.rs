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
pub struct Ship {
    pub x: isize,
    pub y: isize,
    pub facing: Facing
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Facing::E
        }
    }

    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Translate(Facing::N, amount) => self.y += amount,
            Instruction::Translate(Facing::E, amount) => self.x += amount,
            Instruction::Translate(Facing::S, amount) => self.y -= amount,
            Instruction::Translate(Facing::W, amount) => self.x -= amount,
            Instruction::Turn(TurnDirection::Left, amount) => self.turn_left(*amount),
            Instruction::Turn(TurnDirection::Right, amount) => self.turn_right(*amount),
            Instruction::Forward(amount) => self.run(&Instruction::Translate(self.facing, *amount)),
            _ => {}
        };
    }

    pub fn turn_left(&mut self, amount: isize) {
        let times = amount / 90;
        for _ in 0 .. times {
            self.facing = match self.facing {
                Facing::N => Facing::W,
                Facing::E => Facing::N,
                Facing::S => Facing::E,
                Facing::W => Facing::S,
                Facing::None => self.facing
            };
        }
    }

    pub fn turn_right(&mut self, amount: isize) {
        let times = amount / 90;
        for _ in 0 .. times {
            self.facing = match self.facing {
                Facing::N => Facing::E,
                Facing::E => Facing::S,
                Facing::S => Facing::W,
                Facing::W => Facing::N,
                Facing::None => self.facing
            };
        }
    }
}