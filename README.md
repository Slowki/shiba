[![Crates.io](https://img.shields.io/crates/v/shiba.svg?style=flat-square)](https://crates.io/crates/shiba)
[![license](https://img.shields.io/github/license/mashape/apistatus.svg?style=flat-square)](https://opensource.org/licenses/MIT)

# shiba #
A git hook installer for Cargo inspired by [husky](https://github.com/typicode/husky).

## Usage ##
* Add `shiba` to your dev-dependencies:
```toml
[dev-dependencies]
shiba = "0.1.0"
```
* Create a `.shiba` directory with your git hooks in it
* When Cargo builds the `shiba` dependency it will setup the git hooks in your `.shiba` directory.

## Example Commit Hooks ##
* [Automatic `rustfmt`](.shiba/pre-commit)

## Future Plans ##
Make it less terrible.
