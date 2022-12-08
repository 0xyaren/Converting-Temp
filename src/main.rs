/************************* Convert temps between Fahrenheit and Celsius *************************/
use std::io;

fn main(){
    loop {
        println!("Select the conversion type:");
        println!("1. F -> C");
        println!("2. C -> F");

        let mut con_type = String::new();

        io::stdin().read_line(&mut con_type)
            .expect("Failed to read line");

        let con_type = con_type.trim();
        let con_type = match con_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Input the temp:");
        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid temp");
                continue;
            }
        };

        let converted_temp = 
        
        if con_type == 1{
            (5.0/9.0)*(temp - 32.0) 
        } 
        
        else {
            ((5.0/9.0)*temp) + 32.0
        };

        println!("The converted temp is {}", converted_temp);
    }
}