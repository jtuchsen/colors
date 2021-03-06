Colors
======

  Colors is a library that helps you color and style your text. It's a port of the colors
  library from [NodeJS/NPM][NPM colors], mostly undertaken as a learning exercise :).

Examples
--------

```rust
use colors::Style; // Import this to add colors to your string types
println!("{}", "I'm now magically printed in green!".green());
```

  If you prefer to not have the traits added to your &str and strings then just call style
  directly, like so:

```rust
use colors::{Styles, style};
println!("{}", style(Styles::Blue, "I'm now blue!"))
```

  Although with the former style you get a nifty chaining syntax:

```rust
use colors::Style;
println!("{}", "Bolded, underlined, and red!".bold().underline().red());
```

Issues
------

- You may have noticed a problem in the examples, we lose the native string substitution that
  Rusts "println!" macro comes with. This is pretty unfortunate and it looks like it's
  unavoidable. According to the [documentation][format], the compiler requires that the passed
  value be a string literal in order to enforce validity checking. If you know of a work around
  please submit a pull request.

- This whole technique is also pretty un-rusty. Dynamically allocating a bunch Strings in order
  to wrap them in fancy Unicode characters isn't going to be very efficient. As an alternative
  you can use the nifty [term][] crate.

Missing from Port / Todo
------------------------

- Extra (silly) styles
- Color themes
- String substitution, for reasons given above

[NPM colors]: https://www.npmjs.org/package/colors
[format]: http://doc.rust-lang.org/std/fmt/index.html#usage
[term]: http://doc.rust-lang.org/term/term/index.html
