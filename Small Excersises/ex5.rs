let vec = vec![1, 1, 2, 2];
println! {"{:?}", uniq_vect(&vec)};

fn uniq_vect(vec: &Vec<i32>) -> Vec<i32> {
    // Empty vector to store unique values
    let mut uniq = vec![];

    for d in vec.iter() {
        if !uniq.contains(d) {
            uniq.push(*d);
        }
    }
    return uniq;
}
