use std::io::Read;
use std::io;
struct BfProg {
	cells: Box<[u8; 2000]>
}
impl BfProg {
	fn run(&mut self, prog: &mut String, input: &String) {
		let mut c: char = 'a';
		let mut inp = input.chars();
		let mut get_next_input = move || {
			if let Some(x) = inp.next() {
				c = x;
				x
			}
			else {
				c
			}
		};
		let mut cell_index: usize = self.cells.len() / 2;
		let mut i = 0;
		let p = prog.as_mut_ptr();
		let mut loop_stack = Vec::<usize>::new();
		while i < prog.len() {
			unsafe {
				let ptr = p.add(i);
				match *ptr as char {
					'+' => self.cells[cell_index] += 1,
					'-' => self.cells[cell_index] -= 1,
					'>' => cell_index += 1,
					'<' => cell_index -= 1,
					'[' => loop_stack.push(i),
					']' => if self.cells[cell_index] == 0 {loop_stack.pop();} else {i = loop_stack[loop_stack.len()-1]},
					'.' => print!("{}", self.cells[cell_index] as char),
					',' => self.cells[cell_index] = get_next_input() as u8,
					_ => ()
				}
			}
			i += 1;
		}
	}
}
fn main() {
	let mut program: String = String::new();
	for i in std::env::args() {
		program += &i;
	}
	let stdin = io::stdin();
	let mut lock = stdin.lock();
	let mut input: String = String::new();
	lock.read_to_string(&mut input).unwrap();
	
	if !check_syntax(program.as_str()) {
		return;
	}
	let mut prog = BfProg {cells: Box::new([0;2000])};
	prog.run(&mut program, &input);
	
	
	return;
}
fn check_syntax(program: &str) -> bool {
	if !program.is_ascii() {
		println!("Program must be ascii");
		return false;
	}
	let mut obc: u64 = 0;
	let mut cbc: u64 = 0;
	for i in program.chars() {
		match i {
			'[' => obc += 1,
			']' => cbc += 1,
			_ => ()
		}
	}
	if obc != cbc {
		println!("Syntax error. Mismatched brackets. Expected {} closing brackets found {} closing brackets", obc, cbc);
		return false;
	}
	true
}
