# faustfmt

A formatter using Topiary for the Faust programming language

# Installing

Run `cargo install --path .` in this directory.  
This will install it in `CARGO_HOME/bin`, so make sure to add it to your `PATH`

# Usage

You can pass in your faust code in stdin to `faustfmt` and it will output the formatted code to stdout.  
To format files, you can pass them as arguments to `faustfmt`  
```
Usage: faustfmt [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -f <FORMAT>      Path to alternative Topiary query scheme (.scm)
  -i <INDENT>      String to use for indentation (e.g. '    ') [default: "    "]
  -h, --help       Print help
  -V, --version    Print version
```


## License

This project is released under the terms of the **GNU General Public License, Version 3 (GPLv3) or any later version**.

**Copyright (C) 2025 Ryan Biju Varghese**

You can find the full text of the license in the **`LICENSE`** file at the root of this repository.
