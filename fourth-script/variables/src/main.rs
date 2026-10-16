fn main() {
    println!("Hello, world!");

    // Immutable by default.
    let _x = 5;
    let mut x = 5;

    println!("x is {x}");

    x = 6;
    println!("x is {x}");


    // Can never be mutable.
    // Conveys that this is used throughout the code and locked.
    const THREE_HOURS: u32 = 60 * 60 * 3;



    let y = 5;

    // Shadows the previous y.
    let y = y + 1;

    {
        // Shadows the previous y again, but only within it's scope.
        // Inner shadowing.
        let y = y * 2;

        println!("y in the inner scope is {y}");
    }

    println!("y in the outer scope is {y}");

    // Changing the type of spaces.
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces is {}", spaces);


    let _guess: u8 = "42".parse().expect("Should be a number!");

    let float_num: f32 = 3.0;
    println!("Float is {float_num}");

    let maths: f32 = 5.0;
    println!("Maths is {}", maths);
    let maths = 5 / 4;
    println!("Maths is {}", maths);
    let maths = 1532 / 411;
    println!("Maths is {}", maths);

    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _) = my_tuple;
    println!("Tuples here! {}", y);

    let first = my_tuple.0;
    println!("First is {first}");
}
