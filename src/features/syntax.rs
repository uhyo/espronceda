#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatementFeature {
    /// {}
    Block,
    /// let ...
    LetBinding,
    /// const ...
    ConstBinding,
    /// var ...
    VariableStatement,
    /// ;
    EmptyStatement,
    /// expr;
    ExpressionStatement,
    // if (...) { ... }
    IfStatement,
    // if-else
    IfElseStatement,
    // while
    WhileStatement,
    // do-while
    DoWhileStatement,
    // for (expr; ...; ...)
    ForExprStatement,
    // for (var ...; ...; ...)
    ForVarStatement,
    // for (let ...; ...; ...)
    ForLexicalStatement,
    // for (expr in ...)
    ForInExprStatement,
    // for (var pat in ...)
    ForInVarStatement,
    // for (let pat in ...)
    ForInLexicalStatement,
    // for (expr of ...)
    ForOfExprStatement,
    // for (var pat of ...)
    ForOfVarStatement,
    // for (let pat of ...)
    ForOfLexicalStatement,
    // for await (expr of ...)
    ForAwaitOfExprStatement,
    // for await (var pat of ...)
    ForAwaitOfVarStatement,
    // for await (let pat of ...)
    ForAwaitOfLexicalStatement,
    // continue;
    ContinueStatement,
    // continue label;
    ContinueLabelStatement,
    // break;
    BreakStatement,
    // break label;
    BreakLabelStatement,
    // return;
    ReturnNothingStatement,
    // return expr;
    ReturnExprStatement,
    // with (expr) statement
    WithStatement,
    // switch (expr) { ... }
    SwitchStatement,
    // label: stmt
    LabelledStatement,
    // throw expr;
    ThrowStatement,
    // try-catch
    TryCatchStatement,
    // try-finally
    TryFinallyStatement,
    // try-catch-finally
    TryCatchFinallyStatement,
    // debugger;
    DebuggerStatement,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionFeature {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MiscFeature {
    /// = expr
    Initializer,
    /// BindingIdentifier
    BindingIdentifier,
    /// { ... }
    ObjectBindingPattern,
    /// [...]
    ArrayBindingPattern,
    /// ...rest in object binding pattern
    ObjectRestBindingPattern,
    /// ...rest in array binding pattern
    ArrayRestBindingPattern,
    /// , /* here */ in array binding pattern
    EmptyBindingPattern,
    /// case expr: ... in switch
    CaseClause,
    /// default: ... in switch
    DefaultClause,
    // catch(e) in try
    CatchBinding,
    // catch in try
    CatchNoBinding,
}
