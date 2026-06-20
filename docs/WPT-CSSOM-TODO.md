# WPT css/cssom wins (top of the report)

Working down from the top of the report (alphabetical → css/cssom first). Overall 91.5%; css/cssom 82%.

- [ ] **CSSOM rule object model** — `document.styleSheets[].cssRules[]`, `insertRule`/`deleteRule`, rule types: `CSSStyleRule`/`CSSMediaRule`/`CSSImportRule`/`CSSFontFeatureValuesRule`, `.cssText` serialization. Files: `CSSStyleRule.html` (0/10), `cssimportrule.html` (0/11), `serialize-media-rule.html` (0/12), `CSSFontFeatureValuesRule.html` (0/8), `CSSStyleSheet.html` (dead), `serialize-namespaced-type-selectors.html` (31/60). (js + css)
- [ ] **getComputedStyle on pseudo-elements** — `getComputedStyle(el, '::before')` etc. Files: `getComputedStyle-pseudo.html` (1/28), `getComputedStyle-pseudo-with-argument.html` (0/22), `getComputedStyle-pseudo-picker.html` (1/9). (js + style)
- [ ] **CSSStyleDeclaration shorthands + custom props** — `el.style` shorthand get/set (margin/padding/border/font/background/`all`) + `--custom` props. Files: `cssstyledeclaration-all-shorthand.html` (3/27), `shorthand-values.html` (6/21), `serialize-custom-props.html` (8/20). (js + css)
- [ ] **getComputedStyle insets remaining** (sticky-container/fixed/absolute edge cases, ~144) — needs real layout; deferred.
