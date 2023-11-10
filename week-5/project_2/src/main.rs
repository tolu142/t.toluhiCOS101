use std::io;

fn main() {
	let mut _worker:f32 = 0.0;
    let mut _experience:f32 = 0.0;

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    _worker = input1.trim().parse().expect("Not a valid number");

   
	if _worker >= 40.0
	{
	    println!("You are experienced and Incentive is 1_560_000");
    }
    else if _worker == 30.0 && _worker <= 40.0
    {
    	println!("You are experienced and Incentive is 1_480_000 ");
    }
    else if _worker < 28.0
    {
    	println!("You are experienced and Incentive is 1_300_000");
    }
    else
    {
    	println!("You are inexperienced and Incentive is 100_000");
    }
}