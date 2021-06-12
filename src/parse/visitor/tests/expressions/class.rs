use crate::features::syntax::ExpressionFeature;
use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_expr_feature;
use crate::parse::visitor::tests::assert_misc_feature;

mod body;

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

#[test]
fn named_class_extends() {
    assert_misc_feature(
        "const expor = class C extends a {};",
        MiscFeature::ExtendsHeritage,
    )
}

#[test]
fn anon_class_extends() {
    assert_misc_feature(
        "const expor = class extends a {};",
        MiscFeature::ExtendsHeritage,
    )
}
