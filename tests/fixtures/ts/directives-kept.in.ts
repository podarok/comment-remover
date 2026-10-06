// plain comment
// @ts-expect-error legacy
const a: number = "x";
// @ts-ignore
const b: number = "y";
/* eslint-disable no-console */
console.log(a, b);
// eslint-disable-next-line no-undef
foo();
// prettier-ignore
const m = [1,0,0,
           0,1,0];
/* istanbul ignore next */
function g() {}
const lazy = import(/* @vite-ignore */ path);
const chunk = import(/* webpackChunkName: "x" */ "./x");
// @vitest-environment jsdom
