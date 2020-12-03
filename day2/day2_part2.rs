use std::fs;

fn main() {
	let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let mut total_valid = 0;

    for password in contents.split("\n") {
    	if password != "" {
    		let info_content_vec = split_to_vec(password.to_string(), ":");
    		let (first_pos, second_pos, character) = get_password_info(info_content_vec[0].to_string());
            let password_as_vec = info_content_vec[1].chars().collect::<Vec<_>>();

            // No need for input offset, as the vector starts with a space in the first position
    		if (password_as_vec[first_pos] == character && password_as_vec[second_pos] != character) ||
               (password_as_vec[first_pos] != character && password_as_vec[second_pos] == character) {
    			total_valid += 1;
            }
    	}

    }

    println!("{:?}", total_valid);

}

fn split_to_vec(input: String, split_char: &str) -> Vec<String> {
	return input.split(split_char).map(|s| s.to_string()).collect();
}

fn get_password_info(info: String) -> (usize, usize, char) {
	let first_pos: usize;
	let second_pos: usize;
	let character: char;

	let mut range_and_char : Vec<String> = split_to_vec(info, " ");
	let range : Vec<String> = split_to_vec(range_and_char.remove(0), "-");
	first_pos = range[0].parse::<usize>().unwrap();
	second_pos = range[1].parse::<usize>().unwrap();
	character = range_and_char.remove(0).chars().nth(0).unwrap(); // is this really the best way to do it...?


	return (first_pos,second_pos,character);
}