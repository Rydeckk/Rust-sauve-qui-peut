use commun::structs::{ActionError, RelativeDirection};
use super::player::Player;
use tracing::{info, warn};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 7;

pub const MAZE: [[char; WIDTH]; HEIGHT] = [
    ['•','-','•','-','•','-','•'],
    ['|',' ',' ',' ',' ',' ','|'],
    ['•','-','•',' ','•',' ','•'],
    ['|',' ','|',' ','|',' ','|'],
    ['•',' ','•',' ','•','-','•'],
    ['|',' ',' ',' ',' ','*','|'],
    ['•','-','•','-','•','-','•']
];

const WALLS: [char; 3] = ['-','|','•'];

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn check_movement_possible(direction: RelativeDirection, player: &Player) -> Result<Point, ActionError> {
    let position = player.get_position();
    info!("Checking movement: {:?} from position: x={}, y={}", direction, position.x, position.y);

    let mut new_position = position.clone();

    match direction {
        RelativeDirection::Front => {
            if WALLS.contains(&MAZE[position.y as usize - 1][position.x as usize]) {
                warn!("Wall detected in Front!");
                return Err(ActionError::CannotPassThroughWall);
            }
            new_position.y -= 2;
        },
        RelativeDirection::Right => {
            if WALLS.contains(&MAZE[position.y as usize][position.x as usize + 1]) {
                warn!("Wall detected on the Right!");
                return Err(ActionError::CannotPassThroughWall);
            }
            new_position.x += 2;
        },
        RelativeDirection::Back => {
            if WALLS.contains(&MAZE[position.y as usize + 1][position.x as usize]) {
                warn!("Wall detected in Back!");
                return Err(ActionError::CannotPassThroughWall);
            }
            new_position.y += 2;
        },
        RelativeDirection::Left => {
            if WALLS.contains(&MAZE[position.y as usize][position.x as usize - 1]) {
                warn!("Wall detected on the Left!");
                return Err(ActionError::CannotPassThroughWall);
            }
            new_position.x -= 2;
        }
    }

    info!("Movement possible to: x={}, y={}", new_position.x, new_position.y);
    Ok(new_position)
}