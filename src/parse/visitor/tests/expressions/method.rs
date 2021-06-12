use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;

#[test]
fn normal_method() {
    assert_misc_feature(
        "const obj = {
            method() { return 3 },
        }",
        MiscFeature::Method,
    )
}

#[test]
fn generator_method() {
    assert_misc_feature(
        "const obj = {
            *method() { yield 100 },
        }",
        MiscFeature::GeneratorMethod,
    )
}

#[test]
fn async_method() {
    assert_misc_feature(
        "const obj = {
            async method() { await null; },
        }",
        MiscFeature::AsyncMethod,
    )
}

#[test]
fn async_generator_method() {
    assert_misc_feature(
        "const obj = {
            async *method() { await (yield 3); }
        }",
        MiscFeature::AsyncGeneratorMethod,
    )
}

#[test]
fn class_normal_method() {
    assert_misc_feature(
        "class A {
            method() { return 3 }
        }",
        MiscFeature::Method,
    )
}

#[test]
fn class_generator_method() {
    assert_misc_feature(
        "class Z {
            *method() { yield 100 }
        }",
        MiscFeature::GeneratorMethod,
    )
}

#[test]
fn class_async_method() {
    assert_misc_feature(
        "const c = class {
            async method() { await null; }
        }",
        MiscFeature::AsyncMethod,
    )
}

#[test]
fn class_async_generator_method() {
    assert_misc_feature(
        "class A {
            async*method() { await (yield 3); }
        }",
        MiscFeature::AsyncGeneratorMethod,
    )
}
