use std::io;
fn main() {
	let mut ch=String::new();
	println!("Are your friends coming ?");
	io::stdin().read_line(&mut ch).expect("Failed");
	ch=ch.trim().to_string();
	if ch=="y" {
		println!("Yeah! Going for Movie");
	}else {
		println!("Stay at home");
	}

}