// use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let m = &mut guess;

    // r is Result type, enum, Ok or Err
    let _r = std::io::stdin().read_line(m);
    // & indicate that this arg is a ref
    // .expect("Failed to read line");

    // _r.expect("Failed to read line");
    //
    // let x = 5;
    // let y = 10;

    // println!("x = {x} and y + 2 = {}", y + 2);

    println!("You guessed: {guess}");
}

// reference give u a way to let multiple parts of ur code access one piece of data without needing
// needing to copy that data into mem multiple times.
// ref are immutable by default
//
// a crate is a collection of rust source code files. this proj is a binary crate, which is a
// executable. the rand crate is a library crate, which contains its code that is intended to be
// used in another programs and can't be executed on its own
//
// registry, copy of data from crate.io is where ppl in the rust eco post their open source rust
// projects for others to use.
