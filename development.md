### R++

Our Rust code is actually more like C++ due to some fun pointer manipulation we do and heavily interlinked data structures.

The main points to know are:

1. We have a custom ref-counted smart-pointer that acts, superficially, like any other ref-counted smart pointer whose backing is a raw pointer.
2. The smart-pointer can point to anything pointer-like: actual pointers, references, mutable references, anything you can actually get a pointer too.
3. It uses macros to manipulate/call/fetch the pointers directly and avoid pointer-to-ref compiler issues.
4. You can mutate anything at any time through the smart-pointer and its macros, so mutability keywords are irrelevant.

The smart pointer and its macros mean that two important features of Rust are disabled when they are involved: lifetime tracking/borrow checking and the one-mutable-reference constraint. 
Pointer lifetimes is dealt with programatically - similar to C++ - and its internal structure allows bypassing Rusts aliasing rules when using its macros.

Because of this the Rust you'll see will look a little different than in other projects. There are almost no lifetime constraints and those that are will be enforced to be the widest scope.
The `mut` keyword means nothing if a smart-pointer is in play, it will be able to be mutated anyway.

But what does this actually mean in regard to writing code? Well, not much really. 
You just write Rust as normal, no need to think about anything additional. 
Just think of `Ptr` as a more feature-rich `Rc`.

For anyone immediately concerned by reading this: raw pointers have special designation, specially those in `UnsafeCell`'s.
We are leaning upon some pretty niche documented constraints to keep within the bounds of Rusts expectations, if barely.

We'd prefer to use more normal Rust, but right now its rules do not allow that without some major contortions.
