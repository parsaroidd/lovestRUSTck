use std::{
	fs::read_to_string, 
	io::{Write, stdout}, 
}; 

struct Animation
{
	name            : String, 
	height          : u16,
	width           : u16,
	frames          : u16, 
	OwnedPixelArray : Vec<(u16, u16)>, 
	LeadingPixel    : (u16, u16), 
}

impl Animation
{
	fn new(&mut self, name: String, LeadingPixel: (u16, u16), frames: u16) -> Self
	{
		let (h, w) = Self::get_height_and_width_with_pleasure(self); 
		Self
		{
			name, 
			height: h / self.frames, 
			width : w, 
			frames,
			OwnedPixelArray : vec![(0, 0)],
			LeadingPixel
		}
	}

	fn get_height_and_width_with_pleasure(&mut self) -> (u16, u16)
	{
		let mut length: Vec<usize> = Vec::new();
		let mut index: u16 = 0;

		for line in read_to_string(self.name.clone()).unwrap().lines()
		{
			length.push(line.len()); 
			index += 1; 
		}

		(index, *length.iter().max().unwrap() as u16) 
	}

	fn draw_once<T: Write>(&mut self, mut stream: T) -> () 
	{
		let mut index = 0;

		for line in read_to_string(self.name.clone()).unwrap().lines()
		{
			stream.write(line.as_bytes()); 
			stream.write("\n".as_bytes());
			let second = format!("{}", line.len()); 
			stream.write(second.as_bytes());
		}
	}
}

fn main() 
{
	let mut stream = stdout();

	let mut elahe = Animation 
	{
		name: "ball".to_string(), 
		height: 10,
		width: 10,
		frames: 3, 
		OwnedPixelArray: vec![(0, 0)], 
		LeadingPixel: (0, 0),
	};

	elahe = Animation::new(&mut elahe, "ball".to_string(), (0, 0), 3);
	println!("{:?}", elahe.height); 
}
