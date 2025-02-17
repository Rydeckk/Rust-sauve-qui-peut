use super::{action::{ActionError, RelativeDirection}, challenge::ChallengePosition, player::Player};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 7;

pub const MAZE: [[&str; WIDTH]; HEIGHT] = [
    ["•","-","•","-","•","-","•"],
    ["|"," "," "," "," "," ","|"],
    ["•","-","•"," ","•"," ","•"],
    ["|"," ","|"," ","|"," ","|"],
    ["•"," ","•"," ","•","-","•"],
    ["|"," "," "," "," ","*","|"],
    ["•","-","•","-","•","-","•"]
];

const WALLS: [&str; 3] = ["-","|","•"];
const XPOSCHALLENGE: i32 = 3;
const YPOSCHALLENGE: i32 = 5;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn check_movement_possible(direction: RelativeDirection, player: &Player) -> Result<Point,ActionError> {
    let player_is_challenge_actif = player.clone().get_is_challenge_actif();

    if player_is_challenge_actif == true {return Err(ActionError::SolveChallengeFirst);}
    
    let position = player.clone().get_position();
    let mut new_position: Point = position.clone();

    match direction {
        RelativeDirection::Front => if WALLS.contains(&MAZE[position.y as usize - 1][position.x as usize]) {
            return Err(ActionError::CannotPassThroughWall);
        } else {
            new_position.x = position.x;
            new_position.y = position.y - 2;
        } ,
        RelativeDirection::Right => if WALLS.contains(&MAZE[position.y as usize][position.x as usize + 1]) {
            return Err(ActionError::CannotPassThroughWall);
        } else {
            new_position.x = position.x + 2;
            new_position.y = position.y;
        } ,
        RelativeDirection::Back => if WALLS.contains(&MAZE[position.y as usize + 1][position.x as usize]) {
            return Err(ActionError::CannotPassThroughWall);
        } else {
            new_position.x = position.x;
            new_position.y = position.y + 2;
        } ,
        RelativeDirection::Left => if WALLS.contains(&MAZE[position.y as usize][position.x as usize - 1]) {
            return Err(ActionError::CannotPassThroughWall);
        } else {
            new_position.x = position.x - 2;
            new_position.y = position.y;
        }
    }

    Ok(new_position)
}