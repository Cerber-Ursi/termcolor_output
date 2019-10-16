use trybuild::TestCases;

#[test]
fn test() {
    let t = TestCases::new();
    t.compile_fail("tests/fails/*.rs");
    t.pass("tests/passes/*.rs");
}
