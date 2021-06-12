use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;

#[test]
fn static_method() {
    assert_misc_feature(
        "const cl = class A {
            static method() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_generator_method() {
    assert_misc_feature(
        "const c = class {
            static *method() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_get_method() {
    assert_misc_feature(
        "const a = class A extends B {
            static get [expr]() {}
        }",
        MiscFeature::StaticMethod,
    )
}

#[test]
fn static_set_method() {
    assert_misc_feature(
        "var a = class A {
            static set abc(n) {}
        }",
        MiscFeature::StaticMethod,
    )
}
