use std::io;
fn main() {

   control_struct(4, 8 );
    
}

fn control_struct(x: i8, y: i8) {
    if x > y {
        println!("true");
    }else {
        println!("false");
    }
}

//return functions
// fn five()   -> i128 {
//     5
// }


// fn another_function(x: f64) {
//     println!("The value of x is: {x}");
// }

// fn com() {
//     let tup: (i32, f64, u8) = (-5, 0.6, 9);
//     let v1 = tup.0;
//     let v2 = tup.1;
//     let v3 = tup.2;
//     println!("the tuple values are {}, {}, {}", v1, v2, v3);

//     let a = [5; 6];
//     let b = a[0];
//     println!("the array value at index 0 is {}", b);

//     let a = [1, 2, 3, 4, 5];
//     println!("Please enter an array index.");
//     let mut index = String::new();
//     io::stdin()
//     .read_line(&mut index)
//     .expect("Failed to read line");
//     let index: usize = index
//     .trim()
//     .parse()
//     .expect("Index entered was not a number");
//     let element = a[index];
//     println!("The value of the element at index {index} is: {element}");

//     another_function(78e2);
// }