use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;

mod rest_args;

#[test]
fn func_expr() {
    assert_expr_feature(
        "const func = function foo(a) {
        };",
        ExpressionFeature::NamedFunctionExpression,
    )
}

#[test]
fn anon_func_expr() {
    assert_expr_feature(
        "const func = function(a) {
        };",
        ExpressionFeature::AnonymousFunctionExpression,
    )
}

#[test]
fn arrow_func_body() {
    assert_expr_feature(
        "func(
            (foo) => { return foo * 2 }
        );",
        ExpressionFeature::ArrowFunction,
    )
}

#[test]
fn concise_arrow_func_body() {
    assert_expr_feature(
        "func(
            (foo) => foo * 2,
            123
        );",
        ExpressionFeature::ArrowFunctionConcise,
    )
}

#[test]
fn generator_func_expr() {
    assert_expr_feature(
        "const func = function*foo(a) {
        };",
        ExpressionFeature::NamedGeneratorFunctionExpression,
    )
}

#[test]
fn anon_generator_func_expr() {
    assert_expr_feature(
        "const func = function*(a) {
        };",
        ExpressionFeature::AnonymousGeneratorFunctionExpression,
    )
}

#[test]
fn async_generator_func_expr() {
    assert_expr_feature(
        "const func = async function*foo(a) {
        };",
        ExpressionFeature::NamedAsyncGeneratorFunctionExpression,
    )
}

#[test]
fn anon_async_generator_func_expr() {
    assert_expr_feature(
        "const func = async function*(a) {
        };",
        ExpressionFeature::AnonymousAsyncGeneratorFunctionExpression,
    )
}
