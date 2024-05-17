/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

use std::fmt;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Tetromino */
/// Represents a Tetromino character '[ ]' with its x and y position (Simulates a pixel)
#[derive(Copy, Clone)]
struct TetrominoCharacter {
    x: i32,
    y: i32,
    value: &'static str, // Use &'static str for string literals
}

impl TetrominoCharacter {
    fn new(x_pos: i32, y_pos: i32, value: &'static str) -> TetrominoCharacter {
        TetrominoCharacter {
            x: x_pos,
            y: y_pos,
            value,
        }
    }

    fn default() -> Self {
        TetrominoCharacter {
            x: 0,
            y: 0,
            value: "", // Default value, replace with actual default if needed
        }
    }

    fn move_character(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;
    }
}

struct Line {
    x: i32,                              //Represents thes start of the line
    y: i32,                              // Represents the y position of the line
    characters: Vec<TetrominoCharacter>, // List of TetrominoCharacter
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for character in &self.characters {
            write!(f, "{}", character.value)?;
        }
        Ok(())
    }
}

impl Line {
    fn new(x_pos: i32, y_pos: i32, num: i32) -> Line {
        Line {
            x: x_pos,
            y: y_pos,
            characters: Line::create_characters(x_pos, y_pos, num),
        }
    }

    /// Creates a list of TetrominoCharacter based on the given x and y position and the number of characterss
    fn create_characters(x_pos: i32, y_pos: i32, num: i32) -> Vec<TetrominoCharacter> {
        let mut characters: Vec<TetrominoCharacter> = Vec::new();

        for i in 0..num {
            characters.push(TetrominoCharacter::new(x_pos + i, y_pos, "[ ]"));
        }

        characters
    }

    /// Moves the line with its characters by the given x and y unitss
    fn move_line(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;

        for character in &mut self.characters {
            character.move_character(x_units, y_units);
        }
    }
}

struct Tetromino {
    first: Line,
    second: Line,
    third: Line,
    fourth: Line,
    multi_line: bool,
    rotation: i32,
    shape_type: i32,
    stationary: bool,
}

impl Tetromino {
    fn new(first_line: Line, second_line: Line, multiple_line: bool, shape_type: i32) -> Tetromino {
        Tetromino {
            first: first_line,
            second: second_line,
            multi_line: multiple_line,
            third: Line::new(0, 0, 0),
            fourth: Line::new(0, 0, 0),
            rotation: 0,
            shape_type,
            stationary: false,
        }
    }

    fn move_tetromino(
        &mut self,
        x_units: i32,
        y_units: i32,
        game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
    ) {
        let collides: bool = self.collides(game_borders);

        if self.stationary {
            return;
        }

        if collides {
            self.stationary = true;

            //self.remake_gameborders(game_borders);
        } else {
            self.first.move_line(x_units, y_units);
            self.second.move_line(x_units, y_units);
            self.third.move_line(x_units, y_units);
            self.fourth.move_line(x_units, y_units);
        }
    }

    fn blank_tetromino(x_position: i32) -> Tetromino {
        Tetromino::new(Line::new(x_position, 1, 0), Line::new(0, 2, 0), false, 0)
    }

