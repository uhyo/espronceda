use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn continue_no_label() {
    assert_stmt_feature(
        "function a() {
            return;
        }",
        StatementFeature::ReturnNothingStatement,
    );
}

#[test]
fn continue_with_label() {
    assert_stmt_feature(
        "const a = function() {
            return 123;
        }",
        StatementFeature::ReturnExprStatement,
    );
}

#[test]
fn distinct() {
    assert_no_stmt_feature(
        "const abc = () => { return };",
        StatementFeature::ReturnExprStatement,
    );
    assert_no_stmt_feature(
        "function foo() { return undefined; }",
        StatementFeature::ReturnNothingStatement,
    );
}
