struct laptop{
	brand:String;
	product_cost:u32,
	constant:u32
}

impl laptop{
	fn price(&self)->u32{
		self.product_cost * self.constant
	}
}

fn main() {
	let cost1 = laptop{
		brand:String::from("HP"),
		product_cost:650000,
		constant:3

	};

	let cost2 = laptop{
		brand:String::from("IBM"),
		product_cost:750000,
		constant:3
	};
    
    let cost3 = laptop{
		brand:String::from("Toshiba"),
		product_cost:550000,
		constant:3
	};

	let cost4 = laptop{
		brand:String::from("Dell"),
		product_cost:850000,
		constant:3
	};

	let total = cost1.price() + cost2.price() + cost3.price() + cost4.price();

	display(cost1);
	display(cost2);
	display(cost3);
	display(cost4);
	println!("The total cost of 3 purchases from each laptop brand is {}",total);	
}
    fn display(cost:laptop){
    	println!("The cost for 3 {} laptops is {}",cost.brand, cost.price());

    }