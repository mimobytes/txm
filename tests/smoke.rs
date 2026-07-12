use std::process::Command;

#[test]
fn prints_usage_without_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage:"));
}

#[cfg(not(feature = "fancy"))]
#[test]
fn boxes_simple_identifier() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("x")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌───┐\n│   │\n│ x │\n│   │\n└───┘\n"
    );
}

#[cfg(not(feature = "fancy"))]
#[test]
fn boxes_wide_identifier() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("你")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌────┐\n│    │\n│ 你 │\n│    │\n└────┘\n"
    );
}

#[cfg(not(feature = "fancy"))]
#[test]
fn boxes_adjacent_wide_identifiers() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("你你")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌──────┐\n│      │\n│ 你你 │\n│      │\n└──────┘\n"
    );
}

#[cfg(not(feature = "fancy"))]
#[test]
fn render_returns_raw_lines_for_simple_identifier() {
    let rendered = txm::render("x").expect("render failed");

    assert_eq!(rendered, "x\n");
}

#[test]
fn render_returns_error_for_unclosed_group() {
    assert!(txm::render("{x").is_err());
}

#[test]
fn render_returns_error_for_invalid_lexer_input() {
    assert!(txm::render("@").is_err());
}

#[test]
fn render_returns_error_for_unknown_matrix_environment() {
    assert!(txm::render(r"\begin{unknown}x\end{unknown}").is_err());
}

#[test]
fn render_returns_error_for_ragged_matrix() {
    assert!(txm::render(r"\begin{matrix}a&b\\c\end{matrix}").is_err());
}

#[test]
fn cli_reports_render_errors_without_panicking() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg(r"\begin{unknown}x\end{unknown}")
        .output()
        .expect("failed to run txm");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "error: unknown matrix environment: unknown\n"
    );
}

#[cfg(not(feature = "fancy"))]
#[test]
fn mathbf_maps_to_bold_alphabet() {
    assert_eq!(txm::render(r"\mathbf{x}").unwrap(), "𝐱\n");
}

#[cfg(not(feature = "fancy"))]
#[test]
fn mathbb_uses_letterlike_specials() {
    assert_eq!(txm::render(r"\mathbb{R}").unwrap(), "ℝ\n");
}

#[cfg(not(feature = "fancy"))]
#[test]
fn single_token_argument_needs_no_braces() {
    assert_eq!(txm::render(r"\mathbf n").unwrap(), "𝐧\n");
}

#[cfg(not(feature = "fancy"))]
#[test]
fn accent_stacks_mark_above_argument() {
    assert_eq!(txm::render(r"\hat{x}").unwrap(), "^\nx\n");
}

#[cfg(not(feature = "fancy"))]
#[test]
fn latex_style_parentheses_render_as_paired_delimiters() {
    let rendered = txm::render(r"\left( x \right)").unwrap();

    assert!(rendered.contains('('));
    assert!(rendered.contains(')'));
    assert!(rendered.contains('x'));
}

#[cfg(not(feature = "fancy"))]
#[test]
fn latex_style_brackets_render_fraction_inside() {
    let rendered = txm::render(r"\left[ \frac{1}{2} \right]").unwrap();

    assert!(rendered.contains('[') || rendered.contains('⎡'));
    assert!(rendered.contains(']') || rendered.contains('⎤'));
    assert!(rendered.contains('1'));
    assert!(rendered.contains('2'));
}

#[cfg(not(feature = "fancy"))]
#[test]
fn unmatched_latex_delimiters_fail_gracefully() {
    assert!(txm::render(r"\left( x ").is_err());
}

#[cfg(not(feature = "fancy"))]
#[test]
fn inline_punctuation_renders_literally() {
    assert_eq!(txm::render(r"(3,0)").unwrap(), "(3,0)\n");
}

#[cfg(not(feature = "fancy"))]
#[test]
fn stretchy_brackets_use_side_correct_extensions() {
    let rendered = txm::render(r"\begin{bmatrix}a\\b\\c\end{bmatrix}").unwrap();
    let lines: Vec<&str> = rendered.lines().collect();
    assert!(lines.len() >= 3, "expected tall bracket: {rendered:?}");
    assert!(
        lines[1].starts_with('⎢') && lines[1].ends_with('⎥'),
        "middle row should use left/right bracket pieces: {:?}",
        lines[1]
    );
}

#[cfg(not(feature = "fancy"))]
#[test]
fn pipe_delimiters_render_like_abs() {
    assert_eq!(
        txm::render("|x|").unwrap(),
        txm::render(r"\abs{x}").unwrap()
    );

    assert_eq!(txm::render("|x|").unwrap(), "│x│\n");
}

#[cfg(feature = "fancy")]
#[test]
fn advanced_style_commands_support_hex_colors_and_font_styles() {
    let rendered = txm::render(r"\textcolor{#ff8800}{\textbf{A}}\textit{B}").unwrap();

    assert!(rendered.contains("\u{1b}[1"));
    assert!(rendered.contains("38;2;255;136;0"));
    assert!(rendered.contains("\u{1b}[3"));
}

#[cfg(feature = "fancy")]
#[test]
fn named_colors_and_nested_styles_work_together() {
    let rendered = txm::render(r"\color{orange}{\textit{x}} \textcolor{blue}{y}").unwrap();

    assert!(rendered.contains("38;2;255;165;0"));
    assert!(rendered.contains("34"));
}

#[cfg(feature = "fancy")]
#[test]
fn declaration_style_commands_apply_until_reset() {
    let rendered = txm::render(r"\textbf{A}\textit{B}\normalfont C").unwrap();

    assert!(rendered.contains("\u{1b}[1m"));
    assert!(rendered.contains("\u{1b}[3m"));
    assert!(rendered.contains("\u{1b}[0m"));
    assert!(rendered.contains("C"));
}

#[cfg(feature = "fancy")]
#[test]
fn color_declarations_apply_to_following_content() {
    let rendered = txm::render(r"\color{red}x").unwrap();

    assert!(rendered.contains("31"));
    assert!(rendered.contains("x"));
}
