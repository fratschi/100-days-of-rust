# Day 002 - 2023-02-15

# Things learned
* Returning a reference of a struct does not work
* String is allocated on the heap when Strings:from(...) is used
* &str is a string slice
* Using str as argument of a function does not work. Use &sr instead...
* println! takes a reference or the variable as argument
* Need to understand the borrow checking better
  * This works, even when its repeatet ```    println!("{:?}", p);```
  * This does not work ```println!("Is {} older than 18? {}", p.name, check_adult(p));``
  * p.name in combination with check_adult(&p) works, but not when you flip the ampersands
* You can return multipe values as tuple... thats nice
* Tuples can be access with .0 and .1 ... not so nice
* Tuples can be deconstructed... nice ```let (s1, s2) = tuples();```
* Returning a string slice is not possible, have to use ```&'static str``` instead
  * After deconstruction, the variable is of type &str ...strange
* Vscode debugger says a String if of type alloc::string::String ...strange
  * alloc seems to be a create... maybe this and other creates are automatically imported

# Todo's
* Why can println! take no reference and there is no error of the borrow checker?
