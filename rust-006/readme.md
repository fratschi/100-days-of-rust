

* log needs to be imported and mentioned in the Cargo.toml
* logging with info! macro does not log anything (need to use env_logger::init())
* Examples like https://docs.rs/log/latest/log/ do not work out of the box, hard to use if dependencies are not mentioned
* Using ```env_logger::init();``` does not need a use statement because its fully qualified
* Default level is error, so info! macro does not log anything => Worng, my beloved copilot!. It's because I did not use the env_logger::init() function.
* structured logging as json seems to be a painful thing and not really compatible with the standard log crate
``` 
[dependencies]
json_env_logger = { version = "0.1", features = ["iso-timestamps"] }
log = ">=0.4.13, <0.4.14"
```
* It looks like rust has no transitive dependencies?
* tokio tracing_subscriber doest produce json output
* This syntax is a good example of unnecessary complexity in rust ```info!("{{ \"msg\": \"{}\" }}", "preparing to shave yaks");```
  * { is escaped with {{  while the " is escaped with \"

# TODO's
* Why does the log macro not work without the env_logger::init() function? The crate is not a production optimized one.