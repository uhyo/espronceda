use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;

#[test]
fn static_method() {
    assert_misc_feature(
        "class A {
            static method() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_generator_method() {
    assert_misc_feature(
        "class A {
            static*method() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_get_method() {
    assert_misc_feature(
        "class A {
            static get [expr]() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_set_method() {
    assert_misc_feature(
        "class A {
            static set abc(n) {}
        }",
        MiscFeature::StaticMethod,
    )
}
