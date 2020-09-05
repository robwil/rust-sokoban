use crate::components::BoxColour;
use crate::components::Position;
use crate::entities::*;
use specs::World;

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB KB . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . KS . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Blue);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Red);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Red);
                }
                "KB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Black);
                }
                "KS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Black);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
