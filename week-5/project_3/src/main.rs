use std::io; 

fn main() {
	let p = "Poundo yam / Edinkaiko soup";
	let f = "Fried rice";
	let a = "Amala & Ewedu soup";
	let e = "Eba & egusi soup";
	let w = "white rice & stew";

	let mut input1 = String::new();

	println!("Menu: \n{} = N3200 \n{} = N3000 \n{} = N2500 \n{} = N2000 \n{} = N2500", p,f,a,e,w);

	println!("\nEnter price ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let price:f32 = input1.trim().parse().expect("Not a valid number");

    if price == 3200.0
    {
    	println!("Your order is: {}", p);
    }
    else if price == 3000.0
    {
    	println!("Your order is: {}", f);
    }
    else if price == 2500.0
    {
    	println!("Your order is: {} or {}", a,w);
    }
    else if price == 2000.0
    {
    	println!("Your order is: {}", e);
    }
    else if price >= 10000.0
    {
    	let discount = price - (0.05 * price);
    	println!("Discount: {}", discount);
    }
    else 
    {
    	println!("Insuffiecient Money");
    }
}
   