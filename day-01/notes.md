println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). Rust macros are a way to write code that generates code to extend Rust syntax

easy way to get that Cargo.toml file is to run cargo init

the default build is a debug build, Cargo puts the binary in a directory named debug.
 $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

run cargo check periodically as they write their program to make sure it compiles

 :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types 

 read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

 Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.


 Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables,