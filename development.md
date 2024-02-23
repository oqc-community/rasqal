### Rust++

Our Rust code is actually more like C++ due to some fun pointer manipulation we do and heavily interlinked data structures.

The main points to know are:

1. We have a custom ref-counted smart-pointer that acts like any other ref-counted smart pointer whose backing is a raw pointer.
2. The smart-pointer can point to anything pointer-like: actual pointers, references, mutable references, anything you can actually get a pointer too.
3. It uses macros to manipulate/call/fetch the pointers directly and avoid pointer-to-ref compiler issues.
4. You can mutate anything at any time through the smart-pointer and its macros, so mutability keywords are irrelevant.

In most situations you can just treat our smart-pointer like a normal `Rc` and don't need to care about its internals. 
Except if you steal a reference, then it's on you to make sure the memory is not referenced outside its lifetime - the old fashioned way.

Otherwise you can just write Rust as normal, and anything not within a smart-pointer still has the usual rules.  

Note: If you are concerned about this or are surprised it works at all - raw pointers have special designation, specially those in `UnsafeCell`'s.
We are leaning upon some pretty niche documented constraints to keep within the bounds of Rusts expectations, if barely, but they are all official.

The language _almost_ works for what we need, so we bend it to meet that.
