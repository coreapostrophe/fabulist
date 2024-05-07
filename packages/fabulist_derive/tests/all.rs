#[test]
fn try_all() {
    let t = trybuild::TestCases::new();
    t.pass("tests/element.rs");
    t.pass("tests/ast.rs");
}
