fn main() {
    let num: i32 = 6;
    println! {"The power of {} is -> {}", num, square_num(num)};
}

fn square_num(num: i32) -> i32 {
    num * num
}
