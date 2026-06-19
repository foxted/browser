//! A lightweight Web Platform Tests runner for our engine.
//!
//! WPT's official harness drives a browser over WebDriver; we don't have one yet, so this runs
//! `testharness.js` tests *in-process*: a tiny static HTTP server serves a WPT checkout (so the
//! tests' `/resources/...` references resolve), with our own `testharnessreport.js` injected to
//! disable DOM output and stash the results on `window`. For each test the engine loads the URL,
//! ticks the event loop until the harness completes (or times out), and we read the result counts.
//!
//! Usage: `wpt-runner <wpt-root> <subpath> [max-tests]`
//!   e.g. `wpt-runner ./wpt dom/nodes 200`

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Our replacement for WPT's `testharnessreport.js`: turn off the harness's DOM output (it would
/// `appendChild` into a `#log`/body that test fixtures often lack) and capture the structured
/// results onto `window` so the runner can read them via `console_eval`.
const REPORT_JS: &str = r#"
setup({ output: false });
add_completion_callback(function (tests, status) {
  var c = [0, 0, 0, 0];
  for (var i = 0; i < tests.length; i++) { c[tests[i].status] = (c[tests[i].status] || 0) + 1; }
  window.__wpt_pass = c[0]; window.__wpt_fail = c[1];
  window.__wpt_timeout = c[2]; window.__wpt_notrun = c[3];
  window.__wpt_total = tests.length;
  window.__wpt_harness = status.status;       // 0 OK, 1 ERROR, 2 TIMEOUT
  window.__wpt_harness_msg = status.message || "";
  // First failure detail, for quick triage.
  window.__wpt_firstfail = "";
  for (var j = 0; j < tests.length; j++) {
    if (tests[j].status === 1) { window.__wpt_firstfail = tests[j].name + ": " + (tests[j].message || ""); break; }
  }
  window.__wpt_done = 1;
});
"#;

fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" | "htm" | "xht" | "xhtml" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}

/// Minimal blocking static file server. One request per connection (`Connection: close`). The
/// `/resources/testharnessreport.js` path is overridden with [`REPORT_JS`]; everything else is read
/// from `root`.
fn serve(stream: &mut TcpStream, root: &Path) {
    let mut buf = [0u8; 8192];
    let n = match stream.read(&mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("/");
    // Strip query/fragment + percent-decode spaces.
    let path = path.split(['?', '#']).next().unwrap_or("/");
    let path = path.replace("%20", " ");

    let (body, ctype): (Vec<u8>, &str) = if path == "/resources/testharnessreport.js" {
        (REPORT_JS.as_bytes().to_vec(), "text/javascript; charset=utf-8")
    } else {
        let rel = path.trim_start_matches('/');
        let full = root.join(rel);
        // Contain within root.
        if !full.starts_with(root) {
            let _ = stream.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
            return;
        }
        match std::fs::read(&full) {
            Ok(b) => (b, content_type(&path)),
            Err(_) => {
                let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
                return;
            }
        }
    };
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&body);
}

/// Recursively collect runnable testharness files under `dir`, skipping the obvious non-tests
/// (references, manual tests, support files, the resources dir).
fn collect_tests(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for e in entries.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if matches!(name.as_str(), "support" | "resources" | "tools" | "META.yml") {
                continue;
            }
            collect_tests(&p, out);
        } else if name.ends_with(".html") || name.ends_with(".xht") || name.ends_with(".xhtml") {
            // Skip reftest references, manual tests, and visual/non-harness helpers.
            if name.contains("-ref.")
                || name.ends_with("-ref.html")
                || name.contains("-manual.")
                || name.contains(".tentative.") && false
            {
                continue;
            }
            out.push(p);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: wpt-runner <wpt-root> <subpath> [max-tests]");
        std::process::exit(2);
    }
    let root = std::fs::canonicalize(&args[1]).expect("wpt-root not found");
    let root = Arc::new(root);
    let subpath = &args[2];
    let max: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);

    // Start the static server on an ephemeral port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    {
        let root = root.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    let root = root.clone();
                    std::thread::spawn(move || serve(&mut s, &root));
                }
            }
        });
    }

    let mut all = Vec::new();
    collect_tests(&root.join(subpath), &mut all);
    all.sort();
    // Keep only testharness.js tests (skip reftests / visual tests, which have no JS result).
    let mut tests: Vec<PathBuf> = all
        .into_iter()
        .filter(|p| std::fs::read_to_string(p).map(|s| s.contains("testharness.js")).unwrap_or(false))
        .collect();
    let total_found = tests.len();
    tests.truncate(max);
    eprintln!(
        "running {} testharness tests from {} (server :{})",
        tests.len().min(total_found),
        subpath,
        port
    );

    let (mut files_ok, mut sub_pass, mut sub_fail, mut harness_err, mut timeouts) = (0, 0u64, 0u64, 0u64, 0u64);
    for path in &tests {
        let rel = path.strip_prefix(&*root).unwrap().to_string_lossy().replace(' ', "%20");
        let url = format!("http://127.0.0.1:{port}/{rel}");
        let mut e = engine::Engine::new();
        e.set_viewport(800, 600, 1.0);
        e.load_url(&url);

        // Tick until the harness reports completion or we time out (~10s wall).
        let start = Instant::now();
        let mut done = false;
        while start.elapsed() < Duration::from_secs(5) {
            for _ in 0..5 {
                e.tick();
            }
            if e.console_eval("window.__wpt_done || 0") == "1" {
                done = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        let short = path.strip_prefix(&*root).unwrap().to_string_lossy().to_string();
        if !done {
            timeouts += 1;
            println!("TIMEOUT  {short}");
            continue;
        }
        let harness = e.console_eval("window.__wpt_harness");
        if harness == "1" {
            harness_err += 1;
            let msg = e.console_eval("window.__wpt_harness_msg");
            println!("HARNESS-ERR  {short}  — {msg}");
            continue;
        }
        let p: u64 = e.console_eval("window.__wpt_pass").parse().unwrap_or(0);
        let f: u64 = e.console_eval("window.__wpt_fail").parse().unwrap_or(0);
        sub_pass += p;
        sub_fail += f;
        files_ok += 1;
        let mark = if f == 0 { "PASS" } else { "FAIL" };
        if f == 0 {
            println!("{mark} [{p}/{}]  {short}", p + f);
        } else {
            let ff = e.console_eval("window.__wpt_firstfail");
            println!("{mark} [{p}/{}]  {short}  — {ff}", p + f);
        }
    }

    println!("\n==== WPT summary: {subpath} ====");
    println!("files: {} ran, {} harness-errors, {} timeouts", files_ok, harness_err, timeouts);
    let total = sub_pass + sub_fail;
    let pct = if total > 0 { 100.0 * sub_pass as f64 / total as f64 } else { 0.0 };
    println!("subtests: {sub_pass}/{total} passed ({pct:.1}%)");
}
