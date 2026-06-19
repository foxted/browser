# HTML element support audit

Audited by rendering each element through the `engine` crate (layout + paint + `getComputedStyle`/
`getBoundingClientRect`/`visible_text` + click dispatch). **Full** = renders + correct display +
reasonable default styling/behavior. **Partial** = renders but wrong/missing default styling.
**None** = doesn't render or display is wrong.

A lot of the "Partial" inline/text items are *one-liners* — the cascade + `::before`/`::after`
machinery work; the **UA stylesheet (`crates/style` `user_agent_stylesheet()`) is just sparse** (it
sets `display` + heading sizes + b/strong/i/em, and almost nothing else: no margins, no
`white-space:pre`, no `text-decoration`, no `background` on `mark`, no list markers, no `th` styling).

## Top gaps, prioritized

### Quick wins — UA stylesheet + small layout/paint (huge visual payoff)
1. **`<br>` doesn't break lines** — text after `<br>` stays on the same line. (layout)
2. **No UA margins on block elements** — `p`, `h1`–`h6`, `ul`/`ol`, `blockquote`, `figure`, `dd` stack flush with 0 gap. (UA sheet)
3. **`<pre>` collapses whitespace/newlines + not monospace** — code blocks render as one mangled line. (UA `white-space:pre` + monospace; needs layout white-space support)
4. **`<hr>` renders nothing** — 0-height empty box. (UA border + paint)
5. **Inline decorations missing** — `s`/`del`/`strike` no strikethrough, `u`/`ins` no underline, `mark` no highlight bg, `sub`/`sup` no shift/size, `code`/`kbd`/`samp` not monospace, `a` no underline/link color, `cite`/`var`/`dfn`/`address` not italic, `q` no auto-quotes. (UA sheet + text-decoration/sub-sup rendering + monospace)
6. **Lists: no bullet/number markers + no indentation** — `ul`/`ol`/`li` show no `•`/`1.`, and `li`/`dd`/nested lists sit flush at x=0. (marker rendering + UA padding/margins)
7. **`th` not bold/centered; `caption` overlaps table** — (UA sheet + table layout)

### Bigger features
8. **No table layout** — `td`/`th` are `display:inline`, cells flow as inline text on one line; `thead`/`tbody`/`tfoot` make their rows VANISH (most real tables render as an empty border box). No grid/column model, no `colspan`/`rowspan`/`border-collapse`. (a real table layout algorithm)
9. **Form widgets** — `input` `range`/`color`/`date`/`time`/`file`/`month`/`week` render as 0×0 nothing; `progress`/`meter` show no bar; checkbox glyph (U+2611/2610) is **invisible** (font lacks it — radios using ○/● are fine); text/submit/button inputs have no border/chrome.
10. **`<label>` has no hit box** (`getBoundingClientRect` = 0) — so `<label for>` click→control focus/toggle never fires (the resolution code exists, just unreachable).
11. **SVG renders nothing** — inline `<svg>` (`rect`/`circle`/`path`) → 0×0, no vector graphics at all.
12. **`<video>`/`<audio>`/`<iframe>`/`<embed>`/`<object>`** — render nothing but fallback text.
13. **`<img>` `width`/`height` HTML attributes ignored** (only CSS sizing works); broken `<img>` shows no `alt`; `naturalWidth`/`naturalHeight` undefined.
14. **`dialog`** — ignores `open`, `showModal()` is undefined (throws); **textarea/select `.value`** JS getters return empty/undefined.

## What works well (Full)
- All sectioning (`div`/`section`/`article`/`nav`/`header`/`footer`/`main`/`aside`), `h1`–`h6` (sized+bold), `figcaption`, `span`, `strong`/`b` (bold), `em`/`i` (italic), `time`/`data`/`abbr`.
- `form` (+ submit on click), `details`/`summary` (toggle + ▸/▾), `select`/`option` (dropdown + selection), radio (○/● + group exclusivity), `password` (masked), `output`, checkbox toggle logic (glyph aside).
- `img` (file:// + data: URL decode/blit, CSS sizing, intrinsic size), `picture`→`img` fallback.
- **`canvas` 2D — fully rasterizes** (fillRect, path fill, fillText, linear/radial gradients all paint into the frame).

## Element matrix (condensed)
Sectioning: html/body/div/section/article/nav/header/footer/main/aside/h1–6/figcaption = Full · p/blockquote/figure/address/pre = Partial (no margins / not styled) · hr = None
Inline: span/strong/b/em/i/abbr/time/data = Full · a/u/s/strike/code/kbd/samp/var/mark/small/sub/sup/del/ins/q/cite/dfn/bdi/bdo/ruby/rt/rp/wbr = Partial · br = None
Lists: dl/dt/figcaption = Full · ul/ol/li/dd/nested = Partial (no markers/indent) · menu = None
Tables: caption/table = Partial · thead/tbody/tfoot/tr/td/th/col/colgroup = None
Forms: form/output/password/hidden/radio = Full · text/search/email/url/tel/number/checkbox/submit/reset/button/textarea/label/datalist/fieldset/legend = Partial · range/color/date/time/datetime-local/month/week/file/progress/meter = None
Interactive: details/summary = Full · dialog = Partial(broken) · menu = Partial
Media: canvas/img(data:)/img(CSS-size) = Full · img(file)/picture/figure/figcaption/wbr = Partial · svg/video/audio/iframe/embed/object/map/area/track/source = None
