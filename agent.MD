In what follows, the prototypes of the function will define generic arguments for each
functions. However, you only have to implement it for real numbers, which will be defined
as the type f32, which is what you should understand when you see K. Do note, however,
that there is a bonus at the end of this module which is to re-do every exercise with the
field K being implemented as your own Complex number type, rather than f32. If you’ve
done your work in a nice and generic way, this shouldn’t prove too much fixing.

Some of the prototypes given in Rust are more "pseudocode" than actual code: a good
linear algebra library in Rust would leverage the Rust trait system, and you would thus
need to specify the traits implemented by your generics in the signatures. Generally,
it should even be expected that the generic types provided would actually have some
form of type constraint in a real implementation (for example, making a Field type, and
having the generic K extend Field; this is especially the case for functional languages).
It is thus expected that the examples in some exercises, as they are, may not compile.
Also, in most of the exercises in this module, we may generally decide for V to be
any sort of vector space, of finite dimension, over its scalar field K. So V , as a generic,
means a set that may contain scalars, or vectors, or matrices, or tensors, etc...
Finally, many of the exercises’ functions store their result within one of their inputs.
This is a choice. This type of function can be much faster when using your object as an
accumulator, but you might want to have the inputs be immutable instead (ie, a "pure
function"), and simply return your value. If your language of choice does not permit such
object-oriented constructs, or if you simply prefer the pure function syntax, you are of
course allowed to instead have a function that returns a scalar/vector/matrix as output,
but keeps its inputs immutable. You may also implement both versions, if you find it
worthwhile: the pure function, and the accumulating function. We do ask some amount
of coherence though: if you choose to implement only one version, stick to your choice
throughout the module.
Conclusion: there is thus some level of freedom of interpretation left to the student
and their evaluator as to verify if the function signature is valid. What is really important
is that the underlying mathematical function is implemented in a way that is:
• generic, so that the overall library of code written during this module may work
with other fields than the real numbers;
• coherent, so that the library has a consistent style and usage;
• faithful, as-in, the function implemented is a true implementation of the required
mathematical concept, and not something else.
In the following exercises, n, m and p each correspond to one size
component of a vector/matrix. This means that O(nm) for an n-by-m
matrix signifies ’no more operations than a constant multiple of the
amount of rows times the amount of columns’.
Unless specified otherwise, all matrices shown in the examples are in
column-major order.