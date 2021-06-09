use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn explicit() {
    assert_stmt_feature("expr;", StatementFeature::ExpressionStatement);
}

#[test]
fn call() {
    assert_stmt_feature("func()", StatementFeature::ExpressionStatement);
}
