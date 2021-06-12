use crate::parse::parse_code;
use crate::parse::visitor::ExpressionFeature;
use crate::parse::visitor::MiscFeature;
use crate::parse::visitor::StatementFeature;
use crate::parse::NodeVisitor;
use swc_common::sync::Lrc;
use swc_common::SourceMap;

mod expressions;
mod statements;

fn check_code<S: Into<String>>(code: S) -> NodeVisitor {
    let rm: Lrc<SourceMap> = Default::default();
    parse_code(&rm, code).unwrap()
}

/// Asserts given visitor has given feature.
fn assert_stmt_feature<S: Into<String>>(code: S, feature: StatementFeature) {
    let visitor = check_code(code);
    assert!(visitor.statement_features.contains(&feature))
}

/// Asserts given visitor does not have given feature.
fn assert_no_stmt_feature<S: Into<String>>(code: S, feature: StatementFeature) {
    let visitor = check_code(code);
    assert!(!visitor.statement_features.contains(&feature))
}

/// Asserts given visitor has given feature.
fn assert_expr_feature<S: Into<String>>(code: S, feature: ExpressionFeature) {
    let visitor = check_code(code);
    assert!(visitor.expression_features.contains(&feature))
}

/// Asserts given visitor does not have given feature.
fn assert_no_expr_feature<S: Into<String>>(code: S, feature: ExpressionFeature) {
    let visitor = check_code(code);
    assert!(!visitor.expression_features.contains(&feature))
}

/// Asserts given visitor has given feature.
fn assert_misc_feature<S: Into<String>>(code: S, feature: MiscFeature) {
    let visitor = check_code(code);
    assert!(visitor.misc_features.contains(&feature))
}

/// Asserts given visitor does not have given feature.
fn assert_no_misc_feature<S: Into<String>>(code: S, feature: MiscFeature) {
    let visitor = check_code(code);
    assert!(!visitor.misc_features.contains(&feature))
}
