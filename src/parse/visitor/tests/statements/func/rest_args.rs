use crate::features::syntax::MiscFeature;
use crate::parse::visitor::tests::assert_misc_feature;

#[test]
fn rest_in_func_decl() {
    assert_misc_feature(
        "function foo(...args) {
            console.log(args);
        }",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_export_func_decl() {
    assert_misc_feature(
        "export function foo(...args) {
            console.log(args);
        }",
        MiscFeature::RestArguments,
    );
}

#[test]
fn rest_in_export_default_func_decl() {
    assert_misc_feature(
        "export default function(...args) {
            console.log(args);
        }",
        MiscFeature::RestArguments,
    );
}
