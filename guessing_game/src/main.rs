use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Menebak Angka");

    //let mut rng = rand::thread_rng();
    //let secret_number = rng.gen_range(1..101);

    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("Angka rahasia adalah: {}", secret_number);

    loop {
        println!("Masukkan angka:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Gagal membaca input");

        //let guess: u32 = guess.trim().parse().expect("Invalid number or NOT a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Anda menebak: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Anda BENAR!!!");
                break;
            }
        }
    }
}
