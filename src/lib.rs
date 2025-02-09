use pulldown_latex::config::DisplayMode;
use wasm_bindgen::prelude::wasm_bindgen;

fn convert_math_to_mathml(input: &str, display_mode: DisplayMode) -> String {
    let mut output = String::new();
    let storage = pulldown_latex::Storage::new();
    let parser = pulldown_latex::Parser::new(input, &storage);
    pulldown_latex::push_mathml(
        &mut output,
        parser,
        pulldown_latex::RenderConfig {
            display_mode,
            ..Default::default()
        },
    )
    .unwrap();
    output
}

#[wasm_bindgen]
pub fn display_math_to_mathml(input: &str) -> String {
    convert_math_to_mathml(input, DisplayMode::Block)
}

#[wasm_bindgen]
pub fn inline_math_to_mathml(input: &str) -> String {
    convert_math_to_mathml(input, DisplayMode::Inline)
}
