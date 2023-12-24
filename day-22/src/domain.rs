use std::{ops::{Range, Deref, DerefMut}, cell::RefCell, rc::Rc, hash::Hash};

use glam::UVec3;

pub type Bricks = Vec<BrickRef>;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Brick {
    pub x: Range<u32>,
    pub y: Range<u32>,
    pub z: Range<u32>,
}

impl Brick {
    pub fn new(start: UVec3, end: UVec3) -> Self {
        Self {
            x: start.x .. end.x+1,
            y: start.y .. end.y+1,
            z: start.z .. end.z+1,
        }
    }

    pub fn move_down(&mut self) {
        if self.z.start == 1 { panic!("This brick is on the floor!") }
        self.z = self.z.start - 1 .. self.z.end - 1
    }
}

#[derive(Clone, Debug, Eq)]
pub struct BrickRef(pub Rc<RefCell<Brick>>);

impl BrickRef {
    pub fn from_brick(brick: Brick) -> Self{
        BrickRef(Rc::new(RefCell::new(brick)))
    }
}

impl Deref for BrickRef {
    type Target = Rc<RefCell<Brick>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BrickRef {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for BrickRef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for BrickRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}