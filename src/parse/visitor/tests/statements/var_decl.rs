use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

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
#[ignore]
fn let_in_for() {
    assert_stmt_feature("for (let i of arr);", StatementFeature::LetBinding);
}

#[test]
fn let_in_block() {
    assert_stmt_feature("{ const foo = 3; }", StatementFeature::ConstBinding);
}

#[test]
fn var_is_not_let() {
    assert_no_stmt_feature("var foo;", StatementFeature::LetBinding);
    assert_no_stmt_feature("var foo = 123;", StatementFeature::ConstBinding);
}
