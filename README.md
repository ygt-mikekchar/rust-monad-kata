# Rust Monad Kata

This is a kata I have adapted from Mighty Byte's Monad Challenge for
Haskell: http://mightybyte.github.io/monad-challenges/pages/set1.html
The intent of the original challenge is to guide the user into a more
natural understanding of how monads are constructed and why you need
monads. I've implemented the Random Number portion of this kata in
many languages and I wanted to have a go at trying it with Rust.  My
normal approach differs slightly from the original challenge, so I
will document the steps that I suggest for Rust.

Note: I think that everything in this kata so far can be done with the
stable compiler.  "Higher order types", which is what we are doing in
this kata is a well known pain point in Rust.  I didn't know this
before I started along this path, but even still I think this is a
great exercise for discovering both the good things about Rust and the
limitations.

## How Much Rust Do I Need To Know?

I did this kata knowing almost nothing about Rust.  Somebody on the
internet posted a nice comparison between Rust and some other
functional languages and I wanted to see how easy it would be to
write code in that style.  I have learned quite a lot about Rust in
the process, so in that sense it was quite successful.

I've tried to smooth out a few of the tricky bits with hints along the
way, but make no mistake: this is learning by getting into the deep
end.  If you already have a good understanding about monads *or* a
good understanding about Rust, you will have an easier time.  However,
I think you can succeed without either.

Expect this kata to take you a few days to complete if you are
learning a lot of new things, but you should be able to find out how
to do everything you need by reading the excellent Rust documentation
(as I did).

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

We could use a good pseudo-random number generator to give realistic output
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

You should write unit tests for this kata.  There is a lot of
refactoring involved and it will save you a lot of time ensuring that
you haven't broken something along the way.  Since you can't easily
test an infinite series (and such a challenge is not what this kata
is for), just test a couple of examples.

### Step 2. Playing with assignments

Values of type `Seed` and the tuples that we are using are stack allocated.
They implement a "trait" called `Copy`.  What that means is that when
you do an assignment, the value is copied.

We also need to work with heap allocated data structures.  An example
of a Heap allocated data structure is a `Vec` (vector).  These data structures
implement a "trait" called `Move`.  That means that when you do an
assignment, the "ownership" of the memory moves.

Note: A "trait" is kind of like an interface.  It's a promise to the
compiler that there are specific functions implemented for that data
type.  We will work with traits later.

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
yourself.  Most of the rest of this kata is discovering how to
implement this function easily without mutating any data.

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

  - Make a type called `Rand` for a tuple that contains an unsigned
    32 bit integer and a `Seed`.
  - Write a function called `rand_map`.  It should have the following
    type signature: `fn rand_map(fn(uint32) -> uint32, Rand) -> Rand`
  - Use the `rand_map` functionality in `rand_even` and `rand_odd`.
  - Make a trait called `Functor` with a `map` function and implement
    this function with the code from `rand_map` (you can delete `rand_map`
	afterwards).

Note: we are going to pass functions (type `fn(...) -> ...`) into `map`
rather than closures (type `Fn(...) -> ...`) for the moment.  This
will allow us to avoid issues with lifetimes.  Later we will need
to modify this to use closures, but for now stick to functions.

#### Side note: Why is it called a "Functor"?

It seems like a strange name for a collection.  In Category theory, a
"functor" is actually a "mapping" between 2 categories.  You can
consider a category as a set of objects.  For example, the
set of positive integers is a category.  The set of unicode characters
is also a category.

Imagine you have a set of integers from 0-9, called A.  Then imagine
a set of integers from 1-10, called B.  We can easily "map" between
A and B with a function like: `|x| x + 1`, called `add_one`.  Using
`add_one` we can "map" between the categories A and B.

This is what's known as a "functor" and you can probably see why it
has a name that is similar to "function".  We usually say that "A is a
functor", even though we don't specify the function that we are using
to map it with.  What we mean by this is that we can easily write a
function called `map` that will map integers from A to some other
(or the same) category, given a transforming function.

