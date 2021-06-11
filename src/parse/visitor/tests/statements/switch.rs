use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_misc_feature;
use crate::parse::visitor::tests::assert_no_misc_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn switch() {
    assert_stmt_feature(
        "switch (b) {
            case true: return false;
            case false: return true;
        }",
        StatementFeature::SwitchStatement,
    );
}

#[test]
fn case_clause() {
    assert_misc_feature(
        "switch (b) {
            case true: return false;
            case false: return true;
        }",
        MiscFeature::CaseClause,
    );
}

#[test]
fn default_clause() {
    assert_misc_feature(
        "switch (b) {
            case true: return false;
            default: {
                return true;
            }
        }",
        MiscFeature::DefaultClause,
    );
}

#[test]
fn distinct() {
    assert_no_misc_feature(
        "switch (0) {
            default: {
                return true;
            }
        }",
        MiscFeature::CaseClause,
    );
    assert_no_misc_feature(
        "switch (0) {
            case 100: {
                return true;
            }
        }",
        MiscFeature::DefaultClause,
    );
}
