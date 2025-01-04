fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);

    // Safe way to access elements
    if let Some(first) = vec.get(0) {
        println!("First element: {}", first);
    }
    if let Some(second) = vec.get(1) {
        println!("Second element: {}", second);
    }

    // Handling potential out-of-bounds access
    match vec.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("Index out of bounds"),
    }
} 