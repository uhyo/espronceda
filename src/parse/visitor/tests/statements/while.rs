use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn normal() {
    assert_stmt_feature(
        "let i = 0; while (i < 10) {i++;}",
        StatementFeature::WhileStatement,
    );
}

#[test]
fn do_while() {
    assert_stmt_feature(
        "do { hi } while (false)",
        StatementFeature::DoWhileStatement,
    );
}

#[test]
fn distinct() {
    assert_no_stmt_feature("while (true);", StatementFeature::DoWhileStatement);
    assert_no_stmt_feature("do {} while (true);", StatementFeature::WhileStatement);
}
