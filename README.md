<h1 align="center">
  <img src="assets/all-the-things.png" alt="All the markdowns" width="300">
  <br>
  All The Markdowns!
  <br>
</h1>

<div align="center">
  <a href="https://github.com/bombsimon/mdbook-all-the-markdowns/actions/workflows/rust.yml">
    <img src="https://github.com/bombsimon/mdbook-all-the-markdowns/actions/workflows/rust.yml/badge.svg">
  </a>
  <a href="https://crates.io/crates/mdbook-all-the-markdowns">
    <img src="https://img.shields.io/crates/v/mdbook-all-the-markdowns.svg">
  </a>
</div>

This is a [`mdbook` preprocessor][preprocessor] that will walk a specified base path and add all
the markdowns to your `mdbook`. This is pretty naive and will probably work best
for smaller projects but feel free to give it a go for any folder structure!

## Configuration

The preprocessor can be configured with the following settings:

```toml
[preprocessor.all-the-markdowns]
# This will mark all folders generated for a proper index as draft, making them
# non clickable. By default this is false and the content will just be the name
# of the folder as title.
draft-folders = true

[[preprocessor.all-the-markdowns.section]]
# The title to use in the index on the left. Can be useful if book also consist
# of static content or if creating multiple sections.
title = "Auto generated"

# The base directory to find markdowns in, this is usually the root of your
# project if you're only creating a single section.
base = "./examples/example-folder-structures/slim"

# Paths to ignore. No matter where you set your base you can always ignore given
# patterns. These needs to be relative to the base since the directory traverser
# will match if a file or director _starts with_ any of these patterns.
ignore = []
```

Since `section` is a list of tables you can add multiple groups by adding
multiple sections.

```toml
[preprocessor.all-the-markdowns]

[[preprocessor.all-the-markdowns.section]]
title = "Libraries"
base = "./examples/example-folder-structures/slim/my-libraries"
ignore = []

[[preprocessor.all-the-markdowns.section]]
title = "Services"
base = "./examples/example-folder-structures/slim/my-services"
ignore = []
```

## Example

Given the following folder structure:

```sh
.
├── my-libraries
│   └── lib-biz
│       ├── INSTALLATION.md
│       ├── README.md
│       ├── sub-lib-a
│       │   ├── CONTRIBUTORS.md
│       │   ├── README.md
│       │   └── USAGE.md
│       └── sub-lib-b
│           └── README.md
└── my-services
    ├── service-bar
    │   ├── README.md
    │   └── infra
    │       └── kubernetes
    │           └── README.md
    └── service-foo
        └── README.md
```

Based on the title in each document, combined with the folder names as title
case, the preprocessor with the first configuration would render the following:

<table>
  <tr>
    <td width="440" align="center"><img src="./assets/example-index.png"></td>
    <td width="440" align="center"><img src="./assets/example-index-2.png"></td>
  </tr>
  <tr>
    <td align="center">Result with the first example with one section.</td>
    <td align="center">Result with the second example with multiple sections.</td>
  </tr>
</table>

## Test

You can test rendering any of the example folders in this directory with the
book found in [`examples/book1`][book1] with `mdbook serve examples/book1`.

  [book1]: ./examples/book1/
  [preprocessor]: https://rust-lang.github.io/mdBook/for_developers/preprocessors.htmlu
