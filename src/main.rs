use std::{env::{self, join_paths}, path::Path, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let project_dir = env::current_dir().unwrap();

	match args[1].as_str() {
		"init" => { initialize_repository(&project_dir) }
		_ => { panic!("Invalid command! List commands with rvc --help") }
	}
}

fn initialize_repository(project_path: &Path) {
	let rvc_path = project_path.join(".rvc"); // Create a path of project_path + .rvc        
	fs::create_dir(rvc_path.clone()).unwrap(); // Create the .rvc path
	fs::create_dir(rvc_path.join("objs")).unwrap(); // Create .rvc/objs     
	fs::create_dir(rvc_path.join("ptrs")).unwrap(); // Create .rvc/ptrs 
	
	println!("Initialized empty Rvc repository at {}", project_path.to_string_lossy());
}