use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;
use crate::parse::visitor::tests::assert_no_expr_feature;

#[test]
fn r#await() {
    assert_expr_feature(
        "async function a() {
            await null;
        };",
        ExpressionFeature::AwaitExpression,
    )
}

#[test]
#[ignore] // TODO: only valid in non-strict-mode code
fn identifier_is_not_await_expr() {
    assert_no_expr_feature(
        "function foo() { return await; }",
        ExpressionFeature::AwaitExpression,
    )
}
