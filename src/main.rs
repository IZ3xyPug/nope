use std::collections::HashMap;
use crossterm::{execute, terminal::ClearType};
use std::io::{stdout, Write};

fn main() {
    let mut gamestate: GameState = GameState::new();
    let start_coord: Coordinate = Coordinate::new(2, 1);
    gamestate.add_char(start_coord, '^');
	gamestate.remove_char(start_coord);
    gamestate.start();
} 
#[derive(Debug, Hash)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
struct Coordinate {
    x: u8, 
    y: u8
}
//this constructor allows me to give a value to the struct but only when i give the values through main using ::new();
impl Coordinate {
    fn new(x: u8, y: u8) -> Coordinate {
        Coordinate{x: x, y: y}
    }
}
#[derive(Debug)]
//the gamestate contains a hashmap which maps a given coordinate to a character
struct GameState {
    board: HashMap<Coordinate, char>
}

impl GameState {
    fn new() -> GameState {
        GameState{board: HashMap::new()}
    }
	//TODO: print clears the screen then loops through all the coordinates x and y and checks each coordinate for a ship. Prints the ship or if there's not a ship prints star.


    fn print(&self) {
        execute!(stdout(), crossterm::terminal::Clear(ClearType::All)).unwrap();
		//println!("{:#?}", self);
        for x in 0..20 {
            for y in 0..20 {
                match self.board.get(&Coordinate::new(x, y)) {
                    None => {
                        print!("{}", '*')
                    },
                    Some(ch) => {
                        print!("{}", ch)
                    }
                }
            }
            print!("\n");
        }
    }
	//start has the &self parameter that allows the struct to be readline. it starts its loop with a tick speed import and then it calls the print logic. 
    fn start(&self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            self.print()
        }
    }

    //add_char takes in coord: Coordinate and ship: char and inserts ship at coord
    fn add_char(&mut self, coord: Coordinate, ship: char) {
        self.board.insert(coord, ship);
    }
	//TODO: remove_char takes in coord: Coordinate and .remove it from the board
	fn remove_char(&mut self, coord: Coordinate) {
		self.board.remove(&coord);
	}
}
//TODO: replace the Coordinate structure with (u8, u8)
