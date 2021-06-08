use crate::features::syntax::ExpressionFeature;
use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use std::collections::HashSet;
use swc_ecma_ast::Stmt;
use swc_ecma_visit::Node;
use swc_ecma_visit::VisitAll;

#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
pub struct NodeVisitor {
    pub statement_features: HashSet<StatementFeature>,
    pub expression_features: HashSet<ExpressionFeature>,
    pub misc_features: HashSet<MiscFeature>,
}

impl NodeVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl VisitAll for NodeVisitor {
    fn visit_stmt(&mut self, n: &Stmt, _parent: &dyn Node) {
        match n {
            Stmt::Block(..) => {
                self.statement_features.insert(StatementFeature::Block);
            }
            _ => {
                // unimplemented!("Not implemented yet")
            }
        }
    }
}
