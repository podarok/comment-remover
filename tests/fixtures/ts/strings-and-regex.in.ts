const url = "https://example.com/a//b"; // real comment
const s = '/* not a comment */';
const t = `a ${/* in template */ 1} // still string ${"x" /* y */}`;
const r1 = /\/\/ not/;
const r2 = /a\/*b/g;
const ml = `line // one
line /* two */`;
