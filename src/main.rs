extern crate espronceda;
use espronceda::parse::parse_code;
use swc_common::sync::Lrc;
use swc_common::SourceMap;

fn main() -> anyhow::Result<()> {
    let source_map: Lrc<SourceMap> = Default::default();
    parse_code(&source_map, "function a() {}")?;

    Ok(())
}
