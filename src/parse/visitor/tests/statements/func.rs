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

#[test]
fn generator_func_decl() {
    assert_stmt_feature(
        "function* gen(arg) {
            yield* arg;
        }",
        StatementFeature::GeneratorFunctionDeclaration,
    )
}

#[test]
fn anon_generator_func_decl() {
    assert_stmt_feature(
        "export default function*(arg) {
            yield* arg;
        }",
        StatementFeature::AnonymousGeneratorFunctionDeclaration,
    )
}

#[test]
fn export_generator_func_decl() {
    assert_stmt_feature(
        "export function* a(arg) {
        }",
        StatementFeature::GeneratorFunctionDeclaration,
    )
}

#[test]
fn async_generator_func_decl() {
    assert_stmt_feature(
        "async function* gen(arg) {
            yield* (await arg);
        }",
        StatementFeature::AsyncGeneratorFunctionDeclaration,
    )
}

#[test]
fn anon_async_generator_func_decl() {
    assert_stmt_feature(
        "export default async function*(arg) {
            yield* (await arg);
        }",
        StatementFeature::AnonymousAsyncGeneratorFunctionDeclaration,
    )
}

#[test]
fn export_async_generator_func_decl() {
    assert_stmt_feature(
        "export async function* a(arg) {
        }",
        StatementFeature::AsyncGeneratorFunctionDeclaration,
    )
}
