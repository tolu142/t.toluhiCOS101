use std::io;

fn main() {
    let mut a:f32 = 0.0;
    let mut b:f32 = 0.0;
    let mut c:f32 = 0.0;
    
    let mut root_a:f32 = 0.0;
    let mut root_b:f32 = 0.0;

    let mut real:f32 = 0.0;
    let mut image:f32 = 0.0;
    let mut disc:f32  = 0.0;
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Enter A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    a = input1.trim().parse().expect("Not a valid number");

    println!("Enter B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    b = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    c = input3.trim().parse().expect("Not a valid number");

    if a == 0.0 || b == 0.0 || c == 0.0 
    {
        println!("Error: Unable to determine roots");
        return;
    }
    else 
    {
        disc = b * b - 4.0 * a * c;
        if disc < 0.0 
        {
            println!("Imaginary Roots");
            real = -b / (2.0 * a);
            disc = disc.abs();
            image = disc.sqrt() / (2.0 * a);
            println!("Root1 = {}  +i {}", real, image);
            println!("Root2 = {}  -i {}", real, image);
        }
        else if disc > 0.0 
        {
            println!("Roots are real and distinct");
            root_a = (-b + disc.sqrt()) / (2.0 * a);
            root_b = (-b - disc.sqrt()) / (2.0 * a);
            println!("Root1 = {}  ", root_a);
            println!("Root2 = {}  ", root_b);
        }
        else if disc == 0.0
        {
            println!("Roots are real and equal");
            root_a = -b / (2.0 * a);
            root_b = root_a;
            println!("Root1 = {}", root_a);
            println!("Root2 = {}", root_b);
        }
    }
}

