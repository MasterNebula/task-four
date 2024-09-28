use chrono::Local;

fn main(){
	let current_time = Local::now();
	println!("Hello Alika C, right now the time is {}", current_time.format("%Y-%m-%d %H:%M:%S"));
}
