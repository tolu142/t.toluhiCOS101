use std::io::Write;

fn main() {
	let mut file = std::fs::File::create("Highqualityalcohol.txt").expect("create failed");
	file.write_all("Lager:(33 export, desperados, Goldberg, Gulder, Heineken, Star)\n"
		.as_bytes()).expect("write failed");
	file.write_all("\nStout:(Legend, Turbo king, Williams)".as_bytes()).expect("write failed");
	file.write_all("\nNon Alcoholic:(Maltina, Amstel malta, Malta Gold, Fayrouz)".as_bytes()).expect("write failed");
	println!("\nData written to file.");

}
	

