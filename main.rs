use std::io;

fn main() {
	println!("Please enter your name");
	let mut input = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut input).ok();
	input.pop();

	let mut root = new_node(input);

	loop {
		println!("Next action please");
		let mut command = String::new();
		stdin.read_line(&mut command).ok();
		
		command.pop();
		let vec: Vec<&str> = command.split(' ').collect();

		if vec[0] == "quit" {
			println!("Good Bye");
			break;
		}
		else if vec[0] == "add" {
			if vec[2] == "mother" {
				let mut mama = new_node(String::from(vec[1]));
                        	
				add_node(&mut root, mama, &"mother".to_string(), &vec[3].to_string());
				match root.mother {
					Some(ref mut p) => {add_node(&mut *p, mama, &"mother".to_string(), &vec[3].to_string());},
					None => {},
				}
				match root.father {
                                        Some(ref mut p) => {add_node(&mut *p, mama, &"mother".to_string(), &vec[3].to_string());},
                                        None => {},
                                }
			}
			else if vec[2] == "father" {
				let mut papa = new_node(String::from(vec[1]));
				
				add_node(&mut root, papa, &"father".to_string(), &vec[3].to_string());
				match root.mother {
                                        Some(ref mut p) => {add_node(&mut *p, papa, &"father".to_string(), &vec[3].to_string());},
                                        None => {},
                                }
				match root.father {
					Some(ref mut p) => {add_node(&mut *p, papa, &"father".to_string(), &vec[3].to_string());},
					None => {},
                		}
			}
			else {
				println!("Invalid relationship");
			}			
		}
                else if vec[0] == "print" {
			print_node("".to_string(), &root);      
                }
		else if vec[0] == "delete" {
			println!("You want to delete {}?", vec[1]);
		}
		else {
			println!("Invalid command");
		}
	}
}

fn new_node(s: String) -> Node {
	Node {name: s,
		mother: None,
		father: None}
}

fn print_node(s: String, n: &Node) {
	println!("{}{}", s, n.name);
	match n.mother {
		Some(ref p) => {let new_s = format!("{}\t", s); print_node(new_s, p)},
		None => {},
	}
	match n.father {
		Some(ref p) => {let new_s = format!("{}\t", s); print_node(new_s, p)}, 
		None => {},
	}
}

fn add_node(root: &mut Node, parent: Node, relationship: &String, child: &String) -> bool {
	if root.name == *child {
		if *relationship == "mother" {
			match root.mother {
				Some(ref _p) => {println!("Relationship already exists"); true}, 
				None => {root.mother = Some(Box::new(parent)); true}, 
			}
		}
		else {
			match root.father {
				Some(ref _p) => {println!("Relationship already exists"); true},
				None => {root.father = Some(Box::new(parent)); true},
                        }
		}	
	}
	else {
		false
	}
}

struct Node {
	name: String,
	mother: Option<Box<Node>>,
	father: Option<Box<Node>>
}
