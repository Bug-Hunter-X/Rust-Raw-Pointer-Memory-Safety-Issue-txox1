fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    // Safe way to modify the vector
    v[index] = 10;
    println!("v: {:?}", v);
} 