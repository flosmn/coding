fn main() {
	let mut x0 = 1;
	let mut x1 = 1;
	let mut sum = 0;

	while x1 < 4000000 {
		if x1 % 2 == 0 {
			sum += x1;
		}
		let x2 = x0 + x1;
		x0 = x1;
		x1 = x2;
	}

	println!("sum: {}", sum);
}
