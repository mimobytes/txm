use std::env;

use unicode_width::UnicodeWidthStr;

fn boxed(rendered: &str) -> String {
    let lines: Vec<&str> = rendered.lines().collect();
    let width = lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0);
    let border = "─".repeat(width + 2);
    let mut out = format!("┌{border}┐\n│ {} │\n", " ".repeat(width));
    for line in lines {
        let padding = width - UnicodeWidthStr::width(line);
        out.push_str(&format!("│ {line}{} │\n", " ".repeat(padding)));
    }
    out.push_str(&format!("│ {} │\n└{border}┘\n", " ".repeat(width)));
    out
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: txm [LaTeX input]");
        return;
    }

    let rendered = match txm::render(&args[1]) {
        Ok(rendered) => rendered,
        Err(error) => {
            eprintln!("Failed to render: {error}");
            std::process::exit(1);
        }
    };

    print!("{}", boxed(&rendered));
}
