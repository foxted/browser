# WPT css/cssom wins (top of the report)

Working down from the top of the report (alphabetical → css/cssom first). Overall 91.5% → **91.8%**; css/cssom 82% → **88%** (no dom/html-dom regression).

- [x] **CSSOM rule object model** (CSSStyleRule/cssimportrule/serialize-media-rule/CSSFontFeatureValues/CSSStyleSheet → ~100%) — `document.styleSheets[].cssRules[]`, `insertRule`/`deleteRule`, rule types: `CSSStyleRule`/`CSSMediaRule`/`CSSImportRule`/`CSSFontFeatureValuesRule`, `.cssText` serialization. Files: `CSSStyleRule.html` (0/10), `cssimportrule.html` (0/11), `serialize-media-rule.html` (0/12), `CSSFontFeatureValuesRule.html` (0/8), `CSSStyleSheet.html` (dead), `serialize-namespaced-type-selectors.html` (31/60). (js + css)
- [x] **getComputedStyle on pseudo-elements** (pseudo 25/28, pseudo-with-argument 22/22, picker 9/9) — `getComputedStyle(el, '::before')` etc. Files: `getComputedStyle-pseudo.html` (1/28), `getComputedStyle-pseudo-with-argument.html` (0/22), `getComputedStyle-pseudo-picker.html` (1/9). (js + style)
- [x] **CSSStyleDeclaration shorthands + custom props** (all-shorthand 21/27, serialize-custom-props 20/20, shorthand-values 20/21) — `el.style` shorthand get/set (margin/padding/border/font/background/`all`) + `--custom` props. Files: `cssstyledeclaration-all-shorthand.html` (3/27), `shorthand-values.html` (6/21), `serialize-custom-props.html` (8/20). (js + css)
- [ ] **getComputedStyle insets remaining** (sticky-container/fixed/absolute edge cases, ~144) — needs real layout; deferred.
