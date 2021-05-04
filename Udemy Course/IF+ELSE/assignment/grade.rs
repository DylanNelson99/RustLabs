use std::io;
fn main() {
	let mut percent=String::new();
	println!("Enter your percentage");
	io::stdin().read_line(&mut percent).expect("Fail");
	let percent:i32=percent.trim().parse().expect("Fail");
	if percent>=80 {
		println!("A Grade (GPA: 4.0)");
	}else if percent>=70  {
		println!("B+ Grade(GPA: 3.50)");
	}else if percent>=60  {
		println!("B Grade (GPA: 3.0)");
	}else if percent>=55  {
		println!("B- Grade (GPA: 2.75)");
	}else if percent>=50  {
		println!("C+ Grade (GPA: 2.5)");
	}else if percent>=40{
		println!("C Grade (GPA: 2.0)");
	}else if percent>=35 {
		println!("D Grade (GPA: 1.50)");
	}else {
		println!("Fail");
	
	}
}