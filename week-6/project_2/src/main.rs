use std::io;

fn main() {

    for _ in 0..500{

	let name:String = String::new();
	let number:f32 = 0.0;

	let mut input1 = String::new();
    let mut input2 = String::new();

	 println!("Enter name of paper");
    io::stdin().read_line(&mut input1).expect("invalid input");

    println!("Enter number of papers published");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let number:f32 = input2.trim().parse().expect("Not a valid number");

    if number >=3.0 && number <= 5.0 
    {
     	 println!("Incentive is N500_000 ");
    } 

    else if number > 5.0 && number < 10.0 
    {
         println!("Incentive is N800_000 ");
    }

    else if number > 10.0  
    {
    	println!("Incentive is N1_000_000 ");
    }
    
    else if number < 3.0 
    {
    	println!("Incentive is N100_000 ");
    }

    println!("Name of paper published: {}", input1);

}
}

