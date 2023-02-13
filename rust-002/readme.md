# Day 002 - 2023-02-13

# Things learned
* Added a struct and printed it with println!
* println! need to arguments 
  * A format string with a new syntax {:?} 
  * What to print
* The compiler checks the amount of palceholders and the amout of values
* The struct needs an attribte ```#[Debug]``` to be pretty printed
* Rust has no null/nill so an Option has to be used for optional nested structs
  * Option seems to be in the default scopde and doesn't need a use statement
* Option is an Enumeration with the values None and Some
  * None has to be used when there is no value
  * Some(Struct{}) has to be used to use the value
* Printing the struct with an Optional gives you a None as value if there is no value
* Printing the struct with an Optional gives you Some(Struct ) 
* Unused attributes are printed as warning
* You need  a let in front of your variable
* The whole struct is immutable without the mut statement
* Rust distinguishes between string slices and strings
  * string slices are values, the live in the stack?
  * String (objects) are dynamic and can be convertet from a string slice with 
    * .to_string() or .to_owned() 
  * Cow is 'Copy on Write' and not some example with animals


# Todo's
* [Difference between .to_string() or .to_owned()](https://github.com/fratschi/100-days-of-rust/issues/1)
* [Why is there no documentation on e.g. to_string() ?](https://github.com/fratschi/100-days-of-rust/issues/2)
