use specs::{World, System, SystemData, Join, Read, ReadStorage, Write, WriteStorage};
use specs::shrev::{EventChannel, ReaderId};
use piston_window::{PistonWindow, clear, rectangle, Event, Input, Button, ButtonArgs, ButtonState, Key};
use std::collections::HashSet;

use crate::components::{Position, Velocity, Controllable};

///
/// Keeps track of key presses we care about that are currently pressed.
/// We do this so that we don't have to respond to individual Button Events
/// which creates choppy behavior.
/// 
struct KeyPressRegistration {
    /// This is the set of keys that are being pressed right now.
    /// When a key is pressed, it's registered into the vec, and when let go,
    /// is removed from the vec.
    presses: HashSet<Key>,

    key_mask: Option<HashSet<Key>>
}

impl KeyPressRegistration {
    pub fn new() -> KeyPressRegistration {
        KeyPressRegistration {presses: HashSet::new(), key_mask: None}
    }

    ///
    /// Set a group of Keys that we care about registering. If a key is not
    /// in this `key_mask` then `register_key_press` won't register the key.
    /// 
    pub fn with_key_mask<I>(mut self, allowed_keys: I) -> KeyPressRegistration
        where I: IntoIterator<Item=Key> {
        let mut key_mask: HashSet<Key> = HashSet::new();
        for key in allowed_keys.into_iter() {
            key_mask.insert(key);
        }
        self.key_mask = Some(key_mask);
        self
    }

    pub fn register_key_press(&mut self, key: Key) {
        if let Some(allowed) = &self.key_mask {
            if allowed.contains(&key) {
                self.presses.insert(key);
            }
        } else {
            // If key_mask is None, then just register everything
            self.presses.insert(key);
        }
    }

    pub fn deregister_key(&mut self, key: Key) {
        self.presses.remove(&key);
    }

    pub fn current_presses(&self) -> &HashSet<Key> {
        &self.presses
    }
}

pub struct ButtonSystem {
    event_reader_id: ReaderId<piston_window::Event>,
    current_press: KeyPressRegistration
}

impl ButtonSystem {
    pub fn new(world: &mut World) -> ButtonSystem {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<piston_window::Event>>().register_reader();
        ButtonSystem {
            event_reader_id: reader_id, 
            current_press: KeyPressRegistration::new().with_key_mask(vec![Key::Up, Key::Down, Key::Left, Key::Right])
        }
    }
}

impl<'a> System<'a> for ButtonSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controllable>, 
        Read<'a, EventChannel<piston_window::Event>>
    );

    fn run(&mut self, (mut velocities, controllable, event_reader): Self::SystemData) {
        // First grab events and collect all the arrow key presses into the `KeyPressRegistration`
        for event in event_reader.read(&mut self.event_reader_id) {
            match event {
                Event::Input(input, _) => {
                    match input {
                        Input::Button(b) => { 
                            match b {
                                ButtonArgs {
                                    state: ButtonState::Press,
                                    button: Button::Keyboard(k),
                                    scancode: _
                                } => self.current_press.register_key_press(*k),
                                ButtonArgs {
                                    state: ButtonState::Release,
                                    button: Button::Keyboard(k),
                                    scancode: _
                                } => self.current_press.deregister_key(*k),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        for (mut velocity, _control) in (&mut velocities, &controllable).join() {
            // Reset velocity to, so we only go up to max speed, not continously accelerate
            velocity.x = 0.0;
            velocity.y = 0.0;
            if self.current_press.current_presses().contains(&Key::Left) {
                velocity.x += -5.0;
            }
            if self.current_press.current_presses().contains(&Key::Right) {
                velocity.x += 5.0;
            }
            if !self.current_press.current_presses().contains(&Key::Left) && !self.current_press.current_presses().contains(&Key::Right) {
                // If we don't have left and we don't have right, set to 0
                velocity.x = 0.0;
            }

            if self.current_press.current_presses().contains(&Key::Up) {
                velocity.y += -5.0;
            }
            if self.current_press.current_presses().contains(&Key::Down) {
                velocity.y += 5.0;
            }
            if !self.current_press.current_presses().contains(&Key::Up) && !self.current_press.current_presses().contains(&Key::Down) {
                // If we don't have left and we don't have right, set to 0
                velocity.y = 0.0;
            }
        }
    }
}

pub struct MoveVelocitySystem;

impl<'a> System<'a> for MoveVelocitySystem {
    type SystemData = (
        ReadStorage<'a, Velocity>, 
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Controllable>);

    ///
    /// Just add velocity amount the current position. We could do some kind of real time 
    /// timing based on time between frames, but we don't have to worry about that now
    /// Velocity is just number of pixels to move in each frame.
    /// 
    fn run(&mut self, (velocities, mut positions, _controllable): Self::SystemData) {
        for (velocity, mut position) in (&velocities, &mut positions).join() {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}
