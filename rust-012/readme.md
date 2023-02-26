
# Things learned

* A struct is not necessary for reading json, it can be directly loaded into an enum
* Expect does not work as expented, dont know where the expect string is printed or if its used at all
* Enums need a descriminator for deserializing
* Using an old trick to find the correct json: Create the json and then print it to the console
* nether to_string or to_string_pretty are directly usable, they escape the json and makes it unusable
* Rust has nasty side effects on error free lines of code, like this:
  * This has a compile error: 
```rust
    let sladata = serde_json::from_str(&contents).unwrap();

    let sla = calculate_sla(sladata)*100f32;
```

 * This works:
```rust
    let sladata = serde_json::from_str(&contents).unwrap();

    let sla = calculate_sla(&sladata)*100f32;
```

I would expect that the compiler complains about the second line. The error message is (actually for me) not helpful at all.

# TODOs
* Clean this up and add checking for errors