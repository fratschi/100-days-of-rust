
# Things learned

* dependencies are used for compilation
* dev-dependencies are used for testing (so technical, the statement above is false)
* https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html  is a good reference for understanding
* Target specific dev-dependencies are possible ```[target.'cfg(unix)'.dev-dependencies]```
  * Propable no one can memorize this syntax.
* Maybe cargo edit can help with editing dependencies? https://github.com/killercup/cargo-edit
* When you add a dependency, you have to check crates.io for dev-dependencies.
* The compiler error for missing dev-dependencies is not helpful. Don't know how fix it if a missing dev-dependency is the reason for the error.

# TODO
* Is there a way to find dev-devendencies for a crate without using crates.io?
