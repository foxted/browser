# Feature backlog (non-CSS)

See also `docs/CSS-TODO.md` for the CSS backlog.

## Done (recent)
- [x] **V8 JS engine** (replaced Boa) — classic scripts + ES modules + dynamic import.
- [x] **Interactivity** — click/type/forms/checkbox/radio/hover/focus/blur/change/submit, live event loop (timers/rAF/animations).
- [x] **Web APIs** — fetch (GET/POST/…), Headers/Request/Response, FormData (urlencoded), AbortController, URLSearchParams, TextEncoder/Decoder, NodeFilter/TreeWalker, CSS object, devicePixelRatio, defaultView.
- [x] **Async concurrent fetch** — `fetch()` runs on background threads, resolved via the drain; concurrent (imlunahey ~20s → ~slowest request).
- [x] **Cookies** — persistent jar in `net` (Set-Cookie across requests + redirects → stay logged in). In-memory per process.
- [x] **Observers** — MutationObserver / IntersectionObserver / ResizeObserver actually fire (Rust geometry + JS dispatch; scroll drives lazy-load/reveal).
- [x] **DevTools** — ⌘⌥I: Console (with live REPL) + Network tab.
- [x] **Window** — horizontal resize fixed; window position/size/monitor persisted across launches.
- [x] **Build-staleness bug** — the app was loading a stale cdylib; `scripts/build.sh` now mirrors the fresh lib + clean-links so all JS-only fixes actually ship.

## Progressive / incremental rendering (BIG — requested; see analysis)
Today the load is one blocking sequence: full HTML download → full parse → sequential sub-resource fetches → one paint. Goal: paint incrementally like Chrome/Firefox.
- [ ] **Streaming HTML parser** — feed network bytes as they arrive; build the DOM incrementally (today `html::parse(&str)` needs the whole body).
- [ ] **Async/parallel sub-resource fetch** — CSS/JS/images fetched off the main path, in parallel; each completion triggers incremental re-layout/re-paint (today: sequential blocking `net::fetch` loops).
- [ ] **Load scheduler / event loop** — interleave network/parse/layout/paint instead of strict sequence (JS already has a drain + 50ms tick; the LOAD pipeline does not).
- [ ] **Render-blocking semantics** — `<head>` CSS blocks first paint (no FOUC); sync `<script>` blocks the parser; `async`/`defer` scripts + images do not.
- [ ] **Incremental `net` read API** — yield chunks as they arrive (today it buffers the full body before returning).

## Visual fidelity
- [ ] **CSS Grid** — only partial today (block + flex are solid).
- [ ] **CSS transitions + `@keyframes`** — static `transform` works; nothing animates.
- [ ] **SVG rendering** — `<svg>` icons/logos don't draw (raster png/jpg/gif/webp do).
- [ ] **Canvas 2D** — `getContext('2d')` is a stub; charts/drawings are blank.

## Browser UX
- [ ] **Form submit navigation** — we fire the `submit` event but don't GET/POST-navigate.
- [ ] **Text selection + copy** — can't select/copy page text.
- [ ] **File upload** — multipart/form-data + `File`/`Blob` (FormData is urlencoded-only).
- [ ] Find-in-page, zoom, bookmarks, downloads, reload-from-cache.

## Platform APIs
- [ ] **Persistent localStorage / sessionStorage** — currently in-memory (lost on navigation/restart).
- [ ] **History API** — `pushState`/`replaceState`/`popstate` (SPA routing).
- [ ] **WebSocket**, **Web Workers**, **IndexedDB**, **Service Workers** — not implemented.
- [ ] **document.cookie server-sync** — JS reads its own in-memory store; server (non-HttpOnly) cookies aren't surfaced to JS (the HTTP jar in `net` is opaque).

## Media
- [ ] **`<video>` / `<audio>`** — stubs (no playback).
- [ ] **WebGL** — none.

## Networking / correctness
- [ ] **HTTP cache** — proper RFC cache with headers (today: opt-in disk cache via `NET_CACHE_DIR`, GET-only).
- [ ] Layout-fidelity passes against real sites; many CSS properties partial.

## Notes
- **google.com** is the all-or-nothing hardest target: after fixing devicePixelRatio/CSS it still throws google-internal protobuf assertions (`Error: G`) + module-loader errors and won't fully hydrate. Not worth chasing — fixes there benefit no other site.
