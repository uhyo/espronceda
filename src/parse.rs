use crate::parse::visitor::NodeVisitor;
use anyhow;
use swc_common::errors::ColorConfig;
use swc_common::errors::Handler;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_common::FileName;
use swc_common::SourceMap;
use swc_ecma_parser::Parser;
use swc_ecma_parser::Syntax;
use swc_ecma_visit::visit_program;
use swc_ecma_visit::All;

use thiserror::Error;

mod visitor;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parse Error")]
    E,
}

pub fn parse_code<S: Into<String>>(
    source_map: &Lrc<SourceMap>,
    code: S,
) -> anyhow::Result<NodeVisitor> {
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    let source_file = source_map.new_source_file(FileName::Anon, code.into());
    let mut parser = Parser::new(
        Syntax::Typescript(Default::default()),
        StringInput::from(&*source_file),
        None,
    );

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let program = parser.parse_program().map_err(|e| {
        e.into_diagnostic(&handler).emit();
        ParseError::E
    })?;

    let visitor = NodeVisitor::new();
    let mut visitor = All { visitor };
    visit_program(&mut visitor, &program, &());

    Ok(visitor.visitor)
}
