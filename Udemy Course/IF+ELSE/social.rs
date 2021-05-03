use std::io;
fn main () {
	let mut name=String::new();
	let mut age=String::new();
	let mut ch=String::new();
	
	println!("Enter Name and Age");
	io::stdin().read_line(&mut name).expect("Failed");
	io::stdin().read_line(&mut age).expect("Failed");
	let age:i32=age.trim().parse().expect("Failed");
	
	println!("Do you want to create account");
	io::stdin().read_line(&mut ch).expect("Failed");
	ch=ch.trim().to_string();
	if ch=="y" {
		if age<10 {
			println!("Your age is less");
		}else {
			println!("Your Account is created");
			println!("Do you want to upload photo");
			ch.clear();
			io::stdin().read_line(&mut ch).expect("Failed");
			ch=ch.trim().to_string();
			println!("ch={}",ch);
			
			if ch=="y" {
				if age<13 {
					println!("You cannot upload photo");
				}else {
					println!("You can upload your photo");
				}
				
			}else {
				println!("Thanks for Visiting");
			}
			
			
		}
	
	
	}
	
	
	
	
	
	
	
	
	
	
	

	
	
}