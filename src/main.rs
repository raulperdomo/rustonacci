use std::io;

fn main() {
    println!("What is the index of the fibonacci number you would like? ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read the line.");

    let index: u32 = index.trim().parse().expect("Failed to parse an integer.");

    println!("The fibonacci number at index {index} is {}", calc_nth_fibonacci_value(index));
}

fn calc_nth_fibonacci_value(n: u32) -> u128 {
    let mut x = 1;
    let mut y = 1;
    //println!("{x}");
    //println!("{y}");


    if n == 1 {
        x
    } else if n == 2 {
        y
    } else {
        let mut index = 3;
        while index <= n {

            let current_value = x + y;
            x = y;
            y = current_value;

            //println!("{current_value}");
            index += 1;
        }
        //return current value in y
        y
    }

}