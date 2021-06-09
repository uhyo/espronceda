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
}
