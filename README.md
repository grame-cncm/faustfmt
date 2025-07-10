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
