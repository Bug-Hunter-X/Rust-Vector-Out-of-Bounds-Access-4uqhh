# Rust Vector Out-of-Bounds Access

This repository demonstrates a common error in Rust: attempting to access an element in a vector using an index that is out of bounds.  This results in a runtime panic.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file provides a corrected version that avoids the panic using safe methods.

This example highlights the importance of careful index checking when working with vectors in Rust, to prevent unexpected crashes.