use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn for_expr() {
    assert_stmt_feature(
        "for (a = b; true; false) { a++ }",
        StatementFeature::ForExprStatement,
    );
}

#[test]
fn for_expr_empty() {
    assert_stmt_feature("for (;;) return;", StatementFeature::ForExprStatement);
}

#[test]
fn for_var() {
    assert_stmt_feature(
        "for (var i = 0; i < 10; i++) arr.push(i);",
        StatementFeature::ForVarStatement,
    );
}

#[test]
fn for_lexical_let() {
    assert_stmt_feature(
        "for (let i = 0; i < 10; i++) arr.push(i);",
        StatementFeature::ForLexicalStatement,
    );
}

#[test]
fn for_lexical_const() {
    assert_stmt_feature(
        "for (const i = 0;;) { break }",
        StatementFeature::ForLexicalStatement,
    );
}

#[test]
fn for_in_expr() {
    assert_stmt_feature(
        "for (a in obj) { console.log(a) }",
        StatementFeature::ForInExprStatement,
    );
}

#[test]
fn for_in_var() {
    assert_stmt_feature(
        "for (var key in obj) key;",
        StatementFeature::ForInVarStatement,
    );
}

#[test]
fn for_in_lexical_let() {
    assert_stmt_feature(
        "for (let key in obj) key;",
        StatementFeature::ForInLexicalStatement,
    );
}

#[test]
fn for_in_lexical_const() {
    assert_stmt_feature(
        "for (const key in obj) { break }",
        StatementFeature::ForInLexicalStatement,
    );
}

#[test]
fn for_of_expr() {
    assert_stmt_feature(
        "for (a of obj) { console.log(a) }",
        StatementFeature::ForOfExprStatement,
    );
}

#[test]
fn for_of_var() {
    assert_stmt_feature(
        "for (var key of obj) key;",
        StatementFeature::ForOfVarStatement,
    );
}

#[test]
fn for_of_lexical_let() {
    assert_stmt_feature(
        "for (let key of obj) key;",
        StatementFeature::ForOfLexicalStatement,
    );
}

#[test]
fn for_of_lexical_const() {
    assert_stmt_feature(
        "for (const key of obj) { break }",
        StatementFeature::ForOfLexicalStatement,
    );
}

#[test]
fn for_await_of_expr() {
    assert_stmt_feature(
        "for await (a of obj) { console.log(a) }",
        StatementFeature::ForAwaitOfExprStatement,
    );
}

#[test]
fn for_await_of_var() {
    assert_stmt_feature(
        "for await (var key of obj) key;",
        StatementFeature::ForAwaitOfVarStatement,
    );
}

#[test]
fn for_await_of_lexical_let() {
    assert_stmt_feature(
        "for await(let key of obj) key;",
        StatementFeature::ForAwaitOfLexicalStatement,
    );
}

#[test]
fn for_await_of_lexical_const() {
    assert_stmt_feature(
        "for await (const key of obj) { break }",
        StatementFeature::ForAwaitOfLexicalStatement,
    );
}
