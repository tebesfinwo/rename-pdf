# Rename PDF

As suggested by its name, `rename-pdf` is binary renames a PDF file based on its title, sort of.

Sometimes, name of a PDF file downloaded from the internet does not relate to its
content at all, like, `123234.pdf`. `rename-pdf` renames automatically for you.

## Caveats / Things I haven't figured out or implemented yet

[ ] The new name is UGLY. If the title of the PDF is "A morning bird", `rename-pdf` will
rename it to `amorningbird.pdf` instead of `a-morning-bird.pdf`

[ ] Code is UGLY.

## Installation

Clone this repository and `cargo build` or `cargo install` yourself.

## License

See [LICENSE-MIT](LICENSE-MIT) for details.
