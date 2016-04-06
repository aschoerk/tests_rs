extern crate core;
extern crate memstream;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;
use core::iter::Enumerate;
use std::fmt::Debug;




fn print<T: Debug>(x: T) {
	println!("{:?}",x);
}

struct Point {
	x: i32,
	y: i32
}

struct Lexer<'a> {
	lines: Enumerate<Lines<BufReader<&'a [u8]>>>,
	char_count: i32,
	current_x: i32,
	current_y: i32,
	points: Vec<Point>,
	
}

impl<'a> Lexer<'a> {

	
	fn new_by_str(content: &str) -> Lexer  {
		let mut br = BufReader::new(content.as_bytes()); 
		Lexer {
      		lines: br.lines().enumerate(),
			char_count: 0,
			current_x: 0,
			current_y: 0,
			points: Vec::<Point>::new(),
		}
	}
	
	fn read_next_line(&mut self, linenumber_p: &mut usize, line_p: &mut String) -> bool {
		let (linenumber, line) = self.lines.next().unwrap_or((99999999,Ok("".to_string())));
		let ln: usize = linenumber;
		if linenumber == 99999999 {
				return false;
		} else {
			*linenumber_p = ln;			
			*line_p	 = line.unwrap();
		
			return true;
		}
	}
	
	fn read_lines(&mut self) {
		let mut line = String::new();
		let mut linenumber = 0;
		let mut data_started = false;
		while self.read_next_line(&mut linenumber, &mut line) {
			println!("{}: {}", linenumber, line);
			let mut chars = line.chars();
			match chars.next() {
				Some(c) => {
					if !data_started {
						match c {
							'#' => continue,
							'x' => {
								data_started = true;
								continue;
							},
							_ => {
								print("unexpected line");
								continue;
							},
						}
					} else {
						self.eat_char(c);
						loop {
							match chars.next() {
								Some('!') =>  {
									self.eat_char('!');
									data_started = false;
								},
								Some(c) => {
									self.eat_char(c);
								}
								None => break, 
							}							
						}
					}
				},
				None => continue,
			}			
		}
	}
	
    
		                	
	fn eat_char(&mut self, c: char) {
		println!("eating: {:?}",c);
		match c {
			'0' ... '9' => {
				self.char_count *= 10;
				self.char_count += c.to_digit(10).unwrap() as i32;
			},
			'!' => return,
			'$' => { self.current_x = 0; self.current_y += 1; }, 
			'b' => { 
				if self.char_count == 0 {
					self.char_count = 1;
				}
				self.current_x += self.char_count; 
				self.char_count = 0; },
			'o' => { 
				if self.char_count == 0 {
					self.char_count = 1;
				}
				for i in 0..self.char_count {
					self.points.push(Point { x: self.current_x, y: self.current_y});
					self.current_x += 1;
				} 
				self.char_count = 0;
			},
			_ => return,
			
		}		
	}
}


fn read_str(buffer: &str) {
	let mut lexer = Lexer::new_by_str(buffer);
	lexer.read_lines();
}

fn main() {
    // let name = env::args().nth(1).unwrap();
    // println!("arg: {}",env::args().nth(1).unwrap());
    // read_file("NewP7-2009-03-16.rle");
    read_str("#C NewP7-2009-03-16.rle\n
#C by N.Beluchenko\n
x=104, y=26\n
b2o26b2o29b2o27b2o$3bo5bo19b2o30bo28bo$2bo2b2o3bo14b2o6b2o24bo28bo$bo3bo4b\n
o17b4o27b5o24b5o$o2bo3bobo14bo10bo27bo28bo$b4o19bobob4obobo23b3o26b3o$22b\n
2ob2obo2bob2ob2o17b2obo25b2obo$b4o15bo3bo2bo4bo2bo3bo15b2obo4bo19bobobo4b\n
o$o2bo3bobo10bo2b2o10b2o2bo18bo5bo18bobobo5bo$bo3bo4bo14bo8bo20b3o5bo18b2ob\n
o6bo$2bo2b2o3bo10bob2o10b2obo12b2obo6b2o3bo18bo4b2o3bo$3bo5bo8b2obobo12bob\n
ob2o7bo2bobo10bobo2b2o13bobo6bobo2b2o$b2o15b2obobo12bobob2o7b2obobo4bo4bo6b\n
o14b2o5bo6bo$21bob2o10b2obo13bo6bo4bo4bobob2o18bo4bobob2o$25bo8bo17b2o2bob\n
o10bobo2bo23bobo2bo$20bo2b2o10b2o2bo17bo3b2o6bob2o25bob2o$20bo3bo2bo4bo2b\n
o3bo20bo5b3o22b2o2b3o$22b2ob2obo2bob2ob2o21bo5bo25bo$24bobob4obobo24bo4bob\n
2o23b6o$24bo10bo29bob2o28bo$28b4o30b3o29b3o$25b2o6b2o25bo33bo$29b2o29b5o$29b\n
2o33bo$62bo$62b2o!\n
x=104, y=27, h=0, v=-1, color=(0,0,0), alpha=48\n
3bo$b5o23b2o29b2o27b2o$b5o3bo18b4o29bo28bo$b6ob3o14b2obo2bob2o24bo28bo$11o14b\n
10o24b5o24b5o$o2b5obo14b4o4b4o27bo28bo$b5o18bobob4obobo23b3o26b3o$22b2ob2ob\n
o2bob2ob2o17b2obob2o22b2obob2o$b5o14b3obob2o4b2obob3o15b2ob6o19bobob6o$o2b\n
5obo10b6o8b6o18b7o18bobob7o$11o10b2o2bo8bo2b2o16b3o2b5o17b2ob2o2b5o$b6ob3o8b\n
3ob2o10b2ob3o10b2obob2o3b3ob2o18bo4b3ob2o$b5o3bo8b2obobo12bobob2o7bo2bob5o5b\n
4o2b2o13bobo5b4o2b2o$b5o12b2obobo12bobob2o7b2obob6o4b4o3bo14b2o5b4o3bo$3b\n
o15b3ob2o10b2ob3o11bo3b4o4b6obob2o18b6obob2o$21b2o2bo8bo2b2o13b2o2b4o5b5ob\n
o2bo19b5obo2bo$20b6o8b6o17b2ob3o3b2obob2o22b2obob2o$20b3obob2o4b2obob3o19b\n
5o2b3o22b2o2b3o$22b2ob2obo2bob2ob2o21b7o25bo2bo$24bobob4obobo24b6ob2o23b6o$24b\n
4o4b4o26b2obob2o28bo$25b10o27b3o29b3o$25b2obo2bob2o25bo33bo$28b4o28b5o$29b\n
2o33bo$62bo$62b2o!\n
CB 2,2,2,3\n
L CM x=5 y=28 text=\"38P7\" color=(0,0,255)\n
L CM x=30 y=28 text=\"120P7\" color=(0,0,255)\n
L CM x=61 y=28 text=\"92P7\" color=(0,0,255)\n
L CM x=92 y=28 text=\"70P7\" color=(0,0,255)\n");
}

#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn read_new_p7_2009_03_16() {
        // read_file("NewP7-2009-03-16.rle");
    }
}
