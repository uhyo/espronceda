#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatementFeature {
    /// {}
    Block,
    /// let ...
    LetBinding,
    /// const ...
    ConstBinding,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionFeature {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MiscFeature {}
