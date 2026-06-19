# CSS feature backlog

CSS the engine doesn't yet implement (or only partially), to work through.
Reported by the user as "some CSS features still missing." Roughly ordered by impact.

## Selectors (matcher is currently single-compound only)
- [ ] Combinators: descendant (` `), child (`>`), adjacent sibling (`+`), general sibling (`~`)
- [ ] Structural pseudo-classes: `:first-child`, `:last-child`, `:nth-child()`, `:nth-of-type()`, `:only-child`
- [ ] State pseudo-classes in CSS: `:hover`, `:focus`, `:active`, `:checked`, `:disabled`
- [ ] `:not()`, `:is()`, `:where()`, `:has()`
- [ ] Attribute selectors: `[attr]`, `[attr=val]`, `[attr^=]`, `[attr*=]`, `[attr$=]`
- [ ] Pseudo-elements: `::before` / `::after` (with `content`), `::placeholder`, `::marker`

## Layout
- [ ] CSS Grid (currently block + flex only; grid is minimal)
- [ ] `position: sticky`
- [ ] `overflow` clipping / scroll containers
- [ ] `aspect-ratio`, `object-fit`
- [ ] `gap` everywhere, `place-items`/`place-content`

## Paint / visual
- [ ] `transform` (translate/rotate/scale/matrix) + `transform-origin`
- [ ] `transition` + `@keyframes` animations
- [ ] Gradients: `linear-gradient` / `radial-gradient` / `conic-gradient` backgrounds
- [ ] `box-shadow`, `text-shadow`
- [ ] `filter` / `backdrop-filter`
- [ ] `background-image` (url), `background-size`/`position`/`repeat`
- [ ] `border-image`, multiple backgrounds
- [ ] `outline`, `clip-path`

## Typography / misc
- [ ] `@font-face` (web fonts)
- [ ] `white-space` variants, `text-overflow: ellipsis`, `word-break`
- [ ] `writing-mode`, `direction: rtl`
- [ ] `cursor` values, `pointer-events`
- [ ] CSS variables edge cases, `calc()` in more properties
