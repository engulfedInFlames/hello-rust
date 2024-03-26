// cargo new project_name
// cargo build
// ./target/debug/project_name
// cargo run
// cargo build --release

// 4.2 References and Borrowing
// Keywords: Pointer Safety Principle, Immutable Reference, Mutable Reference, Dangling Reference
fn main() {
    ////////// References //////////
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    // greet(m1, m2);
    // let s = format!("{} {}", m1, m2); // Error💣: m1 and m2 are moved

    greet_by_ref(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{}", s);

    let mut r1:Box<i32> = Box::new(10);
    let v1:i32 = *r1;
    *r1 += 1;

    let r2:&Box<i32> = &r1;
    let v2:i32 = **r2;

    let r3:&i32 = &*r1;
    let v3:i32 = *r3;

    println!("v1: {}, v2: {}, v3: {}", v1, v2, v3);

    // fn greet(g1: String, g2: String) {
    //     println!("{} {}!", g1, g2);
    // }

    fn greet_by_ref(g1:&String, g2:&String) {
        println!("{} {}!",g1, g2);
    }


    ////////// Borrowing //////////
    // ❗Pointer Safety Principle: data should never be aliased and mutated at the same time.
    // Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated at the same time.

    let mut vec1: Vec<i32> = vec![1,2,3];
    let r4:&i32 = &vec1[2];

    // vec1.push(4); // Error💣
    println!("The third element is {}", r4);

    ////////// Mutable Reference //////////
    
    fn ascii_capitalize(v: &mut Vec<char>) {
        let c = &v[0];
        if c.is_ascii_lowercase() {
            let up = c.to_ascii_uppercase();
            v[0] = up;
        } else {
            println!("Already capitalized: {:?}", v);
        }
    }
    

}

// OWNERSHIP
// 4.1 What is Ownership?
// Keywords: Ownership, Stack, Heap, Copy, Move, Clone
/*
fn main() {
    // 1. String type VS string literal
    // 2. Double free error
    // 3. Rust just invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    // Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
   
   // Example 1
   let s1 = String::from("hello");
   let s2 = s1.clone(); // deep copy
   println!("{}, world!", s1);
   println!("{}, world!", s2);

   // Example 2
   // As a general rule, any group of simple scalar values can implement `Copy`.
   let x = 5;
   let y = x; // In this case, copy  (not move)

   println!("x = {}, y = {}", x, y);
}
*/

