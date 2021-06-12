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

#[test]
fn class_field() {
    assert_misc_feature(
        "class A {
            field
        }",
        MiscFeature::ClassFieldNoInitializer,
    );
}

#[test]
fn class_field_initializer() {
    assert_misc_feature(
        "class A {
            field = 3;
        }",
        MiscFeature::ClassFieldWithInitializer,
    );
}

#[test]
fn static_class_field() {
    assert_misc_feature(
        "export class Z {
            static field
        }",
        MiscFeature::StaticField,
    );
    assert_misc_feature(
        "export class Z {
            static field
        }",
        MiscFeature::ClassFieldNoInitializer,
    );
}

#[test]
fn static_class_field_initializer() {
    assert_misc_feature(
        "export default class {
            static field = 100;
        }",
        MiscFeature::StaticField,
    );
    assert_misc_feature(
        "export default class {
            static field = this;
        }",
        MiscFeature::ClassFieldWithInitializer,
    );
}
