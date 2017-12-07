# Rust Monad Kata

This is a kata I have adapted from Mighty Byte's Monad Challenge for
Haskell: http://mightybyte.github.io/monad-challenges/pages/set1.html
The intent of the original challenge is to guide the user into a more
natural understanding of how monads are constructed and why you need
monads. I've implemented the Random Number portion of this kata in
many languages and I wanted to have a go at trying it with Rust.  My
normal approach differs slightly from the original challenge, so I
will document the steps that I suggest for Rust.

Note: I suspect that completing this kata is impossible with the
stable compiler (still working on it myself).  However, discovering
*why* it is impossible will teach you a lot about Rust and is
very much worthwhile IMHO.

## Pseudo-Random Number Generation

Computers do not usually generate actual random numbers when you call
a function like `random()`.  Discussion of pseudo-random number
generators is outside of the scope of this document, but if you are
interested, a good first step might be Wikipedia:
https://en.wikipedia.org/wiki/Pseudorandom_number_generator

What you need to know for this kata is that a pseudo-random number
generator generates a series of output in a specified range.  The
output is deterministic, but not easily predictable without running
the algorithm itself.

The random number function takes a "seed" value and generates an
output.  It always returns the same output for the same seed value.
For example, if your "seed" is 42, the random number function might
always output 2432.  The random number function *also* outputs the
next seed.  You use that seed to get the next number in the series.
Pseudo-random number generators are built so that the output is
uniformly distributed in the desired range.

### Step 1.  Building a "Pseudo-random" generator function

We could use a good random number generator to give realistic output
for our kata, but there is no real need.  Instead our "pseduo-random"
generator will be much simpler:

  - Add a type for a `Seed` that is a 32 bit unsigned integer
  - Write a function called `rand` that takes a `Seed` as input
    and outputs a tuple composed of a 32 bit unsigned integer
	and a Seed.
  - The "random value" that `rand` returns will be equal to the
    seed that you used as input.  The next seed it returns will
	be equal to the previous seed plus 1.

The effect is that if you call `rand(1)` you will receive `(1, 2)`.
If you call `rand(2)` you will receive `(2, 3)`, etc.  Not a great
random number generator, but it will be easy to test.

You must write unit tests for this kata.  Since you can't easily
test an infinite series (and such a challenge is not what this kata
is for), just test a couple of examples.

### Step 2. Playing with assignments

Values of type `Seed` and the tuples that we have are stack allocated.
They implement a "trait" called `Copy`.  What that means is that when
you do an assignment, the value is copied.

We also need to work with heap allocated data structures.  An example
of a Heap allocated data structure is a `Vec` (vector).  These data structures
implement a "trait" called `Move`.  That means that when you do an
assignment, the "ownership" of the memory moves.

Note: A "trait" is kind of like an interface.  It's a promise to the
compiler that there are specific functions implemented for that data
type.  We will talk more about traits later.

We will try to illustrate the difference simply:

  - Write a function that makes a local variable `a` and a local
    variable `b`.  Assign `a` the value 5 and assign `let b = a;`.
    Then return `a + b`.  Write a test to ensure that the result
    is 10.
  - Write a second function that also has local variables `a` and
    `b`.  Assign a `Vec` containing a single value, 5, to `a`.  Assign
    `let b = a;`, like in the previous function.  Add the contents
    together (something like `a[0] + b[0]`).  Write a test to ensure
    that the result is 10.

The second task is impossible.  Try to understand the error message.

### Step 3. A Vector of Random Integers

Write a function, called `five_rands`, that generates a `Vec` of 5
"random" 32 bit unsigned integers.  Start with a seed of 1.  The
output should be `[1, 2, 3, 4, 5]`.

There are lots of ways to cheat with this, but try not to.  You will
need to call `rand(1)` to get the first "random" number.  After that
you will need to use the seed from the previous random number to
generate the next random number.

