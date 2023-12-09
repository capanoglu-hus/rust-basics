fn main() {
    println!("Hello, world!");

    let a = 4;
    let mut b = 8;
    b = 9;
    println!("immutable a: {}" ,a);
    println!("mutable b: {}",b);
    
   let pos: u32 = 50;
   let neg: i64 = -45;
   println!("pos: {} , neg:{}",pos,neg);

   let active:bool = true;
   println!("active - {} ",active);

   let character:char  = 'd';
   println!("karakter - {} ",character);

   let karisik:(u32,i32,f32)=(9,-5,3.1);
   let (x, y, z) = karisik; 
   println!("karisik - 1 - {} 2 - {} 3 -{} ",x,y,z);
   let my= [6,8];
   println!("array {} ",my[0]);

   let area =calculate_area(5,4);
   println!("area {} ",area);

   let number = 91;
   if number % 2 == 0  {
    println!("{} is even" ,number);
   } else {
    println!("{} is odd" ,number);
   }

   let mut sayac = 0 ;
   loop{
    sayac+=1;
    if sayac ==7{
        break;
    }
   } 

   println!("{}" ,sayac);

   let mut  rust=String::from("rust!");

   rust.push_str(" fun");
   println!("{}" ,rust);

   let referans = calculate(&rust) ;
   println!("{}" ,referans);

   let referans_mut =calculates(&rust);
   //println!("{}",referans_mut);
}

fn calculate_area(x:i32 ,y:i32) -> i32 {
    x*y
   }

fn calculate(rust:&String) -> usize {
    rust.len()
}

fn calculates(rust:&String) {
 let  rust=String::from("deneme! 123 ");
 println!("{} !! ",rust);
}