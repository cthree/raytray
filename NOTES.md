# Ray Tracer Challenge Implementation Notes

Here I'm keeping notes about the process of working through the book's challenge, building a photo-realistic ray raying application, and at the same time practicing my Rust vocabulary. On the one hand I find Rust overly doting and pedantic and the other hand what compiles works and if you git gud you can solve primal problems that error check themselves at *compile time*. That's what I'm trying to do with this exercise: up my Rust game and keep my tools sharp.

## Chapter 1 & 2

To start I'm following the flow of the book. I have not read ahead so I'm not confident some of the choices are going to live long. I've opted to center around an extensible `Tuple` module.

Implemented vectors, points and color as `Tuple` factory methods. Implemented `Canvas` with PPM output per the book.

Wrote the two examples: `fodder` & `fodder_ppm` which calculate a balistic tragectory and output the path to a PPM file respectively.

## Chapter 1 & 2 Refactor

Following completion of Chapter 2 of the book, I undertook a refactoring effort to make the `Tuple` operations code more idiomatic Rust. I started with a dynamic approach allowing tuples of any length and performing operations on those simple dynamic types but that led to problems:

* runtime error handling
* manual validation of compatible terms
* allocations and reallocations to the heap, sometimes in a tight loop
* posible to write invalid programs, not idiomatic!

The dynamic approach of doing generic operations on tuples of varying lengths means allowing operation which aren't logical and shouldn't be possible such as subtracting a point from a vector. The book uses the `w` element to determine a vector from a point and `w` must be 1 or 0 for the operation to be valid. If you attempt an impossible operation it will succeed but you'll get something which is neither a point nor a vector.

Rust prefers types to states because state exists at run time whereas types exists at compile time and the compiler detects invalid types for us automatically. By relying on types rather than states Rust makes operations which should be impossible, impossible by not allow us to compile such a monster. The other advantage is there is little runtime error checking or handling because we've offloaded that work to the Rust compiler. *Type States* are a powerful concept in Rust.

I replaced `Tuple` with `Vector3D` & `Point3D` and implemented the operations which are possible while relying on the compiler to report the impossible ones. This feels much cleaner and I feel that if I try to do something dumb in the future, like subtracting a point from a vector, the rustc will give me a nice error and fail. My initial attempt was to use generic tuple data types but this turned out to be a fool's errand and instead I settled on defining a type `Unit3D` which maps to `f32`. Supporting integer tuples is a lot of work without any obvious need to do it.

## Chapter 3

Since the book says we are using 4x4 matrices I'v commited to that in how I implemented chapter 3's matrix inversion operations. Added the `Matrix` type for a 4x4 matrix. I'm not super happy about the duplication of the matrix*tuple implementation for points and vectors. I'm not sure how that will shake out so I'm going to leave it for now and see of a single generic implementation make's sense later.

I think there is significant optimization possible.