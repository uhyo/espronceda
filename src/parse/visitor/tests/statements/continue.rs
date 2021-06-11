use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn continue_no_label() {
    assert_stmt_feature(
        "while (cond) {
            continue;
        }",
        StatementFeature::ContinueStatement,
    );
}

#[test]
fn continue_with_label() {
    assert_stmt_feature(
        "lbl: while (cond) {
            continue lbl;
        }",
        StatementFeature::ContinueLabelStatement,
    );
}

#[test]
fn distinct() {
    assert_no_stmt_feature(
        "while (true) { continue }",
        StatementFeature::ContinueLabelStatement,
    );
    assert_no_stmt_feature(
        "label: for(;;) { continue label; }",
        StatementFeature::ContinueStatement,
    );
}
