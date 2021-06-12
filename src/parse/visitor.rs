use crate::features::syntax::ExpressionFeature;
use crate::features::syntax::MiscFeature;
use crate::features::syntax::StatementFeature;
use std::collections::HashSet;
use swc_ecma_ast::BlockStmtOrExpr;
use swc_ecma_ast::ClassMember;
use swc_ecma_ast::Decl;
use swc_ecma_ast::DefaultDecl;
use swc_ecma_ast::Expr;
use swc_ecma_ast::Function;
use swc_ecma_ast::MethodKind;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_ast::ObjectPatProp;
use swc_ecma_ast::Pat;
use swc_ecma_ast::Prop;
use swc_ecma_ast::PropOrSpread;
use swc_ecma_ast::Stmt;
use swc_ecma_ast::VarDecl;
use swc_ecma_ast::VarDeclKind;
use swc_ecma_ast::VarDeclOrExpr;
use swc_ecma_ast::VarDeclOrPat;
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

    fn visit_vardecl_or_pat(
        &mut self,
        vdp: &VarDeclOrPat,
        (expr, var, lexical): (StatementFeature, StatementFeature, StatementFeature),
    ) {
        match vdp {
            VarDeclOrPat::Pat(..) => {
                self.statement_features.insert(expr);
            }
            VarDeclOrPat::VarDecl(ref var_decl) => match var_decl.kind {
                VarDeclKind::Var => {
                    self.statement_features.insert(var);
                }
                VarDeclKind::Let | VarDeclKind::Const => {
                    self.statement_features.insert(lexical);
                }
            },
        }
    }

    fn visit_decl(&mut self, decl: &Decl) {
        match decl {
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
            Decl::Fn(..) => {
                self.statement_features
                    .insert(StatementFeature::FunctionDeclaration);
            }
            Decl::Class(..) => {
                // unimplemented!("Not implemneted yet")
            }
            _ => {
                // unimplemented!("Not implemneted yet")
            }
        }
    }

    fn visit_method_func(&mut self, func: &Function) {
        self.misc_features
            .insert(match (func.is_generator, func.is_async) {
                (false, false) => MiscFeature::Method,
                (true, false) => MiscFeature::GeneratorMethod,
                (false, true) => MiscFeature::AsyncMethod,
                (true, true) => MiscFeature::AsyncGeneratorMethod,
            });
    }
}

