// when we talk about lifetimes, we're talking about
// the length of time that a reference to some value may exist.

// The Rust compiler ensures that all references have valid lifetimes.

// References only exist while the value they point to
// has not been moved to another location or been cleaned up.

// You can do a lot in Rust without having to think about lifetimes
// explicitly. And as long as your code compiles,
// you know that all of your references are valid.

// However, the Rust compiler has limits and occasionally needs
// extra information from us programmers to understand
// how the lifetime of references relate to each other.

// The way we provide the compiler with this information is
// by using generic lifetime parameter annotations.

//  lifetime elision rules. These rules are how the compiler
// understands code when we don't specify lifetime parameters, and
// is why we don't have to annotate every reference.
