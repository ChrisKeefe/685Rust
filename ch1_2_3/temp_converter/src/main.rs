use std::io;

fn main() {
    // get conversion direction
    println!("Convert from:");
    println!("\t1: F to C");
    println!("\t2: C to F");
    println!();

    let direction = loop {
        println!("Please enter the number corresponding to your preferred conversion:");
        let mut dir_in = String::new();

        io::stdin()
            .read_line(&mut dir_in)
            .expect("Err: Read failure");
        
        let dir_in: u8 = match dir_in.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if (dir_in == 1) || (dir_in ==2) {
            break dir_in;
        }
    };
    
    // get temperature
    let temp = loop {
        println!("Enter the temperature you would like to convert:");
        let mut from_temp = String::new();

        io::stdin()
            .read_line(&mut from_temp)
            .expect("Err: Read failure");
        
        let from_temp: f64 = match from_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break from_temp
    };
    
    // calculate result, print, and exit
    if direction == 1 {
        println!("The temperature in Celsius is {}", ftoc(temp));
    }
    else {
        println!("The temperature in Fahrenheit is {}", ctof(temp));
    }
}

fn ftoc(temp_f:f64)->f64 {
    (temp_f - 32.) / 9. * 5.
}

fn ctof(temp_c:f64)->f64 {
    (temp_c * 9. / 5.) + 32.
}