To really be a functor, the `map` function must adhere to 2
conditions:

  - There must be a kind of "identity" mapping: a mapping that maps
    from every item in the category to itself.  For numbers, that function
    would be `|x| x + 0`.
  - The functions must be "composable".  So if we have the functions
    `let f = |x| x + 1` and `let g = |x| x + 2`, we could "compose"
	the 2 functions into `let fg = |x| (x + 2) + 1`.  To be a functor,
	`f(g(x))` must be equal to `fg(x)` for all functions `f` and `g`.

Things like numbers tend to be functors because you can easily write
`map` so that those conditions apply.

You might be wondering, "Why is a functor a collection?"  Imagine a
single number as being a "collection" of one number.  We can also
have a set of tuples: for example the set of `(0, 0), (0, 1), (1, 0),
(1, 1)`.  You can probably imagine that this is a functor as well
because we can easily write `map` to work on each value in the tuple.
The same goes for arrays, strings, and many other collections.

That's why we say that a collection that implements `map` is a functor.

### Step 7. What about `rand_letter`?

Our `Functor` trait is pretty awesome but it's kind of pointless if we
only have a single type.

  - Create a type for the output of `rand_letter` called `RandLetter`,
  - Create a `map` implementation for it so that RandLetter has the Functor trait.
  - Refactor rand_letter to `map` from a `Rand` to a `RandLetter`.

Note: This is impossible.  Why?

### Step 8. Parametric Polymorphism

Raise your hand if you think CS has the *best* jargon of any field!

Because of what you discovered in Step 7, you can't write a single
trait for Rand and RandLetter without using parametric types.

  - Refactor `Rand` and `RandLetter` into one type `Rand<T>`.
  - Make a `Functor` implementation for `Rand<T>`
  - Implement `rand_letter` using `map`.

Note: If you look at the documentation, every example sends a
reference of `self` (i.e. `&self`) into the functions.  If you do
that, you will have to worry about lifetimes.  Because we are only
using data types with the `Copy` trait, save yourself lots of trouble
and use `self` so that it copies the values rather than moving them.

### Step 9. Generating Random Pairs

Write a function, called `rand_pair` that takes a `Seed` and outputs
a tuple like the following: `((char, uint32), Seed)`.  For example,
`rand_pair(1)` should return `(('a', 2), 3)`.  Basically you have
to use the seed in the return value from `rand_letter` as the seed
for `rand`.

Note: It will be very tempting to implement this function using
`map`, but for now resist that temptation.

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

Note: Again, we will eventually need to pass closures into these functions,
but unfortunately you can't make a type alias for a closure.  Using the
`Gen<T>` type will makes certain things easier to understand in the future,
so don't be tempted to jump directly to closures.

### Step 11. Returning a Closure

Why do we bother passing the `Seed` into `general_pair`?  `general_pair`
doesn't have to actually calculate everything immediately.

  - Don't pass the `Seed` to `general_pair`.  Instead have it return a
    closure that takes a `Seed` as a parameter.  `rand_pair` should
    look something like: `general_pair(rand_letter, rand)(seed)`.

There are two spoilers here for dealing with the return type from
`general_pair`.  It's worth trying to figure it out yourself, but
if you get stuck, feel free to read below for some hints.

Spoiler #1: For very good reasons, Rust implements functions and closures
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
type becomes something like (but not exactly like) `Box<Fn(...) -> ...>`.

Spoiler #2: The final boxed function closes over the parameters sent
to `general_pair`.  In that way it is also a container.  The problem
that you will probably find is that the compiler complains about
lifetimes of objects.  Somehow you have to link the lifetime of the
returned `Box` to the lifetimes of the `Gen<A>` and `Gen<B>` that you
are passing to the function.

### Step 12. Is it Pure?

We're going to march down the road towards what is called an
"applicative functor".  This is one of the places where we will
diverge a bit from the category theory nomenclature.  In category
theory the applicative functor is apparently related to a "lax
monoidal functor with tensorial strength" (according to wikipedia).
I like to think that they went with "applicative" because the other
name is too hard to type.

For many people, applicative functors (or "applicatives" for short) are
a bit of a mystery and several language designers intentionally short
circuit directly from functor to monad.  I think this is a shame
because, as you will see later, the applicative falls directly out of
using functors.  If you don't become familiar with applicatives, then
you may find yourself staring at some strange code and wondering how
you got there.

Note: this is probably less likely to happen with a language that
doesn't natively support partial function application (like Rust),
but I still think it's worth understanding what the applicative is
for.

