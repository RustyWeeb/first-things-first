fn main() {
    let x = 5; // immutable integer
    println!("The value of x is: {}", x);
    
    let mut y = 10; // mutable integer
    println!("The initial value of y is: {}", y);
    y = 20; // changing the value of y
    println!("The new value of y is: {}", y);
    
    let z: f64 = 3.14; // floating-point number
    println!("The value of z is: {}", z);
    
    let is_true: bool = true; // boolean value
    println!("The value of is_true is: {}", is_true);
    
    let c: char = 'R'; // character
    println!("The value of c is: {}", c);
    
    // Shadowing
    let a = 5;
    println!("The value of a is: {}", a);
    
    let a = a + 1; // shadowing the previous value of a
    println!("The shadowed value of a is: {}", a);
    
    {
        let a = a * 2; // shadowing within a new scope
        println!("The value of a in the inner scope is: {}", a);
    }
    
    println!("The value of a after the inner scope is: {}", a);
    
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring the tuple
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);
    println!("Accessing tuple elements directly: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr[0] is: {}", arr[0]);
    
    let arr2: [i32; 5] = [3; 5]; // array with all elements set to 3
    println!("The value of arr2 is: {:?}", arr2);
    
    // Slices
    let slice = &arr[1..3];
    println!("The slice is: {:?}", slice);
    
    // Strings
    let s1 = String::from("hello");
    let s2 = "world";
    let s3 = format!("{} {}", s1, s2);
    println!("Concatenated string is: {}", s3);
    
    // Functions
    println!("The result of add(2, 3) is: {}", add(2, 3));
    
    // Control flow
    let number = 6;
    
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
    
    // For loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
