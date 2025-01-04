fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);
    let first = vec[0];
    println!("First element: {}", first);
    let second = vec[1];
    println!("Second element: {}", second);
    // Accessing element at index 2 will cause panic
    let third = vec[2];
    println!("Third element: {}", third);
}