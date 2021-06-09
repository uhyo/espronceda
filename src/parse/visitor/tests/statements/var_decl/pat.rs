use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;
use crate::parse::visitor::tests::assert_no_misc_feature;

#[test]
fn object_pat() {
    assert_misc_feature("const {} = a;", MiscFeature::ObjectBindingPattern);
}

#[test]
fn array_pat() {
    assert_misc_feature("const [] = arr;", MiscFeature::ArrayBindingPattern);
}

#[test]
fn object_pat_in_func_arg() {
    assert_misc_feature(
        "function foo({ a, b, c }) {}",
        MiscFeature::ObjectBindingPattern,
    );
}

#[test]
fn rest_in_object_pat() {
    assert_misc_feature(
        "var {a, ...rest} = obj;",
        MiscFeature::ObjectRestBindingPattern,
    );
}

#[test]
fn rest_in_array_pat() {
    assert_misc_feature(
        "([a, b, [c, ...rest]] = arr);",
        MiscFeature::ArrayRestBindingPattern,
    );
}

#[test]
fn empty_in_array_pat() {
    assert_misc_feature("const [a, , b] = arr;", MiscFeature::EmptyBindingPattern);
}

#[test]
fn initializer_is_not_pat() {
    assert_no_misc_feature("let [a = {}] = arr;", MiscFeature::ObjectBindingPattern);
}

#[test]
fn empty_array_is_not_empty_pattern() {
    assert_no_misc_feature("const [] = arr", MiscFeature::EmptyBindingPattern);
}

#[test]
fn normal_array_is_not_empty_pattern() {
    assert_no_misc_feature("const [a, b] = arr", MiscFeature::EmptyBindingPattern);
}
