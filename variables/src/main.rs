fn main() {
    // variables are immutable by default
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // example of shadowing
}

/**
There are multiple trade-offs to consider in addition to the prevention of bugs. 
For example, in cases where you're using large data structures, 
mutating an instance in place may be faster than copying and returning newly allocated instances. 

With smaller data structures, creating new nisntances and writing in a more functional programming style
may be easier to think through, so performance might a worthwile penalty for gaining that clarity
**/

/*
constants, like immutable variables, are values that are bound to a name and are not allowed to change, but there a few differences between

You aren't allowed to use mut with constants. They're not immutable by default.. they are always immutable

You can declare constants using the const keywords instead of the ley keyword
Type value must be annotated
const MAX_POINTS: u32 = 100_000;

constants can be declared in any scope, including global.

last difference is that constants may be set only to a constant expression,
not the result of a function call or any other value that could be computed at run time

// Rust's naming convention for constants is to use all uppercase with underscores btwn words

constants are valid for the netire time a program runs, within the scope they were declared in
*/
 