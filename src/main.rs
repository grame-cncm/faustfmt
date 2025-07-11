use clap::Parser;
use std::io::{self, Read};
use topiary_core::{Language, TopiaryQuery, formatter};
use tree_sitter_faust::LANGUAGE;
use std::str;

#[derive(Parser, Debug)]
#[command(name = "faustfmt")]
#[command(version = "1.0")]
#[command(about = "A formatter for the Faust programming language", long_about = None)]
struct Args {
    files: Vec<String>,
    #[arg(short, help = "Path to alternative Topiary query scheme (.scm)")]
    format: Option<String>,
    #[arg(
        short,
        help = "String to use for indentation (e.g. '    ')",
        default_value = "    "
    )]
    indent: String,
}

pub static DEFAULT_QUERY: &str = r#"
[(comment)] @append_hardline

[(comment) (global_metadata) (function_metadata) (parameters) (string) (fstring)] @leaf


((function_metadata)
  .
  ";"
  @append_hardline) @leaf


["with"
  "="
  ] @prepend_space @append_space


[","] @append_space

(environment
    "{" @append_indent_start @append_hardline @prepend_space
    "}" @prepend_indent_end
  )

(rec_environment
    "{" @append_indent_start @append_hardline @prepend_space
    "}" @prepend_indent_end
  )

(with_environment
  "with" @prepend_hardline @prepend_indent_start
) @append_indent_end

(letrec_environment
  "letrec" @prepend_hardline @prepend_indent_start
) @append_indent_end

[(function_definition) (definition) ";" @append_hardline]

(arguments (_) "," @append_empty_softline)
(arguments (_) . "," @append_indent_start)
(arguments (_)* ",") @append_indent_end

[
 (function_definition)
 (definition)
 (global_metadata)
 (function_metadata)
 (function_call)
 (file_import)
 (comment)
 ] @allow_blank_line_before
"#;

fn main() {
    let args = Args::parse();

    let mut query_content: String = DEFAULT_QUERY.to_string();

    if let Some(format_path) = args.format {
        match std::fs::File::open(&format_path) {
            Ok(mut file) => {
                let mut file_buf = String::new();
                if let Err(e) = file.read_to_string(&mut file_buf) {
                    eprintln!("Error: Could not read format file '{}': {}", format_path, e);
                    std::process::exit(1);
                }
                query_content = file_buf;
            }
            Err(e) => {
                eprintln!(
                    "Error: Could not open format file '{}': {}. Using default format",
                    format_path, e
                );
            }
        }
    }

    if args.files.is_empty() {
        let mut stdin_input = io::stdin();
        let formatted_output = format(&mut stdin_input, args.indent.clone(), &query_content);
        println!("{}", formatted_output);
    } else {
        for file_path in args.files {
            match std::fs::File::open(&file_path) {
                Ok(mut file_input) => {
                    let formatted_output = format(&mut file_input, args.indent.clone(), &query_content);
                    if let Err(e) = std::fs::write(&file_path, formatted_output) {
                        eprintln!("Error writing to file '{}': {}", file_path, e);
                    }
                }
                Err(e) => {
                    eprintln!("Error: Could not open file '{}': {}", file_path, e);
                }
            }
        }
    }
}

fn format(input: &mut impl io::Read, indent: String, query_content: &str) -> String {
    let faust_grammar = tree_sitter_faust::LANGUAGE.into();
    let query = TopiaryQuery::new(&faust_grammar, query_content).unwrap();

    let language: Language = Language {
        name: "faust".to_string(),
        query,
        grammar: LANGUAGE.into(),
        indent: Some(indent.to_string()),
    };

    let mut output_buffer = io::BufWriter::new(Vec::new());

    formatter(
        input,
        &mut output_buffer,
        &language,
        topiary_core::Operation::Format {
            skip_idempotence: true,
            tolerate_parsing_errors: true,
        },
    )
    .unwrap();

    str::from_utf8(output_buffer.buffer()).expect("Formatter output should be valid UTF-8").to_string()
}
