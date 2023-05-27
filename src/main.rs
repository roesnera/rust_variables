fn main() {
    // variables must be declared as mutable in order to be manipulated later
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // you can redeclare a variable of the same name as one that already exists
    // this is called shadowing, i.e. the former "string" is shadowed by the latter one
    // this can be useful when combined with scoping to manipulate some value without mutating the
    // more global instance

    let string = "   ";
    let string = string.len();
    println!("string value is: {string}");
    println!("string value is: {string}");

    // numbers can be signed or unsigned

    let unsigned_num: u32 = 13;
    let signed_num: i32 = 13;
    
    // five basic math operations also work
    
    let maths = signed_num + 10 -10 * 2 / 2 % 27;

    println!("maths value: {maths}");
    
    // default number type is i32
    // floating points are defaulted to f64, but can also be f32

    // tuples are considered a compound type
    // they are an ordered group of values with fixed length

    let tuple: (i32, bool, f64, u8) = (12345, true, 16.666, 120);

    // we can then access values in a tuple with the dot operator or through destructuring

    let (w, x, y, z) = tuple;
    println!("w is: {w}");
    let first = tuple.1;
    println!("the first element in tuple is: {first}");


    // there are also arrays
    // like tuples, arrays have fixed length
    // unlike tuples, arrays can only hold one value

    let my_array = [1, 2, 3, 4, 5];

    // arrays are useful when you want data allocated to the stack not the heap
    // vectors are like arrays but they can grow and shrink
    // you can also explicitly type the array and its elements on declaration

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // you can also intialized the array to a specified number of one value

    let other_array = [3; 4];

    // I wonder if you can do this

    let guess_array = [1..=10];
    
    // definitely compiles, let's see what happens when we access a value

    // let guess_array_index = guess_array[9];
    // this doesn't return a value but an inclusive rage, I'll have to look into this
    // println!("Guess array at index 9: {guess_array_index}");

    // accessing an array value uses the square bracket syntax like in other langs
    let my_array_three = my_array[2];
    println!("my array's 3rd element: {my_array_three}");

}

