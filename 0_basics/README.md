Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
Rust does not yet have a defined memory model. Single-threaded, need to use Arc for using variables in different threads. Synchronous by default, there are async and other concurrency models. 
- What runtime [Rust] has? Does it use a GC (garbage collector)?
No runtime. As a result - no GC.

- What statically typing means? What is a benefit of using it? Weak typing vs strong typing? Implicit / explicit?
Rust is statically and strongly typed PL. The types are defined at compile time and you must explicitly cast types to each other (i32 to u16). Rust can implicitly resolve a type at compile time but you need explicitly describe how to convert one type to another. IMHO: more explicit than implicit. 

- What are generics and parametric polymorphism? Which problems do they solve?
Generics is an implementation of parametric polymorphism. Parametric polymorphism allows a single piece of code to be given a "generic" type, using variables in place of actual types, and then instantiated with particular types as needed. Example: we need to implement our own `+` operation. Without generics we'll need to implement it for every type that we have and it will result in a boilerplate code for u32 u16 i32..... By utilizing generic parameter in a `+` operator we'll let compiler to generate the appropriate code for each type using our generic operator as some kind of a template. 

- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? Uncovered type? What is a marker trait?
Trait is an abstraction that helps to describe what an object should do. Similar to interface in Java. There are differences in implementing generic traits/interfaces in Rust and Java. E.g. in Rust a type can implement a generic trait multiple times by specifying a generic parameter.
Blanked impl - is rusts syntax possibility to implement a generic trait without specifying an actual type, but instead - only specifying a generic constraint (or a conformation). This behavior is often tightened with    `where` keywoard in implementation. Example: `ToString` trait has a blanked implementation for a type that implmements to a `Display` trait. In this way an actual type gets an automatic implementation for `ToString` trait for free if it implements `Display` trait.
Uncovered type - glossary: A type which does not appear as an argument to another type. For example, T is uncovered, but the T in Vec<T> is covered. This is only relevant for type arguments.
My explanation: for a reciever, Vec<T> is kinda responsible to own type `T`, but not a reciever. 
Marker trait: Empty body trait. A programmatic (or a synthetic) way to add constraints to implementing type. Useful in data-driven programming.

- What are static and dynamic dispatches? Which should I use, and when?
Generics - static dispatch, traits - dynamic dispatch. When we know what code will be execuded at compile time - static dispatch, very fast, safe because checked at compile-time. Dynamic dispatch - code that should be executed, will be found at a runtime, less fast at runtime because runtime will need time to find a code to be executed. Need to think wisely: use s.d. by default, but if the algorithm can run faster with dynamic dispatch - need to think about the tradeoffs then.
- What is a crate and what is a module in Rust? How do they differ? How are the used? What is workspace?
Crate is like a library. Crate can contain metadata. Crate can be published. Each crate has an implicit root module that contains the code for that crate. Module is like C++ namespace. 
- What is cloning? What is copying? How do they compare? What is for trait drop? What is special about the trait?
Clone is explicit. A type needs to implement how it should be cloned. Copy - is a process of copying memory. If a type implement copy, then during assignment a value will be copied, if not - moved. 
Drop - a destructor. You may implement there cleaning up resources. Cannot implement both Copy and Drop. Cannot call Drop by itself.

- What is immutability? What is the benefit of using it?
Variable is immutable - the value cannot be changed. Immutable value can be read by multiple threads and we always know how our program will act. It's cheap to copy immutable values - you can just create another pointer to the actual value.

