use std::fs;

fn main() {
	let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let mut total_valid = 0;

    for password in contents.split("\n") {
    	if password != "" {
    		let info_content_vec = split_to_vec(password.to_string(), ":");
    		let (min, max, character) = get_password_info(info_content_vec[0].to_string());
    		let character_count = info_content_vec[1].matches(&character).count();
    		if character_count >= min && character_count <= max {
    			total_valid += 1;
    		}
    	}

    }

    println!("{:?}", total_valid);


    // println!("{:?}", info_content_vec);
}

fn split_to_vec(input: String, split_char: &str) -> Vec<String> {
	return input.split(split_char).map(|s| s.to_string()).collect();
}

fn get_password_info(info: String) -> (usize, usize, String) {
	let min: usize;
	let max: usize;
	let character: String;

	let mut range_and_char : Vec<String> = split_to_vec(info, " ");
	let range : Vec<String> = split_to_vec(range_and_char.remove(0), "-");
	min = range[0].parse::<usize>().unwrap();
	max = range[1].parse::<usize>().unwrap();
	character = range_and_char.remove(0);


	return (min,max,character);
}