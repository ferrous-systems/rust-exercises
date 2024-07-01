# Rust Exercises

This book contains a collection of Rust Exercises, written by Ferrous Systems. See [ferrous-systems.com/training](https://ferrous-systems.com/training) for more details or a custom quote. You can view this material on-line at <https://rust-exercises.ferrous-systems.com>.

We use these exercises as part of our [Rust Training](https://ferrous-systems.com/training/), but you are welcome to try them for yourself as well.

If you wish to fund further development of the course, why not [book a training with us](https://ferrous-systems.com/training/)!

# Reading the book

You can:

* View the `main` branch on-line at <https://rust-exercises.ferrous-systems.com/latest/book>
* View a specific tag like `v1.10.0` at <https://rust-exercises.ferrous-systems.com/v1.10.0/book>
* Browse [the chapters of the book on Github](./exercise-book/src/SUMMARY.md)
* Browse [the exercises on Github](./exercise-templates)
* Browse [the solutions on Github](./exercise-solutions)
* Clone the repo, and build the book (see [Building the material locally](#building-the-material-locally))
* Download the book in HTML format for off-line use, with solutions and templates, from the [releases area](https://github.com/ferrous-systems/rust-exercises/releases)

## Building the material locally

To build the exercise book, run `mdbook` in the usual fashion:

```console
$ cargo install mdbook
$ cargo install mdbook-mermaid
$ cd ./exercise-book
$ mdbook build
2024-07-01 11:51:21 [INFO] (mdbook::book): Book building has started
2024-07-01 11:51:21 [INFO] (mdbook::book): Running the html backend
$ ls -l book/index.html
-rw-r--r--  1 jonathan  staff  27975  1 Jul 11:51 book/index.html
```

You could use `mdbook serve` instead to start a webserver that serves up the book and also rebuilds the book every time you change a file.

To verify that every code example in the book compiles as expected, run:

```sh
mdbook test
```

To verify that all the examples build, we use the top-level build script `build.sh`. Note that running that script may require a valid CriticalUp token, as we use Ferrocene in our CI.

## License

[![Creative Commons License](https://i.creativecommons.org/l/by-sa/4.0/88x31.png)](http://creativecommons.org/licenses/by-sa/4.0/)

This work is licensed under a [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/).

We encourage the use of this material, under the terms of the above license, in the production and/or delivery of commercial or open-source Rust training programmes.

Copyright (c) Ferrous Systems, 2024
