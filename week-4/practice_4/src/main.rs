fn main(){

	let fullname = "Chidubem John Umeh";
	let department = "Computer Science";
	let uni = "Pan-Atlantic University";


	let mut school = "School of Science".to_string();
	// push string 
	school.push_str("  and technology");

	println!("My name is: {}", fullname);
	//checl length
	println!("The length my fullname is {}",fullname.len());
	println!("I am a student of {} Department", department);
	println!("{}",school);
    println!("{}",uni);
}
