pub use crossterm::{
	terminal::{self, Clear, ClearType},
	event::{Event, read, KeyCode}, 
	cursor::MoveTo,
	execute,
};
pub use std::{
	io::{self, Write, stdout},
	thread, 
	time
};

const MARGIN: u16 = 10;

#[derive(Debug)]
struct Screen
{
	width  : u16,
	height : u16,
	left   : u16,
	right  : u16,
	charc  : char,
	range  : u16
}
impl Screen
{
	pub fn new() -> Self
	{
		let Ok((width, height)) = terminal::size() else { todo!() };

		Self {
			width  : width,
			height : height,
			left   : width / 10,
			right  : width / 2 - width / 10,
			charc  : 'P',
			range  : height / 5,
		}

	}

	pub fn draw_angle(&mut self, stream: &mut std::io::Stdout, offset_x: u16, offset_y: u16) -> (u16, u16)
	{
		execute!(stream, MoveTo(0, offset_y)); /*for drawing the width, we need height!*/
		stream.write("━".repeat(self.width as usize).as_bytes());

		for cell in 0..self.height
		{
			execute!(stream, MoveTo(offset_x, cell)); 
			stream.write("┃".as_bytes()); 
		}

		stream.flush().unwrap();
		return (10, 10);
	}

	pub fn draw_margin(&mut self, stream: &mut std::io::Stdout)
	{
		let base = self.height / 2;
		for cell in 0..=base
		{
			execute!(stream, MoveTo(0, cell));

			if cell == 3
			{
				stream.write("▒".repeat(self.width as usize).as_bytes());
			}

			stream.write("▒".repeat((self.height - cell - MARGIN) as usize).as_bytes());

			execute!(stream, MoveTo(self.width - (self.height - cell - MARGIN), cell));

			//execute!(stream, MoveTo(self.width - (self.height - MARGIN + cell), cell));
			stream.write("▒".repeat((self.height - cell - MARGIN) as usize).as_bytes());
		}

		for cell in base..=self.height
		{

			execute!(stream, MoveTo(self.width - 1, cell));

			if cell == self.height
			{
				stream.write("▒".repeat((self.width) as usize).as_bytes());
			}
			stream.write("▒".repeat((self.height - (self.height - cell) + 1 - MARGIN) as usize).as_bytes());

			/*You must subtract the value that you repeated from window width to get to a proper place 
			 * in the right side of screen */

			execute!(stream, MoveTo(self.width - (self.height - (self.height - cell) + 1 - MARGIN), cell));
			stream.write("▒".repeat((self.height - (self.height - cell) + 1 - MARGIN) as usize).as_bytes());

		}
		stream.flush().unwrap();
	}
}


fn main()
{
	let mut stream = stdout();

	let mut first: Screen = Screen::new();
	let (mut offx, mut offy) = (0, 0);
	let _ = terminal::enable_raw_mode();
	loop
	{
		match read().unwrap()
		{
			Event::Key(event) =>
			{
				match event.code
				{
					KeyCode::Esc => 
					{
						break;
					},
					_ => {},
				}
			}, 
			_ => {}, 
		}
		thread::sleep(time::Duration::from_millis(330));
		execute!(stream, Clear(ClearType::Purge), Clear(ClearType::All));
		offx += 1; offy += 1;
		first.draw_angle(&mut stream, offx, offy);
		first.draw_margin(&mut stream);

	}
}
