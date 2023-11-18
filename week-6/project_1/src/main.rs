use std::io;
fn main() {

	for _ in 0..500{

	let mut name:String = String::new();
	let mut department:String = String::new();
	let mut email:String = String::new();
	let mut state_of_origin:String = String::new();
	let mut class_reps:String = String::new();
	let mut cgpa_level:String = String::new();
	let mut fresher:String = String::new();

	println!("Enter name");
	io::stdin().read_line(&mut name).expect("invalid input");
	
	println!("Enter department");
	io::stdin().read_line(&mut department).expect("invalid input");

	println!("Enter email");
	io::stdin().read_line(&mut email).expect("invalid input");

	println!("Enter state of birth");
	io::stdin().read_line(&mut state_of_origin).expect("invalid input");

	println!("Are you a class representative?");
	io::stdin().read_line(&mut class_reps).expect("invalid input.Please answer yes/no");
	let class_reps:bool = class_reps.to_lowercase().trim() == "yes";
	if !class_reps {
		println!("You are elligible to vote");
		return
	}

	println!("is your cgpa above 4.0?");
	io::stdin().read_line(&mut cgpa_level).expect("invalid input.Please answer yes/no");
	let cgpa_level:bool = cgpa_level.to_lowercase().trim() == "yes";
	if !cgpa_level {
		println!("You are elligible to vote");
		return
	}

	println!("Are you above 100 level?");
	io::stdin().read_line(&mut fresher).expect("invalid input.Please answer yes/no");
	let fresher:bool = fresher.to_lowercase().trim() == "yes";
	if !fresher {
		println!("You are elligible to vote");
		return
	}

	println!("Your name is {}", name);
	println!("Your state of origin is {}", state_of_origin);
	println!("Your department is {}", department);
	println!("Your email is {}", email);
	
}
}
