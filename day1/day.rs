use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    for number1 in contents.split("\n") {
    	if number1 != "" {
	    	let int_number1 = number1.parse::<i32>().unwrap();
    	    for number2 in contents.split("\n") {
		    	if number2 != "" {
			    	let int_number2 = number2.parse::<i32>().unwrap();
				    for number3 in contents.split("\n") {
				    	if number3 != "" {
					    	let int_number3 = number3.parse::<i32>().unwrap();
					 		if int_number1 + int_number2 + int_number3 == 2020 {
					    		println!("{:?}", int_number1 * int_number2 * int_number3);
					    		return;
					    	}   	
					    }
				    }
			    }
		    }
	    }
    }


}