impl VisitAll for NodeVisitor {
    fn visit_stmt(&mut self, n: &Stmt, _parent: &dyn Node) {
        match n {
            Stmt::Block(..) => {
                self.statement_features.insert(StatementFeature::Block);
            }
            Stmt::Decl(decl) => {
                self.visit_decl(decl);
            }
            Stmt::Empty(..) => {
                self.statement_features
                    .insert(StatementFeature::EmptyStatement);
            }
            Stmt::Expr(..) => {
                self.statement_features
                    .insert(StatementFeature::ExpressionStatement);
            }
            Stmt::If(if_stmt) => {
                self.statement_features.insert(if if_stmt.alt.is_none() {
                    StatementFeature::IfStatement
                } else {
                    StatementFeature::IfElseStatement
                });
            }
            Stmt::While(..) => {
                self.statement_features
                    .insert(StatementFeature::WhileStatement);
            }
            Stmt::DoWhile(..) => {
                self.statement_features
                    .insert(StatementFeature::DoWhileStatement);
            }
            Stmt::For(for_stmt) => {
                match for_stmt.init {
                    None | Some(VarDeclOrExpr::Expr(..)) => {
                        // empty or expr
                        self.statement_features
                            .insert(StatementFeature::ForExprStatement);
                    }
                    Some(VarDeclOrExpr::VarDecl(ref var_decl)) => match var_decl.kind {
                        VarDeclKind::Var => {
                            self.statement_features
                                .insert(StatementFeature::ForVarStatement);
                        }
                        VarDeclKind::Let | VarDeclKind::Const => {
                            self.statement_features
                                .insert(StatementFeature::ForLexicalStatement);
                        }
                    },
                }
            }
            Stmt::ForIn(for_stmt) => self.visit_vardecl_or_pat(
                &for_stmt.left,
                (
                    StatementFeature::ForInExprStatement,
                    StatementFeature::ForInVarStatement,
                    StatementFeature::ForInLexicalStatement,
                ),
            ),
            Stmt::ForOf(for_stmt) => self.visit_vardecl_or_pat(
                &for_stmt.left,
                if for_stmt.await_token.is_none() {
                    (
                        StatementFeature::ForOfExprStatement,
                        StatementFeature::ForOfVarStatement,
                        StatementFeature::ForOfLexicalStatement,
                    )
                } else {
                    (
                        StatementFeature::ForAwaitOfExprStatement,
                        StatementFeature::ForAwaitOfVarStatement,
                        StatementFeature::ForAwaitOfLexicalStatement,
                    )
                },
            ),
            Stmt::Continue(continue_stmt) => {
                self.statement_features
                    .insert(if continue_stmt.label.is_some() {
                        StatementFeature::ContinueLabelStatement
                    } else {
                        StatementFeature::ContinueStatement
                    });
            }
            Stmt::Break(break_stmt) => {
                self.statement_features
                    .insert(if break_stmt.label.is_some() {
                        StatementFeature::BreakLabelStatement
                    } else {
                        StatementFeature::BreakStatement
                    });
            }
            Stmt::Return(return_stmt) => {
                self.statement_features
                    .insert(if return_stmt.arg.is_some() {
                        StatementFeature::ReturnExprStatement
                    } else {
                        StatementFeature::ReturnNothingStatement
                    });
            }
            Stmt::With(..) => {
                self.statement_features
                    .insert(StatementFeature::WithStatement);
            }
            Stmt::Switch(switch_stmt) => {
                self.statement_features
                    .insert(StatementFeature::SwitchStatement);
                for case in switch_stmt.cases.iter() {
                    self.misc_features.insert(if case.test.is_some() {
                        MiscFeature::CaseClause
                    } else {
                        MiscFeature::DefaultClause
                    });
                }
            }
            Stmt::Labeled(..) => {
                self.statement_features
                    .insert(StatementFeature::LabelledStatement);
            }
            Stmt::Throw(..) => {
                self.statement_features
                    .insert(StatementFeature::ThrowStatement);
            }
            Stmt::Try(try_stmt) => {
                self.statement_features
                    .insert(match (&try_stmt.handler, &try_stmt.finalizer) {
                        (None, None) => {
                            panic!("try without catch nor finally");
                        }
                        (Some(..), None) => StatementFeature::TryCatchStatement,
                        (None, Some(..)) => StatementFeature::TryFinallyStatement,
                        (Some(..), Some(..)) => StatementFeature::TryCatchFinallyStatement,
                    });
                try_stmt.handler.iter().for_each(|h| {
                    self.misc_features.insert(if h.param.is_some() {
                        MiscFeature::CatchBinding
                    } else {
                        MiscFeature::CatchNoBinding
                    });
                });
            }
            Stmt::Debugger(..) => {
                self.statement_features
                    .insert(StatementFeature::DebuggerStatement);
            }
        }
    }
    fn visit_expr(&mut self, expr: &Expr, _parent: &dyn Node) {
        match expr {
            Expr::Fn(fn_expr) => {
                self.expression_features.insert(if fn_expr.ident.is_some() {
                    ExpressionFeature::NamedFunctionExpression
                } else {
                    ExpressionFeature::AnonymousFunctionExpression
                });
            }
            Expr::Arrow(arrow_expr) => {
                self.expression_features.insert(match arrow_expr.body {
                    BlockStmtOrExpr::BlockStmt(..) => ExpressionFeature::ArrowFunction,
                    BlockStmtOrExpr::Expr(..) => ExpressionFeature::ArrowFunctionConcise,
                });
                for pat in arrow_expr.params.iter() {
                    if let Pat::Rest(..) = pat {
                        self.misc_features.insert(MiscFeature::RestArguments);
                    }
                }
            }
            Expr::Object(obj_expr) => {
                for p in obj_expr.props.iter() {
                    match p {
                        PropOrSpread::Prop(prop) => {
                            match prop.as_ref() {
                                Prop::Method(method) => {
                                    self.visit_method_func(&method.function);
                                }
                                Prop::Getter(..) => {
                                    self.misc_features.insert(MiscFeature::GetterProp);
                                }
                                Prop::Setter(..) => {
                                    self.misc_features.insert(MiscFeature::SetterProp);
                                }
                                _ => {
                                    // unimplemented!("Unimplemented")
                                }
                            }
                        }
                        PropOrSpread::Spread(..) => {
                            // unimplemented!("Unimplemented")
                        }
                    }
                }
            }
            Expr::Class(class_expr) => for member in class_expr.class.body.iter() {},
            _ => {
                // unimplemented!("Unimplemented");
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
    fn visit_module_decl(&mut self, decl: &ModuleDecl, _parent: &dyn Node) {
        match decl {
            ModuleDecl::ExportDecl(decl) => {
                self.visit_decl(&decl.decl);
            }
            ModuleDecl::ExportDefaultDecl(decl) => {
                match decl.decl {
                    DefaultDecl::Fn(ref fn_decl) => {
                        if fn_decl.ident.is_none() {
                            // export default function
                            self.statement_features
                                .insert(StatementFeature::AnonymousFunctionDeclaration);
                        }
                    }
                    _ => {}
                };
            }
            _ => {
                // unimplemented!("Not Implemented")
            }
        }
    }
    fn visit_function(&mut self, func: &Function, _parent: &dyn Node) {
        for param in func.params.iter() {
            match param.pat {
                Pat::Rest(..) => {
                    self.misc_features.insert(MiscFeature::RestArguments);
                }
                _ => {}
            }
        }
    }
    fn visit_class_member(&mut self, member: &ClassMember, _parent: &dyn Node) {
        match member {
            ClassMember::Method(method) => {
                match method.kind {
                    MethodKind::Getter => {
                        self.misc_features.insert(MiscFeature::GetterProp);
                    }
                    MethodKind::Setter => {
                        self.misc_features.insert(MiscFeature::SetterProp);
                    }
                    MethodKind::Method => {}
                }
                self.visit_method_func(&method.function);
            }
            _ => {
                // unimplemented!("Unimplemented");
            }
        }
    }
}
