use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn explicit() {
    assert_stmt_feature(";", StatementFeature::EmptyStatement);
}

#[test]
fn in_while() {
    assert_stmt_feature("while(true);", StatementFeature::EmptyStatement);
}

#[test]
fn no_statement_is_not_empty_statement() {
    assert_no_stmt_feature("", StatementFeature::EmptyStatement);
}

#[test]
fn empty_block_is_not_empty_statement() {
    assert_no_stmt_feature("if (true) {}", StatementFeature::EmptyStatement);
}
