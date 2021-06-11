use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_misc_feature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn try_catch() {
    assert_stmt_feature(
        "try {
            func();
        } catch(e) {
            hi;
        }",
        StatementFeature::TryCatchStatement,
    );
}

#[test]
fn try_finally() {
    assert_stmt_feature(
        "try {
            func();
        } finally {
            return;
        }",
        StatementFeature::TryFinallyStatement,
    );
}

#[test]
fn try_catch_finally() {
    assert_stmt_feature(
        "try {
            func();
        } catch {
            console.log('Ahh!');
        } finally {
            return;
        }",
        StatementFeature::TryCatchFinallyStatement,
    );
}

#[test]
fn catch_binding() {
    assert_misc_feature(
        "try {
            func();
        } catch(e) {
            console.log('Ahh!');
        } finally {
            return;
        }",
        MiscFeature::CatchBinding,
    );
}

#[test]
fn catch_no_binding() {
    assert_misc_feature(
        "try {
            func();
        } catch {
            return;
        }",
        MiscFeature::CatchNoBinding,
    );
}
