mod regex;

fn main() {
	let input = match std::env::args().nth(1) {
		None => {
			println!("Missing input regular expression.");
			return;
		}
		Some(str) => str
	};
	let regex = regex::parse(&input);
	println!("{}", regex);
}
