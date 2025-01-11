// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     // println!("The secret number is: {secret_number}");
    
//     loop{
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
        
//         let guess: u32 =match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guess}");
        
//         match guess.cmp(&secret_number) {
//           Ordering::Less => println!("Too small!"),
//           Ordering::Greater => println!("Too big!"),
//           Ordering::Equal => {
//             println!("You win!");
//             break;
//             }
//         }
//     }
// }


// fn hy(){
//     let mut x=5;
//     x=6;

// // }
// fn main(){
//     let _a=[1,2,3,4,5,6,7,8,9];
//     let _months=["january","february","march","april","may","june","july","august","september","october","november","december"];
//     let _xx:()=();
//   let tup=(2.1,2,2);
//   let _twopointone=tup.2;
//   let  _two=tup.1;
//   let sum  =_two + _twopointone;
//   print!("the sum of {_two} and {_twopointone} is {sum}.");
// }



use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}