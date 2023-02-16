# Day 002 - 2023-02-15

# Things learned
* Enums are powerful and can take arguments
* The match statement is handy and can be used to match on enums
* Match can deconstruct the arguments of an enum
* Github Copilot helps generating code that passes the compiler
* Self referencing enums are possible but you need a Box for encapsulaton and make  the enum of a fixed size
* Constructing a self referenced enum does not look nice... Maybe there is a better way?
* Still have no clue whats all in the default scope of rust
* println! has some options, can't find a complete list of them
  * Here are examples: https://doc.rust-lang.org/rust-by-example/hello/print.html
  * This seems to be the complete list: https://doc.rust-lang.org/std/fmt/#formatting-traits
* Github Copilot gives me this : "The compiler is very helpful and gives you a lot of hints" ... I don't think that i can comply with this statement yet
* eprintln!  write to std err instead of std out
* format_args! is a weird macro... don't understand what is for: https://doc.rust-lang.org/std/fmt/#format_args

# Todo's
* [How to have a look at the generated code of e.g. println!][(](https://github.com/fratschi/100-days-of-rust/issues/4))
