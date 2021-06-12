use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;
use crate::parse::visitor::tests::assert_no_expr_feature;

#[test]
fn yield_nothing() {
    assert_expr_feature(
        "function* a() {
            const a = yield;
        };",
        ExpressionFeature::YieldNothingExpression,
    )
}

#[test]
fn yield_expr() {
    assert_expr_feature(
        "const f = function*(a) {
            const a = yield a;
        };",
        ExpressionFeature::YieldExpression,
    )
}

#[test]
fn yield_star_expr() {
    assert_expr_feature(
        "const f = function*(a) {
            yield* a;
        };",
        ExpressionFeature::YieldStarExpression,
    )
}

#[test]
fn identifier_is_not_yield_expr() {
    assert_no_expr_feature(
        "function foo() { return yield; }",
        ExpressionFeature::YieldExpression,
    )
}
