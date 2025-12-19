Assignment 1: Basic Rust Theory Answers
1. Is Rust single-threaded or multi-threaded? Is it synchronous or asynchronous?
   Rust supports multi-threading and can run workloads across many threads using the standard library. By default, Rust programs are synchronous, but Rust also supports asynchronous programming with async/await and runtimes like Tokio or async-std.
2. What runtime Rust has? Does it use a GC (garbage collector)?
   Rust has a very small runtime, without a heavy virtual machine. It does not use a garbage collector; memory is managed through ownership and borrowing rules enforced at compile time.
3. What static typing means? What are the benefits of using it?
   Static typing means variable and expression types are known and checked at compile time. Benefits include better safety, fewer runtime errors, easier refactoring, and faster optimized code.
4. What is immutability? What is the benefit of using it?
   Immutability means that a value cannot be changed after creation. It makes programs easier to reason about, reduces bugs, and improves safety in concurrent environments.
5. What are move semantics? What are borrowing rules? What is the benefit of using them?
   Move semantics mean ownership of a value can be transferred, and the old owner cannot use it anymore. Borrowing rules allow either many immutable references or one mutable reference at a time. This ensures memory safety and prevents data races without a garbage collector.
6. What are traits? How are they used? How do they compare to interfaces?
   Traits define behavior that types can implement and are used for abstraction and generics. They are similar to interfaces but can include default implementations and powerful generic constraints.
7. What are lifetimes? Which problems do they solve?
   Lifetimes describe how long references are valid. They prevent dangling references and ensure references are always valid, solving many memory safety problems at compile time.
8. What are macros? Which problems do they solve?
   Macros are Rust’s metaprogramming tool that allow generating code. They reduce boilerplate, enable custom DSLs, and automate repetitive patterns that normal functions cannot.
9. Difference between &String and &str (or &Vec and &[u8])? Fat vs thin pointers?
   &str and &[u8] are slices that reference data without owning it, while &String and &Vec<u8> reference owning containers. Slices are fat pointers storing pointer + length, while thin pointers only store the address.
10. What are static and dynamic dispatches?
    Static dispatch resolves the concrete type at compile time (usually via generics), producing faster optimized code. Dynamic dispatch uses runtime vtables through dyn Trait, giving flexibility and runtime polymorphism.
