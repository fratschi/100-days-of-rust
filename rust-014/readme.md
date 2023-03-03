

# Things learned

* There is no implicit way to define modules and use them directly like in golang.
* lib.rs and mod.rs are implicitly entry points
* Again, rust feels extremly non-orthogoal (mixing explicit and implicit rules fot the same thing)
* Modules needs to be declared in the parent module or main.rs or lib.rs file
* A module has one of these structures
  * A mod.rs file in a directory (name of the directory is the module name) which is the entry point of the module
  * A xyz.rs file in the same directory as the main.rs file (name of the file is the module name)
* Modules an be nested
* Functions are private by default in a module
* With use, you can pick a submodule and use it when the module is declated
* 

# TODOs
* Are there rules when to use a module as a file or as a directory?
* Can moules be defined with overlapping structures?