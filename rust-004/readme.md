# Day 002 - 2023-02-15

# Things learned
* Concatenation of strings with `+` is not possible. Use `format!` instead.
* Concatenation of strings is hard
* There are too many string representations and I tend to call them all strings and mixing them up
* format! is versatile
* contact! is only usable with literals and not with variables
* push_str needs a mutable reference and is not returning a new string
* List of all string representations - holy moly ;-)
  * 'c' - char
  * "test" - literal
  * str - string slice
  * &str - string slice reference
  * &mut str - mutable string slice reference
  * &'static str - static string slice reference
  * String - owned string
  * &String - owned string reference
  * &mut String - mutable owned string reference

# Todo's
* Is clone() the only way to unmutable a string?
* I need to create a matrix of all string representations and their possible operations for converting them to each other
* Is there a list of all macros of the default scope? Maybe macros aren't scoped at all?
* Does the list of string types must be extened with pointers?
* How rust macros know that e.g. only take string literals?
* 'static lifetime seems to be a special case. What is it good for?