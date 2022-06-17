use rand::Rng;
use phf::phf_map;
use std::fmt;

/// Holds the game state.
pub struct Grid {
    //rows contain cols
    rows: Vec<Vec<u8>>,
    pipes: &'static PipeMap,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            rows: vec![vec![0; 4]; 4],
            pipes: &PIPEMAP_THICK,
        }
    }
}

impl Grid {
    /// Creates a new grid with the given size and adds two numbers on random positions.
    /// The size must be greater than 0 and greater than 1 in at least one dimension.
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    ///
    /// ```
    pub fn new(x_size: usize, y_size: usize) -> Grid {
        if x_size < 1 || y_size < 1 || x_size < 2 && y_size < 2 {
            panic!("Grid size cannot be 0");
        }
        let grid = Grid {
            rows: vec![vec![0; x_size]; y_size],
            ..Default::default()
        };
        //add two starting numbers
        grid.add_random_number().unwrap().add_random_number().unwrap()
    }

    /// Creates a new grid from a predefined grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// 
    /// //Create a 4x4 vector
    /// let rows = vec![vec![0; 4]; 4];
    /// //Create a grid from the vector
    /// let grid = Grid::from_rows(rows);
    ///
    /// ```
    pub fn from_rows(rows: Vec<Vec<u8>>) -> Grid {
        Grid {
            rows,
            ..Default::default()
        }
    }

    /// Returns a new instance of the grid, with a custom pipe-map for the borders.
    ///
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// use cli_2048::PIPEMAPS;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    /// //Create a grid with a custom pipe-map
    /// let grid = grid.with_pipes(PIPEMAPS.get("Medium").unwrap());
    /// 
    /// //Or like this
    /// let grid = Grid::new(4, 4).with_pipes(PIPEMAPS.get("Medium").unwrap());
    /// 
    /// ```
    pub fn with_pipes(&self, pipes: &'static PipeMap) -> Grid {
        Grid {
            rows: self.rows.clone(),
            pipes,
        }
    }
    /// Gets the size of the grid in characters and with borders.
    ///
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    /// 
    /// let x_size = 4*2 + (4-1) + 2;
    /// let y_size = 4 + (4-1) + 2;
    ///
    /// assert_eq!(grid.get_size(), (x_size, y_size));
    /// 
    /// ```
    pub fn get_size(&self) -> (usize, usize) {
        (self.rows.len() * self.formatted_numbers()[0][0].len() + self.rows.len() + 1, self.rows[0].len() * 2 + 1)
    }
    /// Returns a new Grid from the previous.
    /// Slides and combines the grid in the given direction.
    /// If the tiles change a new tile will be added at a random empty position.
    /// If the grid is full the game is over (Err("no more options")).
    ///
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// use cli_2048::Direction;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    /// let grid = grid.slide(Direction::Up);
    /// let grid = grid.slide(Direction::Down);
    /// 
    /// ```
    pub fn slide(&self, dir: Direction) -> Result<Grid, &'static str> {
        let mut rows: Vec<Vec<u8>> = self.rows.clone();

        (|| {
            match dir {
                Direction::LEFT => {
                    //Rotate
                    // -
                    //Operate
                    rows = rows.iter().map(|row| self.combine_row(row)).collect();
                    //Rotate back
                    // -
                    //Return
                    return Ok(());
                }
                
                Direction::RIGHT => {
                    //Rotate
                    rows = rows.iter().map(|row| row.iter().rev().cloned().collect()).collect();
                    //Operate
                    rows = rows.iter().map(|row| self.combine_row(row)).collect();
                    //Rotate back
                    rows = rows.iter().map(|row| row.iter().rev().cloned().collect()).collect();
                    //Return
                    return Ok(());
                }
                
                Direction::UP => {
                    //Rotate
                    rows = (0..rows[0].len()).map(|col| rows.iter().map(|row| row[col]).collect()).collect();
                    //Operate
                    rows = rows.iter().map(|row| self.combine_row(row)).collect();
                    //Rotate back
                    rows = (0..rows[0].len()).map(|col| rows.iter().map(|row| row[col]).collect()).collect();
                    //Return
                    return Ok(());
                }
                Direction::DOWN => {
                    //Rotate
                    rows = (0..rows[0].len()).map(|col| rows.iter().map(|row| row[col]).collect()).collect();
                    rows = rows.iter().map(|row| row.iter().rev().cloned().collect()).collect();
                    //Operate
                    rows = rows.iter().map(|row| self.combine_row(row)).collect();
                    //Rotate back
                    rows = rows.iter().map(|row| row.iter().rev().cloned().collect()).collect();
                    rows = (0..rows[0].len()).map(|col| rows.iter().map(|row| row[col]).collect()).collect();
                    //Return
                    return Ok(());
                }
            }
        })()?;

