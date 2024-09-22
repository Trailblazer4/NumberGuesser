use rand::Rng;
use std::io::{stdin, stdout, Write};

fn half(n: i32) -> i32 {
    return n / 2;
}

fn main() {
    let mut c: u8 = 65;
    c += 1;
    println!("{}", c as char);
    println!("{}", 'B' as u8);

    if 'B' as u8 == 66 {
        println!("As it turns out, we can reference integer values for chars");
    }


    let i: i32 = 5;
    {
        let mut i = i;
        i -= 1;
        println!("i: {i}");
    }
    println!("Half: {}", half(i));
    // let j = &mut i;
    // *j += 1;
    println!("Our int i: {}", i);

    let mut rng = rand::thread_rng();
    let answer: i32 = rng.gen_range(0..10); // rand num 0-9

    let mut rn = String::new();
    // let rn2 = &rn;
    print!("Guess a random number! ");
    stdin().read_line(&mut rn).expect("Failed to read line");

    let rn_int: i32 = rn.trim().parse().unwrap();
    println!("You typed: {}", rn_int);
    println!("Answer: {answer}");
    if rn_int == answer {
        println!("Well done!");
    }
}
