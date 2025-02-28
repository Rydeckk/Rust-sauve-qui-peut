use crate::global_map::{GlobalMap, MAP_HEIGHT, MAP_WIDTH};
use commun::structs::RelativeDirection;

#[derive(Debug, Clone)]
pub struct RadarView {
    walls_horiz: [[bool; 3]; 4],
    walls_vert: [[bool; 4]; 3],
    cells: [[u8; 3]; 3],
}

impl RadarView {
    pub fn new(walls_horiz: [[bool; 3]; 4], walls_vert: [[bool; 4]; 3], cells: [[u8; 3]; 3]) -> Self {
        Self {
            walls_horiz,
            walls_vert,
            cells,
        }
    }

    pub fn is_open(&self, dir: RelativeDirection) -> bool {
        match dir {
            RelativeDirection::Front => !self.walls_horiz[1][1],
            RelativeDirection::Right => !self.walls_vert[1][2],
            RelativeDirection::Back => !self.walls_horiz[2][1],
            RelativeDirection::Left => !self.walls_vert[1][1],
        }
    }

    pub fn is_wall_horizontal(&self, y: usize, x: usize) -> bool {
        self.walls_horiz[y][x]
    }

    pub fn is_wall_vertical(&self, y: usize, x: usize) -> bool {
        self.walls_vert[y][x]
    }

    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.cells[y][x]
    }

    pub fn has_hint(&self, dir: RelativeDirection) -> bool {
        let (dx, dy) = Self::direction_to_index(dir);
        self.cells[dy][dx] & 0b0100 != 0
    }

    pub fn has_goal(&self, dir: RelativeDirection) -> bool {
        let (dx, dy) = Self::direction_to_index(dir);
        self.cells[dy][dx] & 0b1000 != 0
    }

    fn direction_to_index(dir: RelativeDirection) -> (usize, usize) {
        match dir {
            RelativeDirection::Front => (1, 0),
            RelativeDirection::Right => (2, 1),
            RelativeDirection::Back => (1, 2),
            RelativeDirection::Left => (0, 1),
        }
    }

    pub fn find_player_position(&self) -> Option<(usize, usize)> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0b0001 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn decode_cell(encoded: u8) -> char {
        match encoded {
            0b0000 => ' ',
            0b0001 => 'P',
            0b0010 => 'O',
            0b0100 => 'H',
            0b1000 => 'G',
            0b1111 => '#',
            _ => '?',
        }
    }

    pub fn update_global_map(&self, global_map: &mut GlobalMap) {
        let (px, py) = global_map.player_pos;

        for dy in 0..3 {
            for dx in 0..3 {
                let global_x = px as isize + dx as isize - 1;
                let global_y = py as isize + dy as isize - 1;

                if global_x < 0 || global_y < 0 || global_x as usize >= MAP_WIDTH || global_y as usize >= MAP_HEIGHT {
                    continue;
                }

                let gx = global_x as usize;
                let gy = global_y as usize;
                let cell = self.get_cell(dx, dy);

                if cell == 0b1111 {
                    continue;
                }

                if self.is_wall_horizontal(dy, dx) {
                    global_map.set_wall(gx, gy, '-');
                }
                if self.is_wall_vertical(dy, dx) {
                    global_map.set_wall(gx, gy, '|');
                }

                if cell & 0b1000 != 0 {
                    global_map.set_explored(gx, gy, 'G');
                } else if cell & 0b0100 != 0 {
                    global_map.set_explored(gx, gy, 'H');
                } else {
                    global_map.set_explored(gx, gy, ' ');
                }
            }
        }
    }
}
