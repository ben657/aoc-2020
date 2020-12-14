use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
	fn add_assign(&mut self, _rhs: Self) {
		self.x += _rhs.x;
		self.y += _rhs.y;
	}
}

impl ops::Sub<Vec2> for Vec2 {
	type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}