// Elision is a relatively uncommon English word that means
// omitting or leaving out. Lifetime elision is a concept unique to Rust.
// It's a set of rules programmed into the compiler that were added
//  so that you don't have to add generic lifetime parameters on every
// reference.

// The first rule says that each parameter that's a reference is considered to have its own lifetime parameter. For example, if we enter this function that has three parameters, the compiler interprets the first two parameters that are references as having their own lifetime annotations, shown here as 'a and 'b.
// The second rule says that if there's only one lifetime in the parameters, a returned reference gets that lifetime. For example, if we enter this function that has one reference parameter, the first rule says that the reference gets its own lifetime, and the second rule says the returned reference gets that lifetime as well.
// This is because any other association would be invalid unless the return type's lifetime is 'static, as we discussed in the previous module, which we would need to add manually.
// The third rule only applies to methods. It gives return types the same lifetime as the self parameter, even if there are other parameters with different lifetimes.
