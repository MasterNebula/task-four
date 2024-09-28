use chrono::Local;

fn main(){
	let current_time = Local::now();
	println!("Hello Adithyan Shabu, right now the time is {}", current_time.format("%Y-%m-%d %H:%M:%S"));
}
