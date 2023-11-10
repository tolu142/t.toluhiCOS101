use std::io;

fn main() {
	let mut _worker:f32 = 0.0;
    let mut _experience:f32 = 0.0;

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    _worker = input1.trim().parse().expect("Not a valid number");

    println!("Enter experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    _experience = input2.trim().parse().expect("Not a valid number");

	if _worker >= 40.0
	{
	    println!("Incentive is 1_560_000");
    }
    if _worker == 30.0
    {
    	println!("Incentive is 1_480_000 ");
    }
    if _worker < 28.0
    {
    	println!("Incentive is 1_300_000");
    }
}