Before we go too far down the rabbit hole, though, let's implement a
simple function that is required for a type to be an applicative
functor:

  - Implement `rand_pure` for `Rand<T>`.  `rand_pure` takes a value
    and puts it in the functor.  For example for a vector, `pure(5)`
    would result in `[5]`.  For `Rand<u32>` that means that we can
    call `pure(5)` and return a `Rand` with 5 in it.

You would be right to wonder why we are implementing this.  While it
makes sense for a vector, does it actually make sense for a `Rand<T>`?
What should the seed be in that case?  Play with this for a while
and decide for yourself what `rand_pure` should do.

Note: Historically `pure` had many different names in various
programming languages including `unit` and `return`.  Lately it seems
that people are settling on the name `pure` which is why I'm using it
in this kata.

### Step 13. An Inconvenient Truth

In fact, the `Rand<T>` type is a functor, but it is *not* an
"applicative functor".  We can write a `rand_pure` but not without
considerable compromise.  Not only do we need to know the "random"
value to store, but we also need to know the value of the seed.

What can we use instead of `Rand<T>`?  Well, we got stuck
because we don't know what seed value to put into `Rand<T>`.  What if
we could defer the choice of the seed until later?  We could have a
kind of "lazy" `Rand<T>` that lets you fill in the seed when you have
it.  A function would work: `fn(Seed) -> Rand<T>`.  But this is just
`Gen<T>`!

But is `Gen<T>` a functor?  What does that mean?  Is a function a
container?  Actually, it is!  It contains whatever values that it
evaluates to.  You can happily implement `map` for `Gen<T>`.

  - Add the `Functor` trait to `Gen<T>` by implementing `map` for it.
  - Rewrite `rand_letter`, `rand_even` and `rand_odd` to use `map` on
    `Gen<T>`.

There is an unfortunate caveat here: `map` is supposed to take a
container, take out the contents, apply a function and then put the
result back into the same kind of container.  `Gen<T>` is the
container and is a function, but we've seen that Rust functions can
not return a function.  It can only return a boxed closure.  This is
not the same type.  We'll roll with it, but this may eventually cause
us problems.

Once you've implemented `map` for `Gen<T>` think about what it is
doing. Does your implementation of map guarantee the 2 conditions
for `Gen<T>` being a functor?  What would the identity function be?
Why is the second condition especially easy to guarantee?

Spoiler #1: Once you get the trait written you will find that it does
not work.  That's because Rust does not do type inference on
functions.  This is very unfortunate, but you can solve the problem by
casting the generator like `(rand as Gen<u32>).map(...)`.

### Step 14. Pure as the Driven Snow

- To show that we've gotten past our `Applicative` problem, try to
  implement `gen_pure`.  For a `Gen<u32>`, you should be able to pass
  a `u32` and it returns a boxed closure.  The boxed closure should
  accept a seed and return the `u32` that we passed in earlier.

Having done that, you can see that we've fixed our problem, but you
may still be wondering *why* we need such a strange function.  You
will see later.

Note: The reason for introducting `pure` so early in the proceedings is
simply to show that `Rand<T>` won't work without wasting too much
time.  After completing the kata once, it can be interesting to try to
implement everything with `Rand<T>` to get a better understanding of
what the problem is.

### Step 15. Working with Pairs

It would be nice to build pairs of random values, but not just in a
tuple.  For example, making a pair of random values in a `String`.

  - Write a function `gen_lift2` that allows you to do this.

Hint: It's very similar to `gen_map`, but with 2 `Gen<T>` values
instead of one.

#### Step 16. Applicative Functors

`gen_lift2` is very similar to `gen_map`, but combines the output of 2
`Gen<T>`s into one.  The function you pass to `gen_lift2` should have a type
signature of `fn(A, B) -> C`.  Similar to `general_pair`,
it's very tempting to use `gen_map` in the implementation.  `gen_map`
uses a function with a signature of `fn(A) -> B`, though.  If we could
somehow convert `fn(A, B) -> C` into `fn(A) -> B`, then we could use it.

In a more usual functional language, you would simply "curry" the
value.  For example, if I am supposed to call a function like
`my_func(a, b)`, I can "partially apply" the function by passing only
the first parameter.  The result is a function that takes the second
parameter. In other words, I can do `my_func(a)(b)` and it will have
the same output.

