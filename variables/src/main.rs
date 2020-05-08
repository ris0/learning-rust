fn main() {
    // variables are immutable by default
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // example of shadowing
    // let x = 5; // 5
    // let x = x + 1; // 6
    // let x = x * 2; // 12

    // shadowing construct allows us to change type of value while reusing the same name
    let spaces = "   ";
    let spaces = spaces.len()

    // mutability can't do this
    let mut spaces = "  ";
    spaces = spaces.len(); // run time error


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

shadowing is different from marking a variable as mut, because we'l lget a compile-time error if we accidentally try
to reassign to this variable without using the let keyword. By using let, we can perform a few transormations
on a value but have the variable be immutable after those transformations have been completed

the other difference between mut and shadowing is that because we're effectively creating a new variable when
we use the let keyword again, we can change the type of the value but reuse the same name.
*/
 