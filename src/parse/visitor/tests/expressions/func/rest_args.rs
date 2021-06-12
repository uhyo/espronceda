use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;

#[test]
fn rest_in_func_expr() {
    assert_misc_feature(
        "const f = function(a, ...args) {
            console.log(a, args);
        }",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_generator_func_expr() {
    assert_misc_feature(
        "const f = function*(a, ...args) {
            console.log(a, args);
        }",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_async_func_expr() {
    assert_misc_feature(
        "const f = async function(a, ...args) {
            console.log(a, args);
        }",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_arrow_func() {
    assert_misc_feature(
        "const f = (...args) => {
            console.log(args);
        };",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_method() {
    assert_misc_feature(
        "const obj = {
            func(...args) {
                console.log(args);
            }
        };",
        MiscFeature::RestArguments,
    );
}
