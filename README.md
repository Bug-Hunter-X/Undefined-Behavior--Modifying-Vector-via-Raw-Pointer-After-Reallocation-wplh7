# Rust Bug: Undefined Behavior with Raw Pointers and Vector Reallocation

This repository demonstrates a common pitfall in Rust:  undefined behavior resulting from modifying a vector through a raw pointer after the vector might have been reallocated.  This can lead to crashes or unpredictable results.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` provides a safe and correct solution.

**Key Learning:** Avoid using raw pointers to modify vectors unless you have a deep understanding of memory management and are absolutely certain that the vector's underlying memory hasn't been reallocated.
