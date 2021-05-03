fn main() {
    let price: i32 = 10;
    println! {"Original price: {} ", price};
    println! {"Discounted price: {} ", calculate_price(price)};
}

fn calculate_price(price: i32) -> i32 {
    if price % 2 == 0 {
        return price - 10;
    } else {
        return price - 2;
    }
}
