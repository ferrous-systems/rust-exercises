# Ferrous Systems' Rust Exercises

This is free exercise material produced by Ferrous Systems for trainings. See [ferrous-systems.com/training](https://ferrous-systems.com/training) for more details or a custom quote. You can view this material on-line at <https://rust-exercises.ferrous-systems.com>.

It accompanies the Slide decks we use for training, which you can view on-line at <https://rust-training.ferrous-systems.com>.

The material is created for people with anywhere from zero Rust experience (but with a programming background) to advanced Rust knowledge.

Ferrous Systems offers a large Rust curriculum for both beginner and advanced Rust developers.

Ferrous Systems specialises in custom, topic focused workshops for enterprises starting or expanding their use of Rust. Supplemental to our courses, we offer ongoing help and feedback.

## Overview

The materials are presented as a set of small, self-contained lessons on a specific topic. Note that lessons might be revised, extended, or removed, when necessary to keep material up to date or relevant to new audiences.

We assemble these lessons into various programmes for our commercial trainings. We can also provide custom lessons - please [reach out](https://ferrous-systems.com/contact) if that is of interest.

## Reading the material

This material is organised as an [`mdbook`](https://crates.io/crates/mdbook), along with source code in the form of worked solutions and/or starting templates.

You can:

* View the `main` branch on-line at <https://rust-exercises.ferrous-systems.com/latest/book>
* View a specific tag like `v1.10.0` at <https://rust-exercises.ferrous-systems.com/v1.10.0/book>
* Browse [the chapters of the book on Github](./exercise-book/src/SUMMARY.md)
* Browse [the starting templates on Github](./exercise-templates)
* Browse [the worked solutions on Github](./exercise-solutions)
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
cd ./exercise-book
mdbook test
```

To verify that all the examples build, we use the top-level build script `build.sh`. Note that running that script may require a valid CriticalUp token, as we use Ferrocene in our CI.

## Credits

The development of this course is financed by Ferrous Systems. They are open sourced as a contribution to the growth of the Rust language.

If you wish to fund further development of the course, [book a training](https://ferrous-systems.com/training)!

## License

[![Creative Commons License](https://i.creativecommons.org/l/by-sa/4.0/88x31.png)](http://creativecommons.org/licenses/by-sa/4.0/)

This work is licensed under a [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/).

We encourage the use of this material, under the terms of the above license, in the production and/or delivery of commercial or open-source Rust training programmes.

We unfortunately cannot accept copyrightable contributions from third parties unless they enter into a Contributors' License Agreement with us. Please contact <training@ferrous-systems.com> if you would like to collaborate with us on this material.

Copyright (c) Ferrous Systems, 2025