    /// Checks if the tetromino collides with the game borders
    ///
    /// # Arguments
    ///
    /// * `game_borders` - A vector containing the game borders
    ///
    /// # Returns
    ///
    /// A boolean indicating if the tetromino collides with the game borders
    fn collides(&mut self, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) -> bool {
        let mut collides: bool = false;

        // Iterates through the y axis
        for y in 1..game_borders.len() - 1 {
            // Checks if the fourth line is empty (as it is not always used)
            if !self.fourth.to_string().is_empty() {
                // Iterates through the characters of the fourth line's characters
                for character in &self.fourth.characters {
                    // Iterates through the x axis
                    for x in 1..game_borders[y].len() {
                        // Checks if the characters X coordinate is the same as the game border's X coordinate, and if the game border at the fourth line's Y coordinate is true
                        if character.x == x as i32
                            && game_borders[(self.fourth.y + 1) as usize][x] == true
                        {
                            // collides is set to true
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            // Apply the same process for the rest of the lines
            if !self.third.to_string().is_empty() {
                for character in &self.third.characters {
                    for x in 1..game_borders[y].len() {
                        if character.x == x as i32
                            && game_borders[(self.third.y + 1) as usize][x] == true
                        {
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            if !self.second.to_string().is_empty() {
                for character in &self.second.characters {
                    for x in 1..game_borders[y].len() {
                        if character.x == x as i32
                            && game_borders[(self.second.y + 1) as usize][x] == true
                        {
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            // As the first line is always used, skip the check if it is empty
            for character in &self.first.characters {
                for x in 1..game_borders[y].len() {
                    if character.x == x as i32
                        && game_borders[(self.first.y + 1) as usize][x] == true
                    {
                        collides = true;
                        return collides;
                    }
                }
            }
        }

        return collides;
    }

    fn clear(&mut self) {
        self.first.characters.clear();
        self.second.characters.clear();
        self.third.characters.clear();
        self.fourth.characters.clear();
    }

    fn rotate(&mut self, rotation: i32, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) {
        if self.stationary {
            return;
        }

        let mut new_rotation = self.rotation + rotation;
        let normalized_rotation = match new_rotation % 360 {
            r if r < 0 => r + 360,
            r => r,
        } % 360;

        new_rotation = match normalized_rotation {
            360 => 0,
            r => r,
        };

        if self.shape_type == 3 {
            return;
        }

        let past_rotation = self.rotation;
        self.rotation = new_rotation;

        self.rotate_shape(self.rotation);

        if self.collides(game_borders) {
            self.rotation = past_rotation;
            self.rotate_shape(self.rotation);
        }
    }

    fn rotate_shape(&mut self, rotation: i32) {
        match rotation {
            90 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                    self.fourth = Line::new(self.first.x, self.first.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x - 1, self.first.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x + 1, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                }
                _ => {}
            },
            180 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 4);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 3);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 3);
                }
                _ => {}
            },
            270 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                    self.fourth = Line::new(self.first.x, self.first.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 2);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x - 1, self.first.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);

                    self.second.x = self.first.x - 1;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                _ => {}
            },
            0 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 4);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 3);
                    self.second = Line::new(self.first.x + 2, self.first.y + 1, 1);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x - 1, self.first.y, 3);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 1);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/* Game loop */

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    let mut game_borders: [[bool; WIDTH]; HEIGHT + 1] = [[false; WIDTH]; HEIGHT + 1];
    for x in 0..WIDTH {
        game_borders[HEIGHT][x] = true;
    }

    // When the tetrominoes' stationary state is reached, they are added to the built_tetrominoes array
    // and removed from the unredered_tetrominoes_list
    //
    // The built_tetrominoes array is used to check for collisions with the tetrominoes that have already
    // been built. Also, the built_tetrominoes array is used to render the tetrominoes that have already
    // been built and to handle the scoring system.
    //
    // Also helps in reducing the number of computations per second needed to render moving tetrominoes as the unredered_tetrominoes_list reduces in size

    let mut built_tetrominoes: [[TetrominoCharacter; WIDTH]; HEIGHT] =
        [[TetrominoCharacter::default(); WIDTH]; HEIGHT];

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut movement_counter = 1;

    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
        &mut stdout,
        &built_tetrominoes,
    );

    loop {
        //write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, -1, &mut game_borders);

            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
            );
        }
        if let Some(Ok(b'd')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, 1, &mut game_borders);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
            );
        }
        if let Some(Ok(b'r')) = b {
            rotate_tetrominoes(&mut unrendered_tetrominoes_list, 90, &mut game_borders);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
            );
        }

        thread::sleep(Duration::from_millis(100));

        let mut create: bool = true;

        for tetromino in &unrendered_tetrominoes_list {
            if !tetromino.stationary {
                create = false;
                break;
            }
        }

        if create {
            create_tetronimo(&mut unrendered_tetrominoes_list);
        }

        if movement_counter % 2 == 0 {
            update_tetrominoes(&mut unrendered_tetrominoes_list, &mut game_borders);
        }
        movement_counter += 1;

        display_screen(
            &screen,
            &mut unrendered_tetrominoes_list,
            &mut rendered_tetrominoes_list,
            &mut stdout,
            &built_tetrominoes,
        );
        move_to_built(&mut unrendered_tetrominoes_list, &mut built_tetrominoes);

        check_complete_line(&mut built_tetrominoes);

        remake_gameborders(&mut game_borders, &mut built_tetrominoes);

        stdout.flush().unwrap();
    }
}

