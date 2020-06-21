#[test]
fn short() {
    assert_eq!(here::doc!(hello world), "hello world");
}

const MULTILINE_TEST: &str = "\
<html>
    <body>
        <h1>Hello</h1>
    </body>
</html>";

#[test]
fn multiline() {
    assert_eq!(
        here::doc! {
            <html>
                <body>
                    <h1>Hello</h1>
                </body>
            </html>
        },
        MULTILINE_TEST,
    );
}

#[test]
fn weird_indenting() {
    assert_eq!(
        here::doc! (<html>
                        <body>
                            <h1>Hello</h1>
                        </body>
                    </html>),
        MULTILINE_TEST,
    );
}