- What are move semantics? What are borrowing rules? What is the benefit of using them?
Move semantics - rules of ownership. A (resource) memory can have only one owner. When `=` or passing as an argument, a memory gets owned by another variable, and current variable is not accessible anymore. 
Borrowing rules - ability to pass that memory by reference to another variable or to a function. This will allow the previous variable not to loose access to the memory.
Benefit from doc - The compiler statically guarantees (via its borrow checker) that references always point to valid objects. That is, while references to an object exist, the object cannot be destroyed.
From multithreaded point of view - having one owner guarantees that the memory cannot be modified by different threads at the same time.
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
Resource Acquisition Is Initialization. Whenever an object goes out of scope, its destructor is called and its owned resources are freed. It is implemented using move semantics. It prevents from memory leaks where a developer forgents to clean a captured memory explicitly. 
**- What are lifetimes? Which problems do they solve? Which benefits do they give?
- What is an iterator? What is a collection? How do they differ? How are they used?
Iterator - design pattern + trait in Rust. Need to implement next() in order to emit optional elements. If you have no elements to emit - you must emit `None` all the time, so design your iterator wisely. Collection - a data structure that contains elements: hashmap, array, linkedlist, tree etc. Collection is a contaier, Iterator provides access to the next element. Iterators are lazy - if you don't `collect` all your operations that you applied to it, it will not provide any values then. Almost any collection implements Iterator - it's a way to provide an element for example in a loop. Collection is a way to store elements (usually efficienlty, with some purpuse). You may implement Iterator for any type, not just for collections.
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
Macros - is rust's ability to write code that writes code (metaprogramming). Macroses modify (usually by adding) the actual code that needs to be compiled. E.g. adding implementation of traits to the type by utilizing #derive macro.
Declarative - works like "match". It takes tokens and matches with its known patterns. If if finds one - it will generate an appropriate code.
Procedural - a code that will execute before compilation. It will take input tokens (a simplest element of compilation) as an argument, modify them (by generating more tokens usually), and return modified tokens. These modified tokens may be some type that implements a trait.
- How code is tested in [Rust]? Where should you put tests and why?
Unit tests. The purpose of unit tests - is to test (and cover) the most of your `units` in your module. 
Mark a module as testable, not buildable by adding `#[cfg(test)]` attribute. This will exclude tests from the final binary binary.
A test module may get access to a super module's private fields by utilizing `use super::*`. As a result - all methods and fields from the parent module and its submodules become accessible.
Integration tests. The purpose of integration tests - is to test if your module gets integrated correctly into the whole system.
Create a separate `tests` directory and only test public methods (obviously, because integration tests don't exist in a testable module and you don't have an access to private methods).
By adding an attribute `test` to a method - you mark it as a test method. It will not be included in a final binary of a product. 
- What is special about slice? What is layout of Rust standard data types? Difference between fat and thin pointers?
Slices let you reference a contiguous sequence of elements in a collection, it doesn't take ownership. It contains a pointer to an interested starting byte of a collection and a length. 
The layout of a type is its size, alignment, and the relative offsets of its fields. The alignment of a value specifies what addresses are valid to store the value at. The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. The size of a value is always a multiple of its alignment.  
The term "fat pointer" is used to refer to references and raw pointers to dynamically sized types (DSTs) – slices or trait objects. A fat pointer contains a pointer plus some information that makes the DST "complete" (e.g. the length). So a slice is a fat pointer.
A thin pointer is a pointer that is essentially a single usize that points to a value of a known type.

- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them? Why str slice coexist with slice?
The str type, also called a ‘string slice’, is the most primitive string type. It is usually seen in its borrowed form, &str. A &str is made up of two components: a pointer to some bytes, and a length, as a result - it's a fat pointer. String slices have a fixed size, and cannot be mutated. 

String is a growable, mutable string type. It is implemented as a vector of bytes.
String implements Deref<Target = str>, and so inherits all of str’s methods. In addition, this means that you can pass a String to a function which takes a &str by using an ampersand (&)

Use String if you need owned string data (like passing strings to other threads, or building them at runtime), and use &str if you only need a view of a string.

- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance? Is Rust functional language?
Yes. Yes/yes. A trait can inherit another trait. But it will mean that the real type will need to   comply to all traits i.e. implement all traits' methods.

After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Trait Drop][10]
- [Common Lifetime Misconception][11]
- [Visualizing Memory Layout][12]

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://vojtechkral.github.io/blag/rust-drop-order/
[11]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
[12]: https://www.youtube.com/watch?v=rDoqT-a6UFg
