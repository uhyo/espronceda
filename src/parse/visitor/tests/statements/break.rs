use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn continue_no_label() {
    assert_stmt_feature(
        "while (cond) {
            break;
        }",
        StatementFeature::BreakStatement,
    );
}

#[test]
fn continue_with_label() {
    assert_stmt_feature(
        "lbl: while (cond) {
            break lbl;
        }",
        StatementFeature::BreakLabelStatement,
    );
}

#[test]
fn distinct() {
    assert_no_stmt_feature(
        "while (true) { break }",
        StatementFeature::BreakLabelStatement,
    );
    assert_no_stmt_feature(
        "label: for(;;) { break label; }",
        StatementFeature::BreakStatement,
    );
}
