use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn continue_no_label() {
    assert_stmt_feature(
        "with ({ a: 123 }) {
            a = 0;
        }",
        StatementFeature::WithStatement,
    );
}
