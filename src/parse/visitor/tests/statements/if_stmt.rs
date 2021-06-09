use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn normal() {
    assert_stmt_feature("if (true) {}", StatementFeature::IfStatement);
}

#[test]
fn if_else() {
    assert_stmt_feature("if (true) {} else a;", StatementFeature::IfElseStatement);
}

#[test]
fn distinct() {
    assert_no_stmt_feature("if (true) {}", StatementFeature::IfElseStatement);
    assert_no_stmt_feature("if (true) {} else a;", StatementFeature::IfStatement);
}
