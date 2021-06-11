use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn throw() {
    assert_stmt_feature(
        "function a() {
            throw new Error;
        }",
        StatementFeature::ThrowStatement,
    );
}
