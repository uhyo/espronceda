use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn block_only() {
    assert_stmt_feature("{}", StatementFeature::Block);
}

#[test]
fn block_in_if() {
    assert_stmt_feature("if (true) { let a }", StatementFeature::Block);
}

#[test]
fn block_in_func() {
    assert_stmt_feature(
        "function foo() {
        {
            const abc = 3;
        }
    }",
        StatementFeature::Block,
    );
}

#[test]
fn function_body_is_not_block() {
    assert_no_stmt_feature("function foo() {}", StatementFeature::Block);
}

#[test]
fn arrow_body_is_not_block() {
    assert_no_stmt_feature("const a = () => {}", StatementFeature::Block);
}
