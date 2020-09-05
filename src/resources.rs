use std::time::Duration;
use ggez::event::KeyCode;
use specs::World;
use std::fmt;
use std::fmt::Display;

pub enum GameplayState {
    Playing,
    Won,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub type EntityId = u32;

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when an entity is moved
    EntityMoved(EntityMoved),

    // Fired when the box is placed on a spot
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
}
