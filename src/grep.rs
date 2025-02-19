pub fn our_grep(args: &Vec<&str>) -> (){
	
	use std::fs::File;
	use std::string::String;
	use std::io::{BufRead, BufReader};
	
	let mut len = args.len() - 1;
	if len == 0 {
		println!("Error: No input");
		return
	}
	if len==1 {
		let find = args[1];
		while true {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();
			let input = input.trim();
			if input == "EXIT" {
				break;
			}
			if input.contains(&find) {
				println!("{}",input);
			}
		}
	}
	else if len > 1 {
		let find = args[1];
		let mut string = args[1];
		for x in 1..len {
			let filename = args[x+1].trim();
			let file = match File::open(filename) {
				Ok(file) => file,
				Err(e) => { 
						println!("{}: file doesn't exist",filename);
						return
					  }
			};	
			let read = BufReader::new(file);

			for (index,line) in read.lines().enumerate(){
				let line = line.unwrap();			
				if line.contains(&args[1]){
					println!("{}:	{}",filename,line);
				}
			}

		}
		
	}
}


