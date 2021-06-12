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
    // Function declaration
    FunctionDeclaration,
    // anonymous function declaration (only possible as part of export default)
    AnonymousFunctionDeclaration,
    // generator function declaration
    GeneratorFunctionDeclaration,
    AnonymousGeneratorFunctionDeclaration,
    // async generator function declaration
    AsyncGeneratorFunctionDeclaration,
    AnonymousAsyncGeneratorFunctionDeclaration,
    // class name { ... }
    ClassDeclaration,
    // anonymous class declaration
    AnonymousClassDeclaration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionFeature {
    /// function name() {...}
    NamedFunctionExpression,
    /// function() {...}
    AnonymousFunctionExpression,
    /// () => {...}
    ArrowFunction,
    /// () => expr
    ArrowFunctionConcise,
    /// function* name() { ... }
    NamedGeneratorFunctionExpression,
    /// function*() { ... }
    AnonymousGeneratorFunctionExpression,
    /// yield
    YieldNothingExpression,
    /// yield expr
    YieldExpression,
    /// yield* expr
    YieldStarExpression,
    /// async function* name() {}
    NamedAsyncGeneratorFunctionExpression,
    /// async function*() {}
    AnonymousAsyncGeneratorFunctionExpression,
    /// class name { ... }
    NamedClassExpression,
    /// class { ... }
    AnonymousClassExpression,
}

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
    /// ...args in function arguments
    RestArguments,
    /// method(...){...}
    Method,
    /// *method
    GeneratorMethod,
    /// async method(){...}
    AsyncMethod,
    /// async *method(){...}
    AsyncGeneratorMethod,
    /// get prop() { ... }
    GetterProp,
    /// set prop(n) { ... }
    SetterProp,
    /// extends expr
    ExtendsHeritage,
    /// static method() {} in class
    StaticMethod,
}
