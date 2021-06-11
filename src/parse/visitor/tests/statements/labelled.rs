use crate::features::syntax::StatementFeature;
use crate::parse::visitor::tests::assert_stmt_feature;

#[test]
fn labelled() {
    assert_stmt_feature("label: while(true);", StatementFeature::LabelledStatement);
}
