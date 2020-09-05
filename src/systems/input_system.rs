use crate::components::*;
use crate::constants::*;
use crate::events::*;
use crate::resources::*;
use ggez::event::KeyCode;
use specs::join::Join;
use specs::world::Index;
use specs::Entities;
use specs::ReadStorage;
use specs::System;
use specs::Write;
use specs::WriteStorage;
use std::collections::HashMap;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, Gameplay>,
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut gameplay,
            mut event_queue,
            mut input_queue,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to move.
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, *id)),
                        None => {
                            // find an immovable
                            // if it exists, we need to stop and not move anything
                            // if it doesn't exist, we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_) => {
                                    to_move.clear();
                                    event_queue.events.push(Event::PlayerHitObstacle {})
                                }
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        // We've just moved, so let's increase the number of moves
        if !to_move.is_empty() {
            gameplay.moves_count += 1;
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
                // Fire an event for the entity that just moved
                event_queue
                    .events
                    .push(Event::EntityMoved(EntityMoved { id }));
            }
        }
    }
}
