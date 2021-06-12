use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn class_declaration() {
    assert_stmt_feature(
        "class CLCLCL {
            method() {}
        }",
        StatementFeature::ClassDeclaration,
    )
}

#[test]
fn exported_class_declaration() {
    assert_stmt_feature(
        "export class A {
            method() {}
        }",
        StatementFeature::ClassDeclaration,
    )
}

#[test]
fn anon_class_declaration() {
    assert_stmt_feature(
        "export default class {
            method() {}
        }",
        StatementFeature::AnonymousClassDeclaration,
    )
}

#[test]
fn distinct_class_declaration() {
    assert_no_stmt_feature(
        "export default class {}",
        StatementFeature::ClassDeclaration,
    );
    assert_no_stmt_feature(
        "export class C {}",
        StatementFeature::AnonymousClassDeclaration,
    )
}
