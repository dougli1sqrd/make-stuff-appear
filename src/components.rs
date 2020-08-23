use specs::{Component, VecStorage, NullStorage};


#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct BoxShape {
    pub size: f32
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct Controllable;


impl Default for Controllable {
    fn default() -> Self {
        Controllable
    }
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct Renderable;

impl Default for Renderable {
    fn default() -> Self {
        Renderable
    }
}