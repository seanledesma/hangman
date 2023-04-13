fn main() {
	println!("Hello World");
	println!("I'm going to be so good at rust programming");
	let result = add(2, 2);
	println!("sum of numbers: {}", result);
}

fn add(num1: i32, num2: i32) -> i32 {
	num1 + num2
}
