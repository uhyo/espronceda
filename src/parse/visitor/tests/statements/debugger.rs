use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn debugger() {
    assert_stmt_feature(
        "function a() {
            debugger;
        }",
        StatementFeature::DebuggerStatement,
    );
}
