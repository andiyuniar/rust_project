#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
//create method in struct (method/function dalam class)
impl Rectangle {
    //method without parameter
    fn luas_area(&self) -> u32 {
        self.width * self.height
    }

    //method with more parameters
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    //assosiate function that dont have &self
    //it is lke constructor that return new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size
        }
        
    }
}

struct UserName {
    first_name: String,
    last_name: String
}

impl UserName {
    fn full_name(&self) -> String {
        let mut full: String = self.first_name.to_owned();
        let last: &str = &self.last_name;
        full.push(' ');
        full.push_str(last);
        return full;
    }
}
//-------------
//ENUM

enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}
//-------------


fn main() {
    let width = 30;
    let height = 40;
    println!("Hitung Luas with function {}", luas(width, height));
    
    let rect = (20, 30);
    println!("Hitung luas with tupple {}", luas_with_tupple(rect));

    let square = Rectangle {
        width: 10,
        height: 10,
    };
    println!("Hitung luas with struct {}", luas_with_struct(&square));

    //hitung luas dengan  method yg didefinisikan dalam struct
    let square2 = Rectangle {
        width: 12,
        height: 10
    };
    println!("Hitung luas with method {}", square2.luas_area());

    //create new instance of squere
    let bujursangkar = Rectangle::square(2);
    println!("New instance of square rectangle {:#?}", bujursangkar);

    let my_name = UserName {
        first_name: String::from("andi"),
        last_name: String::from("yuniarto")
    };
    println!("My full name is {}", my_name.full_name());

    //call enum
    let m = Message::Write(String::from("hello world"));
    m.call()

}

fn luas(width: u32, height: u32) -> u32 {
    width * height
}

fn luas_with_tupple(dimension:(u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn luas_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}