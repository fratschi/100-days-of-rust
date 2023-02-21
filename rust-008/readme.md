

# Things learned
* Parsing is bound to error handling with a Result
* If an error is not handled, a panic is thrown
* You can get the value with .unwrap() or ? but you need to be sure that the value is parsed correctly
* error! does not work if logging is not initialized

# TODO's
* There is no documentation available in the chrono sources which placeholders are available and what they are doing