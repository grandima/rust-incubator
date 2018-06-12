Rust Incubator
==============

>>>
It wasn’t always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.
_<div align="right">Rust Book's Foreword</div>_
>>>

This project represents a step-by-step [Rust] learning course from language basics to production use.




## Prerequisites


### Toolchain

- [rustup] for installing [Rust] toolchain and keeping it up-to-date.
- [IntelliJ IDEA] + [IntelliJ Rust] and [Toml][IntelliJ Toml] plugins as development environment.
- [rustfmt] for formatting [Rust] code.
- [GNU Make][Make] usage ability for performing commands declared in [`Makefile`].


### Bookshelf

- [Rust Book] teaches and explains [Rust] basics.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Rust Standard Library] documentation.
- [Rust Style Guide] is an official style of formatting [Rust] code.
- [Cargo Book] is a guide to [Cargo], [Rust]'s build tool and dependency manager.
- [Rustdoc Book] is a guide to `rustdoc` documentation tool.
- [Rust By Example] teaches you how to program in Rust using editable examples.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the [Rust] ecosystem.
- [Rust FAQ] answers common question about [Rust].
- [Rust Playground] allows to share and check runnable [Rust] code snippets online.
- [Awesome Rust] is a curated list of Rust code and resources.
- [Baby Steps] blog of [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful [Rust] patterns and ideas.




## Steps

Each step must be performed as separate [MR (Merge Request)][MR] with correspondent name and checked as done after it's completed.

Do not hesitate to ask your lead with questions.

- [ ] [0. Become familiar with Rust basics][Step 0]
- 1. Key concepts
    - [ ] [1.1 Type safety][Step 1.1]
    - 1.2. `Cell`/`RefCell`: shared mutation
    - [ ] [1.3. `Cow`: clone-on-write][Step 1.3]
    - 1.4. `Rc`: reference counting
    - 1.5. `Default`: default values
    - 1.6. `Deref`: dereferencing
- 2. Primitives and tools
    - [ ] [2.1 Date and time][Step 2.1]
    - [ ] [2.2 Regular expressions][Step 2.2]
    - 2.3. Bitmasks
    - 2.4. Collections
    - 2.5. Encoding and serialization
    - 2.6. Randomness
    - 2.7. Cryptography
    - 2.8. Logging
    - 2.9. Command-line arguments
    - 2.10. Environment variables
    - 2.11. Configuration
    - 2.12. Threads, synchronization and parallelism
    - 2.13. Futures and async I/O
    - 2.14. Actors
    - 2.15. Databases and ORMs
- 3. Architecture
    - 3.1. Long-running application
    - 3.2. Clean architecture





[Step 0]: 0_basics
[Step 1.1]: 1_key_concepts/1_1_type_safety
[Step 1.3]: 1_key_concepts/1_3_clone_on_write
[Step 2.1]: 2_primitives_and_tools/2_1_date_and_time
[Step 2.2]: 2_primitives_and_tools/2_2_regular_expressions

[`Makefile`]: Makefile

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Make]: https://www.gnu.org/software/make
[MR]: https://docs.gitlab.com/ce/user/project/merge_requests
[IntelliJ IDEA]: https://www.jetbrains.com/idea
[IntelliJ Rust]: https://intellij-rust.github.io
[IntelliJ Toml]: https://plugins.jetbrains.com/plugin/8195-toml
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust FAQ]: https://www.rust-lang.org/faq.html
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust Standard Library]: https://doc.rust-lang.org/std
[Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustup]: https://rustup.rs
