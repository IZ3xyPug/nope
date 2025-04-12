use rust_on_rails::prelude::*;

use std::collections::HashMap;
use crossterm::{execute, terminal::ClearType};
use std::io::{stdout, Write};
use std::time::Instant;

pub type Coord = (u8, u8);
const BSIZE: u8 = 20;
const CSIZE: u8 = 19;
const SIZE: u32 = 30;
//COMPLETED: create an enum Ship with variants DownBullet, UpBullet.
//COMPLETED: give it a method called get_char that returns the character based on its variant.
//TODO: switch out the char in board to Ship and use the draw method to get the character needed to print.
#[derive(Clone, Debug)]
enum Ship {
	DownBullet,
	UpBullet,
}

struct MyApp {
	player_position: Coord,
	board: HashMap<Coord, Ship>,
	last_tick: Instant,
}

impl App for MyApp {
	async fn new(ctx: &mut Context) -> Self {
		MyApp{
        	board: HashMap::from([((10, 5), Ship::DownBullet)]),
			player_position: (10, 19),
			last_tick: Instant::now(),
		}
	}

	async fn draw(&mut self, ctx: &mut Context) {
		if self.last_tick.elapsed().as_millis() > 1000 {
			Ship.next_move(ctx, self.board, self.player_position);
			self.last_tick = Instant::now();
		}
        self.print(ctx)
	}
	async fn on_click(&mut self, ctx: &mut Context) {}
	async fn on_move(&mut self, ctx: &mut Context) {}
	async fn on_press(&mut self, ctx: &mut Context, t: String) {
		//COMPLETED: make it so if you hit the character e you'll shoot
			match t.as_str() {
				"a" if self.player_position.0 > 0 => self.player_position = (self.player_position.0-1, self.player_position.1),
				"d" if self.player_position.0 < BSIZE - 1 => self.player_position = (self.player_position.0+1, self.player_position.1),
				"w" if self.player_position.1 > BSIZE - 3 => self.player_position = (self.player_position.0, self.player_position.1-1),
				"s" if self.player_position.1 < CSIZE => self.player_position = (self.player_position.0, self.player_position.1+1),
				"e" => {
					let above = self.board.insert((self.player_position.0, self.player_position.1-1), Ship::UpBullet);
				}
				_ => {}
			};
			println!("{}", t);
		}
}
impl Ship {
	fn get_char(self) -> char  {
		match self {
			Ship::UpBullet => '|',
			Ship::DownBullet => '/',
		}
	}

	fn draw(&self, ctx: &mut Context, x: u8, y: u8) {
		match self {
			Ship::UpBullet => ctx.draw(CanvasItem::Shape(
				Area((SIZE * x as u32, SIZE * y as u32), None),
				Shape::Ellipse(0, (SIZE, SIZE)),
				"0000FF", 255
			)),
			Ship::DownBullet => ctx.draw(CanvasItem::Shape(
				Area((SIZE * x as u32, SIZE * y as u32), None),
				Shape::Ellipse(0, (SIZE, SIZE)),
				"ffD700", 255
			)),
			_ => {}
		};
	}
	fn next_move(mut self, ctx: &Context, board: HashMap<Coord, Ship>, player_position: Coord) {
		let occupied: Vec<Coord> = board.keys()
			.copied()
			.collect();
			print!("{:?}", board);
		for coord in occupied {
			match board.get(&coord) {
				Some(borrowed_char) => {
					let ship = borrowed_char.clone();
					let next_coord: Coord = match ship {
						Ship::UpBullet => (coord.0, coord.1-1),
						Ship::DownBullet => (coord.0, coord.1+1),
						_ => (coord.0, coord.0),
					};
					self.remove_char(&coord);
					if coord == player_position {
						panic!("HOUSTON WE HAVE A PROBLEM")
					}
					match board.get(&next_coord) {
						Some(_) => {
							self.remove_char(&next_coord);
						},
						None => {
							self.add_char(next_coord, ship);
						}
					};
				}
				None => {}
			}
		}
	}
}
impl MyApp {
    fn add_char(&mut self, coord: Coord, ship: Ship) {
        self.board.insert(coord, ship);
	}
	fn remove_char(&mut self, coord: &Coord) {
		self.board.remove(coord);
	}
	fn print(&self, ctx: &mut Context) {
		ctx.clear("000000");
        for y in 0..BSIZE {
            for x in 0..BSIZE {
				let coord = (x, y);
				match coord == self.player_position {
					true => {
						ctx.draw(CanvasItem::Shape(
							Area((SIZE * x as u32, SIZE * y as u32), None),
							Shape::Ellipse(0, (SIZE, SIZE)),
							"ffffff", 255
						));
					},
					false => {
						match self.board.get(&coord) {
							None => {
								ctx.draw(CanvasItem::Shape(
									Area((SIZE * x as u32, SIZE * y as u32), None),
									Shape::Ellipse(0, (SIZE, SIZE)),
									"ff00ff", 255
								));
							},
							Some(ship) => {}
						}
					}
				}
            }
        }
    }
}


create_entry_points!(MyApp);

fn main() {
	desktop_main()
}
