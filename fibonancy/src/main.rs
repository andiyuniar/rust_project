use std::io;

fn main() {
    println!("Masukkan angka:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");

    println!("Generate {} angka fibonancy",number);

    let number: u32 = number.trim().parse().expect("Invalid number");
       
    //let mut n:u32 = 0;
    // let mut fibo = 1;
    // let mut prev = 0;
    for x in 0..number {
        println!("{}", fibo(x));
    }
}

fn fibo(n: u32) -> u32 {
    if n <= 1 { 
        return n;
    }
    else {
        return fibo(n - 1) + fibo (n - 2);
    }
   
}