While doing this, you will probably notice that Rust variables are
immutable by default.  You can make a variable mutable with the `mut`
keyword. It is fairly easy to implement this function with mutable
variables.  It is quite challenging to make a non-ugly function
without using `mut`.  It's worth giving it a go, but don't kill
yourself.  The rest of this kata is discovering how to implement this
function easily without mutating any data.

### Step 4. Different kinds of random values

We want to make a different random generator.  This one will return
"random" chars.  It will be called `rand_letter` and it will take
a `Seed` and return a tuple with a `char` and a `Seed`.
`rand_letter(1)` should return `('a', 2)`, `rand_letter(2)` should
return `('b', 3)`, etc.

Additionally, write a function, called `three_rand_letters`, that
returns a `String` (not a `Vec`!) containing 3 "random" letters.
Start with a seed of 1.  The output should be "abc".  The same
notes about Step 3 apply here.

### Step 5. Many kinds of generator functions

We really want to make a variety of different random numbers.
Implement the following:

  - `rand_even`: The same as rand, but each random number is
    multiplied by 2.
  - `rand_odd`: The same as rand, but rach random number is multplied
    by 2 and has one added to it (`x * 2 + 1`)

We could do a million of these.  It would be better to factor out the
common code.  Avert your gaze from the next step and spend some time
playing with refactoring the code.

### Step 6. Making a Trait

As it happens, what we are trying to do is very common.  There
is a special word for the thing that we are trying to build:
"functor".

A "functor" is really a special word for a container.  It has to
follow some special rules, but any container you are familiar with is
likely to also be a functor.  All functors must have a function
associated with it, usually called `map`.

You may be familiar with `map` on arrays from a variety of different
languages.  However, `map` can be used on any functor.  `map`
takes the contents out of the container, applies a function to the
contents and then puts the results back into the same kind of
container.

We're going to implement a `Rand` functor on tuples like
`(uint32, Seed)` and make it a trait called `Functor`.

Note: I haven't done this yet, so the instructions may be subtly
wrong.

  - Make a type called `Rand` for a tuple that contains an unsigned
    32 bit integer and a `Seed`.
  - Write a function called `rand_map`.  It should have the following
    type signature: `fn rand_map(fn(uint32) -> uint32, Rand) -> Rand`
  - Make a trait called `Functor` with a `map` function and implement
    this function with `rand_map`.
  - Use the `map` functionality in `rand_even` and `rand_odd`.

### Step 7. What about `rand_letter`?

Our `Functor` trait is pretty awesome but it's kind of pointless if we
only have a single type.

  - Create a type for the output of `rand_letter` called `RandLetter`,
  - Create a `map` implementation for it
  - Refactor `rand_letter` to use that implementation.

### Step 8. Parametric Polymorphism

Raise your hand if you think CS has the *best* jargon of any field!

Basically it sucks that we have to make a separate implementation of
`map` for both `Rand` and `RandLetter`.  They probably look basically
identical.  The only difference should be in the types being used.
Luckily we can use "Parametric Polymorphism" to refactor this into one
function.

  - Refactor `Rand` and `RandLetter` into one type `Rand<T>`.
  - Use `Rand<T>` in your implementation of `map`
  - Note to self: How does that affect the trait?  Can you have traits
    for parametric types?

### Step 9. Generating Random Pairs

Write a function, called `rand_pair` that takes a `Seed` and outputs
a tuple like the following: `((char, uint32), Seed)`.  For example,
`rand_pair(1)` should return `(('a', 2), 3)`.  So basically you have
to use the seed in the return value from `rand_letter` as the seed
in `rand`.

### Step 10. Generator Types

It would be nice to be able to output pairs of any kind of type.
Ideally, we would like to implement `rand_pair` by passing the
functions `rand_letter` and `rand` and having that function
construct the tuple.  Specifying the parameter list as
`fn general_pair<A,B>(gena: fn(Seed) -> (A, Seed), genb: fn(Seed) -> (B, Seed), Seed) -> ((A, B), Seed)`
is a gigantic PITA.

  - Make a single parametric type, `Gen<T>`, for the functions `rand` and
    `rand_letter`.
  - Write a function, `general_pair`, that takes a `Gen<A>`, a `Gen<B>` and a
    `Seed` and returns the correct random pair.
  - Refactor `rand_pair` to use `general_pair`.

