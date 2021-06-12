use crate::features::syntax::ExpressionFeature;
use crate::parse::visitor::tests::assert_expr_feature;

#[test]
fn named_class_expression() {
    assert_expr_feature(
        "const expr = class Name {};",
        ExpressionFeature::NamedClassExpression,
    )
}

#[test]
fn anon_class_expression() {
    assert_expr_feature(
        "const expr = class {}",
        ExpressionFeature::AnonymousClassExpression,
    )
}
