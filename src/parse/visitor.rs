use crate::features::syntax::ExpressionFeature;
use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use std::collections::HashSet;
use swc_ecma_ast::Decl;
use swc_ecma_ast::ObjectPatProp;
use swc_ecma_ast::Pat;
use swc_ecma_ast::Stmt;
use swc_ecma_ast::VarDecl;
use swc_ecma_ast::VarDeclKind;
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
            Stmt::Decl(decl) => match decl {
                Decl::Var(VarDecl { kind, decls, .. }) => {
                    match kind {
                        VarDeclKind::Var => {
                            self.statement_features
                                .insert(StatementFeature::VariableStatement);
                        }
                        VarDeclKind::Let => {
                            self.statement_features.insert(StatementFeature::LetBinding);
                        }
                        VarDeclKind::Const => {
                            self.statement_features
                                .insert(StatementFeature::ConstBinding);
                        }
                    }
                    for decl in decls {
                        match decl.init {
                            None => {}
                            Some(_) => {
                                self.misc_features.insert(MiscFeature::Initializer);
                            }
                        }
                    }
                }
                _ => {
                    // unimplemented!("Not implemneted yet")
                }
            },
            Stmt::Empty(..) => {
                self.statement_features
                    .insert(StatementFeature::EmptyStatement);
            }
            _ => {
                // unimplemented!("Not implemented yet")
            }
        }
    }
    fn visit_pat(&mut self, pat: &Pat, _parent: &dyn Node) {
        match pat {
            Pat::Object(obj_pat) => {
                self.misc_features.insert(MiscFeature::ObjectBindingPattern);
                for prop in &obj_pat.props {
                    match prop {
                        ObjectPatProp::Rest(..) => {
                            self.misc_features
                                .insert(MiscFeature::ObjectRestBindingPattern);
                        }
                        _ => {}
                    }
                }
            }
            Pat::Array(array_pat) => {
                self.misc_features.insert(MiscFeature::ArrayBindingPattern);
                if array_pat.elems.iter().any(Option::is_none) {
                    self.misc_features.insert(MiscFeature::EmptyBindingPattern);
                }
            }
            Pat::Rest(..) => {
                self.misc_features
                    .insert(MiscFeature::ArrayRestBindingPattern);
            }
            _ => {}
        }
    }
}
