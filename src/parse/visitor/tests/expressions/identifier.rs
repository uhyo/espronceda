use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;
use crate::parse::visitor::tests::assert_no_expr_feature;

#[test]
fn identifier_expr() {
    assert_expr_feature("id;", ExpressionFeature::Identifier);
}

#[test]
fn identifier_yield() {
    assert_expr_feature(
        "function foo() { return yield }",
        ExpressionFeature::Identifier,
    );
}

#[test]
#[ignore] // swc bug?
fn identifier_await() {
    assert_expr_feature(
        "function foo() { return await }",
        ExpressionFeature::Identifier,
    );
}

#[test]
fn identifier_as_default_expr() {
    assert_expr_feature("const { foo = bar } = {};", ExpressionFeature::Identifier);
}

#[test]
fn binding_is_not_identifier() {
    assert_no_expr_feature("let foo = 3;", ExpressionFeature::Identifier);
}
