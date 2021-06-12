use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_misc_feature;
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
fn export_default_named_class_declaration() {
    assert_stmt_feature(
        "export default class C {
            method() {}
        }",
        StatementFeature::ClassDeclaration,
    )
}

#[test]
fn class_decl_extends() {
    assert_misc_feature(
        "class CL extends Base {
            field;
        }",
        MiscFeature::ExtendsHeritage,
    );
}

#[test]
fn export_class_decl_extends() {
    assert_misc_feature(
        "export class CL extends Base {}",
        MiscFeature::ExtendsHeritage,
    );
}

#[test]
fn export_default_class_decl_extends() {
    assert_misc_feature(
        "export default class extends Base {}",
        MiscFeature::ExtendsHeritage,
    );
}

#[test]
fn export_default_named_class_decl_extends() {
    assert_misc_feature(
        "export default class A extends Base {}",
        MiscFeature::ExtendsHeritage,
    );
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
