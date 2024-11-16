#![allow(unused_assignments)]
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader}, cmp::Ordering,
};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub struct Grid {
    pub tiles: [[Tile; WIDTH]; HEIGHT],
    pub start_tile: Tile,
    pub end_tile: Tile,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            tiles: [[Tile {
                x: 0,
                y: 0,
                g_cost: 0,
                h_cost: 0,
                class: TileType::Normal,
                parent: Coordinate(0, 0),
            }; WIDTH]; HEIGHT],
            start_tile: Tile {
                class: TileType::Start,
                ..Default::default()
            },
            end_tile: Tile { 
                class: TileType::End,
                ..Default::default()
            }
        }
    }

    pub fn get_neighbours(&self, tile: Tile) -> Vec<Tile> {
        let mut neighbors: Vec<Tile> = vec![];

        // get the boundaries 
        let left:   isize = if tile.x as isize - 1 < 0 { 0_isize } else { - 1 };
        let right:  isize = if tile.x as isize + 1 >= WIDTH as isize { 0_isize } else { 1 };
        let top:    isize = if tile.y as isize - 1 < 0 {0_isize} else { - 1 };
        let bottom: isize = if tile.y as isize + 1 >= HEIGHT as isize { 0_isize } else { 1 };

        for x in left..=right{
            for y in top..=bottom{
                if x == 0 && y == 0{ continue; }
                let new_x: isize = tile.x as isize + x as isize; 
                let new_y: isize = tile.y as isize + y as isize; 
                neighbors.push(self.tiles[new_x as usize][new_y as usize]);
            }
        }
        neighbors
    }

    pub fn draw(&self) {
        let mut x_label = String::from(" ");
        let mut top_border = String::from(" ");
        for i in 0..10 {
            top_border.push_str("|---------");
            x_label.push_str(&format!("|-{:02}------", i)[..]);
        }
        top_border.push('|');
        print!("{}\n{}\n", x_label, top_border);

        // for every row
        for x in 0..self.tiles.len() {
            let mut first_line = format!("{}", x);
            let mut second_line = String::from(" ");
            let mut third_line = String::from(" ");
            // for every column
            for y in 0..self.tiles.len() {
                let mut top_display = String::new();
                let mut middle_display = "";
                let mut bottom_display = String::new();

                // based on the class
                // display the relevant information
                match self.tiles[y][x].class {
                    TileType::Normal => {
                        top_display = format!(
                            "| {:03} {:03} ",
                            self.tiles[x][y].h_cost, self.tiles[x][y].g_cost
                        );
                        middle_display = "|         ";
                        bottom_display = format!(
                            "|   {:03}   ",
                            self.tiles[x][y].h_cost + self.tiles[x][y].g_cost
                        );
                    }
                    TileType::Wall => {
                        top_display    = "|#########".to_string();
                        middle_display = "|#########";
                        bottom_display = "|#########".to_string();
                    }
                    TileType::Start => {
                        top_display    = "|#########".to_string();
                        middle_display = "|    A    ";
                        bottom_display = "|#########".to_string();
                    }
                    TileType::End => {
                        top_display    = "|#########".to_string();
                        middle_display = "|    B    ";
                        bottom_display = "|#########".to_string();
                    }
                    TileType::Path => {
                        top_display    = "|$$$$$$$$$".to_string();
                        middle_display = "|$$$$$$$$$";
                        bottom_display = "|$$$$$$$$$".to_string();
                    }
                }
                first_line.push_str(&top_display);
                second_line.push_str(middle_display);
                third_line.push_str(&bottom_display);
            }
            first_line.push('|');
            second_line.push('|');
            third_line.push('|');
            println!("{}", first_line);
            println!("{}", second_line);
            println!("{}", third_line);

            print!(" ");
            // seperate the rows
            for _ in 0..10 {
                print!("|---------")
            }
            println!("|");
        }
    }

    pub fn retrace_path(&mut self) {
        println!("the endtile's parent is {:?}", self.end_tile.parent);
        let mut current_node = self.end_tile;
        while current_node != self.start_tile{
            self.tiles[current_node.x as usize][current_node.y as usize].class = TileType::Path;
            current_node = self.tiles[current_node.parent.0 as usize][current_node.parent.1 as usize];
        }
    }
}

impl Default for Grid{
    fn default() -> Self {
        Grid::new()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Start,
    End,
    Normal,
    Wall,
    Path,
}

impl Display for TileType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self
        )
    }
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Normal
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Coordinate(pub i32, pub i32);

#[derive(Clone, Copy, Default)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub g_cost: i32,
    pub h_cost: i32,
    pub class: TileType,
    pub parent: Coordinate,
}

impl PartialEq for Tile{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Tile{
    fn ge(&self, other: &Self) -> bool {
        self.f_cost() >= other.f_cost()
    }
    fn gt(&self, other: &Self) -> bool {
        self.f_cost() > other.f_cost()
    }
    fn le(&self, other: &Self) -> bool {
        self.f_cost() <= other.f_cost()
    }
    fn lt(&self, other: &Self) -> bool {
        self.f_cost() < other.f_cost()
    }
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
impl Eq for Tile{}
impl Ord for Tile{
    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized, {
        if self.f_cost() < min.f_cost() { return min; }
        if self.f_cost() < max.f_cost() { return max; }
        self
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.f_cost().partial_cmp(&other.f_cost()){
            Some(f_order) => f_order,
            None => Ordering::Equal,
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tile: {} ({}, {}) => g_cost: {}, h_cost: {}, f_cost: {}",
            self.class,
            self.x,
            self.y,
            self.g_cost,
            self.h_cost,
            self.g_cost + self.h_cost,
            
        )
    }
}

impl Tile{
    pub fn f_cost(&self) -> i32{
        self.g_cost + self.h_cost
    }
}

pub fn get_distance(tile_a: &Tile, tile_b: &Tile) -> i32{
    let distance_x = (tile_a.x - tile_b.x).abs();
    let distance_y = (tile_a.y - tile_b.y).abs();

    if distance_x > distance_y{
        return 14 * distance_y + 10 * (distance_x - distance_y);
    }
    14 * distance_x + 10 * (distance_y - distance_x)
}

pub fn read_from_file() -> Grid {
    let mut grid = Grid::new();

    let file = File::open("map.txt").expect("map.txt not found");
    let buf_reader = BufReader::new(file);

    for (y, line) in buf_reader.lines().flatten().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            grid.tiles[x][y] = Tile {
                x: x as i32,
                y: y as i32,
                ..Default::default()
            };
            match letter {
                '#' => {
                    grid.tiles[x][y].class = TileType::Wall;
                }
                '.' => {
                    grid.tiles[x][y].class = TileType::Normal;
                }
                'A' => {
                    grid.tiles[x][y].class = TileType::Start;
                    grid.start_tile = grid.tiles[x][y];
                }
                'B' => {
                    grid.tiles[x][y].class = TileType::End;
                    grid.end_tile = grid.tiles[x][y];
                }
                _ => {}
            }
        }
    }

    grid
}