### Step 11. Returning a Closure

Why do we bother passing the `Seed` into `general_pair`?  `general_pair`
doesn't have to actually calculate everything immediately.

  - Don't pass the `Seed` to `general_pair`.  Instead have it return a
    closure that takes a `Seed` as a parameter.  `rand_pair` should
    look something like: `general_pair(rand_letter, rand)(seed)`.

Spoiler: For very good reasons, Rust implements functions and closures
differently.  The type of a function is `fn(...) -> ...` The type of a
closure is `Fn(...) -> ...`.  When Rust returns a value from a
function, it uses the memory allocated on the stack in the calling
function.  For example if `f` calls `g` and `g` returns a value, the
memory for that value is allocated in the stack frame for `f`.

Unfortunately, a closure (`Fn`) has indeterminate size.  It could be
anything.  `f` can't know the size of the closure that `g` will return
until `g` runs -- at which point it is too late.  To solve the
problem, you can allocate the `Fn` in `g` on the heap and pass a smart
pointer back to f.  When that smart pointer goes out of scope in `f`
it will deallocate the memory for the closure.  Since a smart pointer
has a defined size, you can do this.

To do this you need to create a `Box` for the closure.  The return
type becomes `Box<Fn(...) -> ...>`.

### Step 12. An Inconvenient Truth

If you continue on with this kata in this fashion, you will run into a
problem.  The `Rand` type is a functor, but it is *not* an
"applicative functor".  It is worthwhile going down this road and
understanding why, but it's quite a long diversion.  In order to save
time, we'll jump directly to the solution.  Later you may wish to come
back and explore why you can't continue with the `Rand` type.

So even without knowing what an "applicative functor" is and why we need
it, let's make a change.  To give you a slight premonition, when you
look at the type `Rand<T>` it depends on 2 things: the result
and the `Seed`.  In contrast, the `Gen<T>` generator types only depend
on the `Seed`.  The result is generated from the `Seed`.  If we
refactor our code to use `Gen<T>` instead of `Rand<T>` it will improve
things dramatically.

But is `Gen<T>` a functor?  What does that mean?  Is a function a
container?  As it happens, it is.  You can happily implement `map` for
`Gen<T>`.

  - Add the `Functor` trait to `Gen<T>` by implementing `map` for it.
  - Rewrite `rand_letter`, `rand_even` and `rand_odd` to use `map` on
    `Gen<T>`.

There is an unfortunate caveat here: `map` is supposed to take a
container, take out the contents, apply a function and then put the
result back into the same kind of container.  `Gen<T>` is the
container and is a function, but we've seen that Rust functions can not return
a function.  It can only return a boxed closure.  This is not the same
type.  We'll roll with it, but this may eventually cause us problems.

### Step 13. Working with Pairs

It would be nice to build pairs of random values, but not just in a
tuple.  For example, making a pair of random values in a `String`.

  - Write a function `gen_map2` that allows you to do this.

Hint: It's just like `gen_map`, but with 2 `Gen<T>` values instead of
one.

Warning: It will be *very* tempting to use `gen_map` in your
implementation.  Feel free to try if you *really* want to understand
the details of the various closure types in Rust.  I think it's
actually impossible in the current stable version of the compiler.

### Step 14. Vectors of Generators

Being able to generate a pair of random values is slightly useful.
However, we really want to be able to generate an arbitrarily sized
vector of random values.  What would be nice is:

  - Write a function, rep_random, that takes a `Vec` of `Gen<T>` and generates a
    `Gen<Vec<T>>`.  In orther words it takes a vector of generators
    and returns a Generator that gives you a random vector of values.

As a usage example:
```
  let gs = vec![rand, rand, rand, rand, rand];
  assert_eq!(vec![1, 2, 3, 4, 5], rep_random(gs)(1).0);
```
