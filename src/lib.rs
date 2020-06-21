#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::{Literal, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn doc(_: TokenStream) -> TokenStream {
    let mut string = String::new();

    if let Some(mut source_text) = Span::call_site().source_text() {
        // Find and remove the start of the call site.
        if let Some(start) = source_text.find(|c| c == '{' || c == '(') {
            source_text = source_text.split_off(start + 1);
        }

        // Remove the ending token of the call site.
        source_text.pop();

        // Remove extra indentation.
        string = unindent(source_text);
    }

    TokenStream::from(TokenTree::from(Literal::string(&string)))
}

fn unindent(indented: impl AsRef<str>) -> String {
    let indented = indented.as_ref();
    let mut string = String::with_capacity(indented.len());

    // Calculate the largest common indentation (in bytes).
    let indent = indented
        .lines()
        .skip(1)
        .filter_map(|line| {
            for (i, byte) in line.bytes().enumerate() {
                if !byte.is_ascii_whitespace() {
                    return Some(i);
                }
            }
            None
        })
        .min()
        .unwrap_or(0);

    let ignore_first_line = indented.starts_with("\n") || indented.starts_with("\r\n");

    for (i, line) in indented.trim_end().lines().enumerate() {
        if i == 0 {
            if ignore_first_line {
                continue;
            } else {
                string.push_str(line);
            }
        } else {
            if i > 1 || !ignore_first_line {
                string.push('\n');
            }

            if line.len() > indent {
                string.push_str(&line[indent..]);
            }
        }
    }

    string
}
