use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_no_stmt_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

mod rest_args;

#[test]
fn func_decl() {
    assert_stmt_feature(
        "function foo() {
        }",
        StatementFeature::FunctionDeclaration,
    )
}

#[test]
fn export_func_decl() {
    assert_stmt_feature(
        "export function foo() {
        }",
        StatementFeature::FunctionDeclaration,
    )
}

#[test]
fn anon_func_decl() {
    assert_stmt_feature(
        "export default function() {
        } 123",
        StatementFeature::AnonymousFunctionDeclaration,
    )
}
