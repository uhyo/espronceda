use crate::features::syntax::{MiscFeature, StatementFeature};
use crate::parse::visitor::tests::assert_misc_feature;
use crate::parse::visitor::tests::assert_no_misc_feature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

mod pat;

#[test]
fn let_only() {
    assert_stmt_feature("let a", StatementFeature::LetBinding);
}

#[test]
fn let_only_init() {
    assert_stmt_feature("let a = b;", StatementFeature::LetBinding);
}

#[test]
fn const_only() {
    assert_stmt_feature("const f = 123", StatementFeature::ConstBinding);
}

#[test]
fn var_only() {
    assert_stmt_feature("var {} = 1;", StatementFeature::VariableStatement);
}

#[test]
fn let_in_block() {
    assert_stmt_feature("{ const foo = 3; }", StatementFeature::ConstBinding);
}

#[test]
fn check_initializer() {
    assert_misc_feature("const foo = abc", MiscFeature::Initializer)
}

#[test]
fn var_is_not_let() {
    assert_no_stmt_feature("var foo;", StatementFeature::LetBinding);
    assert_no_stmt_feature("var foo = 123;", StatementFeature::ConstBinding);
}

#[test]
fn check_no_initializer() {
    assert_no_misc_feature("let a;", MiscFeature::Initializer)
}

#[test]
fn let_in_for() {
    assert_no_stmt_feature(
        "for (let i = 0; i < 10; i++);",
        StatementFeature::LetBinding,
    );
    assert_no_stmt_feature("for (let i of arr);", StatementFeature::LetBinding);
}
