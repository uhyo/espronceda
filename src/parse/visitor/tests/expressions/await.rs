use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;

#[test]
fn r#await() {
    assert_expr_feature(
        "async function a() {
            await null;
        };",
        ExpressionFeature::AwaitExpression,
    )
}
