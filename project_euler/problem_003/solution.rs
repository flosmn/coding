fn is_prime(x: i64) -> bool {
	if x < 2 { return false; }
	for i in (2..(x/2+1)) {
		if x % i == 0 { return false; }
	}
	true
}

fn main() {
	let mut number = 600851475143_i64;
	for i in (0..number) {
		if is_prime(i) && (number % i == 0) {
			if i == number {
				println!("{}", i);
				return;
			}
			number = number / i;
		}
	}
}
