# Here

A dumb unquoted Heredoc macro for Rust just because I can. Don't use it, its not worth the effort just to omit quotes from your string literals.

Requires nightly.

## Usage

This:

```rust
fn my_html() -> &'static str {
    here::doc! {
        <html>
            <body>
                <h1>Hello</h1>
            </body>
        </html>
    }
}
```

gets turned into this (more or less):


```rust
fn my_html() -> &'static str {
r#"<html>
    <body>
        <h1>Hello</h1>
    </body>
</html>"#
}
```

De-indentation is inferred automatically and all other whitespace is preserved. For certain scenarios depending on the text, having the text not wrapped in quotes might look better in your editor.

## How does it work?

The macro completely ignores the token stream given to it and instead generates a string from the source code file located at the call site. Yuck! Not to mention probably breaks incremental compilation.

## License

This project's source code and documentation is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