fn create_screen(screen: &mut [[&str; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        screen[i][0] = "<!";
        for j in 1..WIDTH - 1 {
            screen[i][j] = " . ";
        }
        screen[i][WIDTH - 1] = "!>";
    }
}

fn display_screen(
    screen: &[[&str; WIDTH]; HEIGHT],
    unrendered_tetrominoes: &mut Vec<Tetromino>,
    rendered_tetrominoes: &mut Vec<Tetromino>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    built_tetroinoes: &[[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    // writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            if built_tetroinoes[i][j].x == j as i32 && built_tetroinoes[i][j].y == i as i32 {
                write!(stdout, "{}", built_tetroinoes[i][j].value).unwrap();
                j += 1;
                continue;
            }

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first.characters.len()) as usize;
                let skip_distance_second = (tetromino.second.characters.len()) as usize;

                let skip_distance_third = (tetromino.third.characters.len()) as usize;

                let skip_distance_fourth = (tetromino.fourth.characters.len()) as usize;

                if tetromino.rotation == 0 || tetromino.shape_type == 3 {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        write!(stdout, "{}", tetromino.first).unwrap();
                        j += skip_distance_first;
                        found_tetromino = true;
                        if tetromino.multi_line == false {
                            rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        }
                        break;
                    } else if tetromino.multi_line == true
                        && (tetromino.second.x as usize == j && tetromino.second.y as usize == i)
                    {
                        found_tetromino = true;
                        j += skip_distance_second;
                        write!(stdout, "{}", tetromino.second).unwrap();
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        break;
                    }
                } else {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        if tetromino.shape_type == 1 && tetromino.rotation == 180 {
                            write!(stdout, "{}", tetromino.first).unwrap();
                            stdout.flush().unwrap();

                            j += skip_distance_first;
                            found_tetromino = true;
                            if tetromino.multi_line == false {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        } else {
                            write!(stdout, "{}", tetromino.first).unwrap();
                            stdout.flush().unwrap();
                            j += skip_distance_first;
                            found_tetromino = true;
                            break;
                        }
                    } else if tetromino.second.x as usize == j && tetromino.second.y as usize == i {
                        found_tetromino = true;
                        j += skip_distance_second;
                        write!(stdout, "{}", tetromino.second).unwrap();
                        stdout.flush().unwrap();
                        break;
                    } else if tetromino.third.x as usize == j && tetromino.third.y as usize == i {
                        if tetromino.third.to_string().is_empty() {
                            break;
                        } else {
                            found_tetromino = true;
                            j += skip_distance_third;
                            write!(stdout, "{}", tetromino.third).unwrap();
                            stdout.flush().unwrap();
                            if tetromino.shape_type != 1 {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        }
                    } else if tetromino.shape_type == 1
                        && tetromino.fourth.x as usize == j
                        && tetromino.fourth.y as usize == i
                    {
                        found_tetromino = true;
                        j += skip_distance_fourth;
                        write!(stdout, "{}", tetromino.fourth).unwrap();
                        stdout.flush().unwrap();
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        break;
                    }
                }

                x += 1;
            }

            if !found_tetromino {
                write!(stdout, "{}", screen[i][j]).unwrap();
                j += 1;
            }
        }
        write!(stdout, "\n\r").unwrap();
    }

    unrendered_tetrominoes.append(rendered_tetrominoes);
}

fn move_to_built(
    tetrominoes_list: &mut Vec<Tetromino>,
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    let mut i = 0;
    while i < tetrominoes_list.len() {
        if tetrominoes_list[i].stationary {
            let tetromino = tetrominoes_list.remove(i);

            for character in tetromino.first.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.second.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.third.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.fourth.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
        } else {
            i += 1;
        }
    }
}

fn check_complete_line(built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT]) -> bool {
    let mut complete_line = true;
    for i in 0..HEIGHT {
        complete_line = true;
        for j in 1..WIDTH - 1 {
            if built_tetrominoes[i][j].value.is_empty() {
                complete_line = false;
                break;
            }
        }

        if complete_line {
            // Clear the current line
            for j in 0..WIDTH {
                built_tetrominoes[i][j] = TetrominoCharacter::default();
            }

            // Shift all rows above the current line down by one row
            for k in (1..=i).rev() {
                for j in 0..WIDTH {
                    built_tetrominoes[k][j] = built_tetrominoes[k - 1][j];
                    // Move character down by one row
                    built_tetrominoes[k][j].move_character(0, 1);
                }
            }

            // Clear the top row since it has been shifted down
            for j in 0..WIDTH {
                built_tetrominoes[0][j] = TetrominoCharacter::default();
            }
        }
    }
    complete_line
}

fn remake_gameborders(
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if built_tetrominoes[i][j].value.is_empty() {
                game_borders[i][j] = false;
            } else {
                game_borders[i][j] = true;
            }
        }
    }
}

fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 4);
            tetromino_shape.shape_type = 1;
        }
        2 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 3);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 2, tetromino_shape.first.y + 1, 1);
            tetromino_shape.second.x = x_position + 2;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 2;
        }
        3 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 2);
            tetromino_shape.second.characters =
                Line::create_characters(x_position, tetromino_shape.first.y + 1, 2);
            tetromino_shape.second.x = x_position;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 3;
        }
        4 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 2);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first.y + 1, 2);
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 4;
        }
        5 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 3);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first.y + 1, 1);
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 5;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

fn move_tetrmonioes(
    tetrominoes_list: &mut Vec<Tetromino>,
    movement: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(movement, 0, game_borders);
    }
}

fn rotate_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    rotation: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation, game_borders);
        print!("{}", tetromino.rotation.to_string());
    }
}

fn update_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1, game_borders);
    }
}

/* Random helper methods */

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
