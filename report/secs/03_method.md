# Method

## Testing

Testing in Rust is normally divided in two categories: *unit tests* and *integration tests*. Unit tests are small codes to test specific functions inside the code. These tests are normally written in the same file as the functions themselves, but inside a module annotated with `cfg(test)`.

On the other hand, integration tests are written externally to the library, and is made to test the integration of the functions in the program. These tests are often much larger than unit tests, and are made to make sure that the internal functions work well with each other, from the standpoint of an external user. Therefore, integration tests are normally written in a separate `tests` directory at the same level as the `src` directory.

We will write mainly unit tests in our program, to ensure that our functions return the expected values, and to reduce the mental overhead of debugging when making larger changes to the codebase.

More on testing can be found in the official documentation of the Rust programming language [@Rust-docs-testing].

## Parallelization

![Temporary diagram for visualization](diagrams/metropolis-hastings-tree.jpg)