        let new_grid = Grid { rows, ..Default::default() }; 
        let new_grid_with_new_number = new_grid.add_random_number()?;
        //see if grid has changed
        if new_grid.rows != self.rows {
            return Ok(new_grid_with_new_number);
        }
        Ok(new_grid)
    }
    
    fn compress_row(&self, row: &Vec<u8>) -> Vec<u8> {
        let mut new_row = row.iter().filter(|&x| *x != 0).cloned().collect::<Vec<u8>>();
        new_row.append(&mut vec![0; row.len() - new_row.len()]);
        new_row
    }

    fn combine_row(&self, row: &Vec<u8>) -> Vec<u8> {
        let mut row = self.compress_row(&row);
        for i in 0..(row.len() - 1) {
            if row[i] == row[i+1] && row[i] != 0 {
                row[i] += 1;
                row[i+1] = 0;
            }
        }
        self.compress_row(&row)
    }

    fn add_random_number(&self) -> Result<Grid, &'static str> {
        //get index of all 0 cells
        let options: Vec<(usize, usize)> = self.rows.iter().enumerate().flat_map(|(x, row)| {
            row.iter().enumerate().filter(|(_, &cell)| cell == 0).map(move |(y, _)| (x, y))
        }).collect();
        
        //check for no options (GAME OVER)
        if options.is_empty() {
            return Err("no more options");
        }

        let mut rng = rand::thread_rng();

        //get random option
        let option = options[rng.gen_range(0..options.len())];

        let mut power = 1;
        //1 in 10 chance of getting 4
        if rng.gen_range(1..10) == 10 {
            power = 2;
        }

        let mut new_rows = self.rows.clone();
        new_rows[option.0][option.1] = power;

        Ok(Grid { rows: new_rows, ..Default::default() })
    }
    fn formatted_numbers(&self) -> Vec<Vec<String>> {

        let mut longest_string_len = 2;
        for number in self.rows.iter().flatten() {
            if format_number(number, 0).len() > longest_string_len {
                longest_string_len = format_number(number, 0).len();
            }
        }

        return self.rows.iter().map(|row| {
            row.iter().map(|number| format_number(number, longest_string_len)).collect()
        }).collect();

        fn format_number(&number: &u8, len: usize) -> String {
            let digits = format_digits(&number);
            if len == 0 {
                return digits;
            }
            return (0..len-digits.len()).map(|_| " ".to_string()).collect::<Vec<String>>().join("") + &digits;

            fn format_digits(&number: &u8) -> String {
                match number {
                    0 => " ".to_string(),
                    x => format!("{}", (2 as usize).pow(x as u32)),
                }
            }
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_str = format!("{} by {} Grid:\n[\n", self.rows.len(), self.rows[0].len());
        for row in &self.rows {
            grid_str.push_str("  [");
            for val in row {
                grid_str.push_str(&format!(" {} ", val));
            }
            grid_str.push_str("]\n");
        }
        grid_str.push_str("]\n");
        
        
        Ok(
            write!(f, "{}", grid_str)?
        )
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        //set pipes
        let pipes = &self.pipes;

        let grid_str = self.formatted_numbers();

        //draw top border
        write!(f, "{}", pipes.get("top_left").unwrap())?;
        for i in 0..grid_str[0].len() {
            for _ in 0..grid_str[0][0].len() {
                write!(f, "{}", pipes.get("horizontal").unwrap())?;
            }
            if i != grid_str[0].len() - 1 {
                write!(f, "{}", pipes.get("top_horizontal").unwrap())?;
            }
        }
        write!(f, "{}\n", pipes.get("top_right").unwrap())?;

        //draw row
        for i in 0..grid_str.len() {
            for col in &grid_str[i] {
                write!(f, "{}{}", pipes.get("vertical").unwrap(), col)?;
            }
            write!(f, "{}\n", pipes.get("vertical").unwrap())?;

            //Draw bottom border or cross-line
            if i == grid_str.len() - 1 {
                //bottom border
                write!(f, "{}", pipes.get("bottom_left").unwrap())?;
                for i in 0..grid_str[0].len() {
                    for _ in 0..grid_str[0][0].len() {
                        write!(f, "{}", pipes.get("horizontal").unwrap())?;
                    }
                    if i != grid_str[0].len() - 1 {
                        write!(f, "{}", pipes.get("bottom_horizontal").unwrap())?;
                    }
                }
                write!(f, "{}\n", pipes.get("bottom_right").unwrap())?;
            } else {
                //cross-line
                write!(f, "{}", pipes.get("left_vertical").unwrap())?;
                for i in 0..grid_str[0].len() {
                    for _ in 0..grid_str[0][0].len() {
                        write!(f, "{}", pipes.get("horizontal").unwrap())?;
                    }
                    if i != grid_str[0].len() - 1 {
                        write!(f, "{}", pipes.get("cross").unwrap())?;
                    }
                }
                write!(f, "{}\n", pipes.get("right_vertical").unwrap())?;
            }
        }

        Ok(())
    }
}

    /// Used as parameters to the slide function.
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// use cli_2048::Direction;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    /// let grid = grid.slide(Direction::Up);
    /// let grid = grid.slide(Direction::Down);
    /// 
    /// ```
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

