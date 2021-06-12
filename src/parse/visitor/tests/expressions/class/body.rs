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

#[test]
fn class_field() {
    assert_misc_feature(
        "const cl = class {
            field
        }",
        MiscFeature::ClassFieldNoInitializer,
    );
}

#[test]
fn class_field_initializer() {
    assert_misc_feature(
        "const cl = class A {
            [expr] = 100;
        }",
        MiscFeature::ClassFieldWithInitializer,
    );
}

#[test]
fn static_class_field() {
    assert_misc_feature(
        "const cl = class {
            static field
        }",
        MiscFeature::StaticField,
    );
    assert_misc_feature(
        "const cl = class {
            static field
        }",
        MiscFeature::ClassFieldNoInitializer,
    );
}

#[test]
fn static_class_field_initializer() {
    assert_misc_feature(
        "const cl = class {
            static field = 100;
        }",
        MiscFeature::StaticField,
    );
    assert_misc_feature(
        "const cl = class {
            static field = this;
        }",
        MiscFeature::ClassFieldWithInitializer,
    );
}
