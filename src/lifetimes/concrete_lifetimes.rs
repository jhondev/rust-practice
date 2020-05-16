// conc_lft for values
// a value's lifetime is the time during which the value exists
// at a particular memory address.

// starts when a value is created or moved into a particular
// location in memory, and
// ends when that value is moved out of or dropped from that location.

// This maps directly to the concepts of ownership, moving, and dropping

// A reference's lifetime must be contained within the
// lifetime of the value being referenced. so always point to a valid value.

// In the future, when you get error messages like the ones we saw
// in this module, such as borrowed value does not live long enough,
// remember that it means Rust can't prove that all of your references
// will always be valid. Try looking at where the lifetimes of the values
// and the references are to understand what code needs to be rearranged.
