fn main() {
 for i in 1..10{ //outer loop
  println!("Multiplication Table of : {}", i);
   for j in 1..10 { // inner loop
       println!("{} * {} = {}", i, j, i * j);
   }
 }
}