<h1 align="center">
  <img src="assets/all-the-things.png" alt="All the markdowns" width="300">
  <br>
  All The Markdowns!
  <br>
</h1>

This is a [`mdbook` preprocessor][preprocessor] that will walk a specified base path and add all
the markdowns to your `mdbook`. This is pretty naive and will probably work best
for smaller projects but feel free to give it a go for any folder structure!

## Example

Given the following folder structure:

```sh
.
├── my-libraries
│   └── lib-biz
│       ├── INSTALLATION.md
│       ├── README.md
│       ├── sub-lib-a
│       │   └── README.md
│       └── sub-lib-b
│           └── README.md
└── my-services
    ├── service-bar
    │   └── README.md
    └── service-foo
        └── README.md
```

Based on the title in each document, combined with the folder names as title
case, the preprocessor would render the following:

![example](./assets/example-index.png)

## Test

You can test rendering any of the example folders in this directory with the
book found in [`examples/book1`][book1] with `mdbook serve examples/book1`.

  [book1]: ./examples/book1/
  [preprocessor]: https://rust-lang.github.io/mdBook/for_developers/preprocessors.htmlu
