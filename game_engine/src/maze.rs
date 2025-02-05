pub struct Maze {
    width: u32,
    height: u32,
    grid: Vec<Vec<char>>, // '.' = libre, '#' = mur
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            grid: vec![vec!['.'; width as usize]; height as usize],
        }
    }
}
