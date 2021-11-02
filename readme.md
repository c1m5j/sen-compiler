This is a (for now) small compiler which generates symbolic expression-tree structures from Lisp-like code. In "src/main.rs" there is a variable `filepath`, set that to the filepath of your source file to compile. If everything goes alright, after `cargo run` you should see your AST in the terminal. Otherwise the program will panic and you'll see the appropriate error message.

