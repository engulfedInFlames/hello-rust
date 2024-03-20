// cargo new project_name
// cargo build
// ./target/debug/project_name
// cargo run
// cargo build --release

// OWNERSHIP
fn main() {
    // 1. String type VS string literal
    // 2. Double free error
    // 3. Rust just invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    // Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
   let s1 = String::from("hello");
   let s2 = s1.clone(); // deep copy
   println!("{}, world!", s1);
   println!("{}, world!", s2);

}
