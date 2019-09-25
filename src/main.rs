fn start_process(args: &Vec<&str>) -> () {
	use std::process;
	use std::panic;

    //This suppresses the panic verbose remove if required.
	panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

	let result = panic::catch_unwind(|| {
    	let mut child = process::Command::new(args[0])
    							.args(args)
    							.spawn()
    							.expect("command failed to start");
    	let ecode = child.wait()
    				.expect("failed to wait on child");
        assert!(ecode.success());
        });

	if result.is_err(){
		println!("'{}' is not recognized as an internal or extrernal command,\n or operable program", args[0]);
	}
}

fn our_exit(args: &Vec<&str>) -> () {
	use std::process;
	if args.len() == 1{
		process::exit(0);
	}
	else {
		let code = args[1].parse().unwrap();
		process::exit(code);
	}
}



fn our_cwd(args: &Vec<&str>)-> std::io::Result<()> {
	use std::env;
    let path = env::current_dir()?;
    println!("{}", path.display());
    //process::exit(code);
    Ok(())
}

fn our_cd(args: &Vec<&str>) -> () {
	//use std::process;
	//use std::env;
	let mut pp = String::from("");
	
	if args.len() == 1{
		pp.push_str("# ");
	}
	if args.len() == 2{
		pp.push_str("# ");
		//let code = args[1].parse().unwrap();
		pp.push_str(args[1]);
		//match env::set_current_dir(&code) 
		}
	
}

fn run_internal(args: &Vec<&str>) -> () {
	if args[0] == "exit" {
		our_exit(&args)
	}
	if args[0] == "cd" {
		our_cd(&args)
	}
	if args[0] == "cwd" {
		our_cwd(&args);
	}
	if args[0] == "echo" {
		our_echo(&args);
	}
}

fn set_prompt(args: &Vec<&str>) -> String {
	let mut pp = String::from("");
	if args.len() == 1 {
		pp.push_str("# ");
	}
	else {
		for i in 1..args.len() {
			pp.push_str(args[i]);
			pp.push_str(" ");
		}
	}
	pp
}


fn our_echo(args: &Vec<&str>) -> () {
	let mut pp = String::from("");
	if args.len() > 1{
		pp.push_str(args[1]);
	}
	for i in 2..args.len() {
		pp.push_str(" ");
		pp.push_str(args[i]);
	}
	println!("{} ", pp);
}


fn main() {
	use std::io::{stdin,stdout,Write};
	let mut prompt = String::from("# ");
	let mut internal_commands = Vec::new();
	
	internal_commands.push("exit");
	internal_commands.push("cd");
	internal_commands.push("cwd");
	internal_commands.push("echo");
	loop {
		let mut command = String::from("");
		print!("{}",prompt);
		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Invalid command");
		command = String::from(command.trim());
		let split = command.split(" ");
		let args = split.collect::<Vec<&str>>();

		let mut internal = false;

		if args[0] == "" {continue;}
		if args[0] == "prompt"{
			prompt = set_prompt(&args).clone();
			continue;
		}
		for com in &internal_commands{
			if args[0] == *com {
				internal = true;
				run_internal(&args);
				break;
			}
		}
		if internal == false {
			start_process(&args)
		}
	}

}
