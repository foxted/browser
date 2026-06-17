//! Diagnostic: run a page's full script set (inline + external) and show remaining errors.
//! Usage: cargo run -p engine --example dump -- <file.html> <base-url>

fn main() {
    let path = std::env::args().nth(1).expect("usage: dump <file.html> <base>");
    let base = std::env::args().nth(2).unwrap_or_else(|| "https://example.com/".into());
    let bytes = std::fs::read(&path).expect("read file");
    let html = String::from_utf8_lossy(&bytes);

    let doc = html::parse(&html);
    let (_doc, console) = engine::run_scripts(doc, &base);
    let errors: Vec<&String> = console.iter().filter(|l| l.starts_with('\u{26a0}')).collect();
    println!("total console lines: {}  errors: {}", console.len(), errors.len());
    for line in &console {
        println!("{line}");
    }
}
