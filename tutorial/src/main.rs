fn main() {
    //println!("Hello, world!");
    //another_function(12, 'm');
    //println!("{} + {} = {}", 12, 20, addition(12, 20) );
    //println!("{} * {} = {}", 12, 20, multiplication(12, 20));
    //work_with_loop();

    return_value_from_loop();
}

fn another_function(value: i32, unit_label: char) {
    println!("The value of x is {} {}", value, unit_label);
}

fn addition(val1: i32, val2: i32) -> i32 {
    val1 + val2
}

fn multiplication(val1: i32, val2: i32) -> i32 {
    return val1 * val2;
}

fn work_with_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count : {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn return_value_from_loop() {
    let mut counter = 0;
    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result {}", result);
}