Let's say that I am writing `gen_lift2` and I just naively pass the
incoming function directly to `gen_map`. Assume `f: Fn(A, B) -> C`,
`ga: Gen<A>` and `gb: Gen<B>`.  So ideally, what would `gen_map(f, ga)`
return?

In an ideal world, it would return `Gen<Fn(B) -> C>` because it would
just partially apply the first parameter to the function and return a
new function wrapped in a `Gen`.

What we need now is a function that can apply the *second* (and
potentially subsequent) parameters to the function.  That function is
called `apply`.  If you can write `apply` for your functor, then you
have an "applicative functor".  As we saw, not every functor is
applicative.

  - Write a function `gen_apply` that takes a `Gen<Fn(A) -> B>` and
    a `Gen<A>` and returns `Gen<B>`.
  - Refactor `gen_lift2` to use `gen_map` and `gen_apply`.
  - Make a functor called `Applicative` that gives you `apply`, `pure` and
    `lift2`.  Make `Gen<T>` `Applicative`.

Note: This is quite tricky and requires that you understand the lifetimes
of your variables/functions.  I *think* you also need to make the function
that you pass to `apply` `FnOnce` because you need to partially apply
the function when you run `map`.  I'm still a Rust novice, so it's possible
I'm missing something.

### Step 17. Vectors of Generators

Being able to generate a pair of random values is slightly useful.
However, we really want to be able to generate an arbitrarily sized
vector of random values.  What would be nice is:

  - Write a function, `gen_sequence`, that takes a `Vec` of `Gen<T>` and
    generates a `Gen<Vec<T>>`.  In orther words it takes a vector of
    generators and returns a Generator that gives you a random vector
    of values.

As a usage example:
```
  let gs = vec![rand, rand, rand, rand, rand];
  assert_eq!(vec![1, 2, 3, 4, 5], gen_sequence(gs)(1).0);
```

  Note: you can implement this with and without `apply`.  This is
  because Rust iterators give you enough capability to avoid using
  `apply` in this instance.  Implement both so you can see the
  difference.

### Step 18. Cleaning Up

You may think the battle has been won at this point, and you are
correct.

  - Refactor `five_rands` and `three_rand_letters` using
    `gen_sequence`.  Do not use `mut`.

We may need to clean up a bit.  When you implemented `gen_sequence`
you will have had to deal with the case where the vector passed to the
function is empty (you *did* handle that didn't you?).  In that case
you will have had to make a `Gen` that held an empty array.

  - Use `pure` for that case if you haven't already.

This is why you need `pure`.

#### Step 19. Traversable

`Applicative` is a natural consequence of wanting to use functors with
functions that have more than one parameter.  Usually `apply` is used
as a kind of stepping stone to get what you actually want.  We saw
that when we implemented `gen_sequence`.

`sequence` converts a vector containing applicative functors to an
applicative functor that contains a vector.  It is normally paired
with another function called `traverse`.

  - Implement `gen_traverse`:  It takes a `fb(A) -> Gen(A)`, a `Vec<A>`
    and returns `Gen<Vec<A>>`.
  - Create a `Traversable` trait with `sequence` and `traverse`.

Note: `traverse` can always be implemented with `sequence` and
vice versa.  Which one you implement first when you are making
a `Traversable` is up to you.

Special Note: This version of `Traversable` is actually incorrect.
`Traversable` works on *any* traversable functor, not just vectors.
However, to keep this already very long kata within bounds, we will
just implement it for vectors.

### Step 20. Chaining functions

Note: This section needs some work to make it more obvious why you
want to do this.

Recall the generators `rand_even` and `rand_odd`.  They are almost the
same.  One of them applies the function `x * 2` and the other applies
the function `x * 2 + 1`.  It would be interesting to define one in
terms of the other.

  - Write a function called `gen_bind` that takes a `Gen<A>`
    and a `Fn(A -> Gen<B>)` and returns a `Gen<B>`.
  - Make a trait called `Monad` that requires `bind` and `pure` and
    give `Gen<T>` that trait.
  - Implement `rand_odd` usinging `rand_even` and `bind`
