# Ray Tracer Challenge Implementation Notes
Compilation of notes...

## Chapter 2 Refactor

Following completion of Chapter 2 of the book, I undertook a refactoring effort to make the `Tuple` operations code more idiomatic Rust. I started with a dynamic approach allowing tuples of any length and performing operations on those simple dynamic types but that led to a basic problem: runtime errors.

The dynamic approach of doing generic operations on tuples of varying lengths means allowing operation which aren't logical and shouldn't be possible such as subtracting a point from a vector. The book uses the `w` element to determine a vector from a point and `w` must be 1 or 0 for the operation to be valid. If you attempt an impossible operation it will succeed but you'll get something which is neither a point nor a vector.

Rust prefers types to states because state exists at run time whereas types exists at compile time and the compiler detects invalid types for us automatically. By relying on types rather than states Rust makes operations which should be impossible, impossible by not allow us to compile such a monster. The other advantage is there is little runtime error checking or handling because we've offloaded that work to the Rust compiler. _Type States_ are a powerful concept in Rust.

I replaced `Tuple` with `Vector3D` & `Point3D` and implemented the operations which are possible while relying on the compiler to report the impossible ones. This feels much cleaner and I feel that if I try to do something dumb in the future, like subtracting a point from a vector, the rustc will give me a nice error and fail. My initial attempt was to use generic tuple data types but this turned out to be a fool's errand and instead I settled on defining a type `Unit3D` which maps to `f32`. Supporting integer tuples is a lot of work without any obvious need to do it.
