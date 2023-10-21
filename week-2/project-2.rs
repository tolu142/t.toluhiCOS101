fn main() {
   let t:f64 = 450000.0;
   let m:f64 = 1500000.0;
   let h:f64 = 750000.0;
   let d:f64 = 2850000.0;
   let a:f64 = 250000.0;

   let tq:f64 = 2.0;
   let mq:f64 = 1.0;
   let hq:f64 = 3.0;
   let dq:f64 = 3.0;
   let aq:f64 = 1.0;

   // sum 
   let s = ( t * tq ) + ( m * mq ) + ( h * hq ) + ( d * dq ) + ( a + aq );
   println!("Sum is {}", s);

   // quantity 
   let q = tq + mq + hq + dq + aq; 
   println!("Quantity is {}", q);

   // average
   let a = s / q; 
   println!("Average is {}", a);

}