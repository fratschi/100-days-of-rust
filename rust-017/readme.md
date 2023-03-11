# Things learned

* Tokio and Axum re working well togehter
* Passing the http request to the handlers is arkward
* Passing shared state or a simple config is complex, closures are needed
* Official documentation about closures and the move statement are not sufficient
* await is a keyword, but used like member. I dont like such a magic
* Don't know where e.g. move |path| getfunc(path) is comming from
* Why is no semicolon necessary at the end?
* What are the curly braces in this context? is this the closure?