type PipeMap = phf::Map<&'static str, &'static str>;

    /// Contains three pipe-map presets:
    /// Thin
    /// Medium
    /// Thick
    /// 
    /// # Examples
    ///
    /// ```
    /// use cli_2048::Grid;
    /// use cli_2048::PIPEMAPS;
    /// 
    /// //Create a 4x4 grid
    /// let grid = Grid::new(4, 4);
    /// //Create a grid with a custom pipe-map
    /// let grid = grid.with_pipes(PIPEMAPS.get("Medium").unwrap());
    /// 
    /// //Or like this
    /// let grid = Grid::new(4, 4).with_pipes(PIPEMAPS.get("Medium").unwrap());
    /// 
    /// ```
pub static PIPEMAPS: phf::Map<&'static str, &'static PipeMap> = phf_map! {
    "Thin" => &PIPEMAP_THIN,
    "Medium" => &PIPEMAP_MEDIUM,
    "Thick" => &PIPEMAP_THICK,
};

static PIPEMAP_THIN: PipeMap = phf_map! {
    "horizontal" => "─",
    "vertical" => "│",
    "top_left" => "┌",
    "top_right" => "┐",
    "bottom_left" => "└",
    "bottom_right" => "┘",
    "top_horizontal" => "┬",
    "bottom_horizontal" => "┴",
    "left_vertical" => "├",
    "right_vertical" => "┤",
    "cross" => "┼",
};
static PIPEMAP_MEDIUM: PipeMap = phf_map! {
    "horizontal" => "━",
    "vertical" => "┃",
    "top_left" => "┏",
    "top_right" => "┓",
    "bottom_left" => "┗",
    "bottom_right" => "┛",
    "top_horizontal" => "┳",
    "bottom_horizontal" => "┻",
    "left_vertical" => "┣",
    "right_vertical" => "┫",
    "cross" => "╋",
};
static PIPEMAP_THICK: PipeMap = phf_map! {
    "horizontal" => "═",
    "vertical" => "║",
    "top_left" => "╔",
    "top_right" => "╗",
    "bottom_left" => "╚",
    "bottom_right" => "╝",
    "top_horizontal" => "╦",
    "bottom_horizontal" => "╩",
    "left_vertical" => "╠",
    "right_vertical" => "╣",
    "cross" => "╬",
};


//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_grid() {
        let _grid = Grid::new(1, 0);
    }
    #[test]
    fn add_random_number() {
        let grid = Grid::new(4, 4);
        let grid = grid.add_random_number().unwrap();
        println!("{:?}", grid);
    }

    #[test]
    fn compress_row() {
        let grid = Grid::new(4, 4);
        let row = grid.compress_row(&vec![2, 0, 4, 0]);
        assert_eq!(row, vec![2, 4, 0, 0]);
        let row = grid.compress_row(&vec![4, 0, 2, 8]);
        assert_eq!(row, vec![4, 2, 8, 0]);
    }

    #[test]
    fn combine_row() {
        let grid = Grid::new(4, 4);
        let row = grid.combine_row(&vec![1, 0, 1, 0]);
        assert_eq!(row, vec![2, 0, 0, 0]);
        let row = grid.combine_row(&vec![2, 2, 3, 4, 6, 6, 5, 0, 6]);
        assert_eq!(row, vec![3, 3, 4, 7, 5, 6, 0, 0, 0]);
    }

    #[test]
    fn overwrite_rows() {
        let grid = Grid::new(2, 2);
        assert_eq!(grid.rows.len(), 2);
        let new_rows = vec![vec![1; 4]; 4];
        let grid = Grid::from_rows(new_rows.clone());
        assert_eq!(grid.rows, new_rows);
    }
}