const o = {
  a: 1, // one
  /* two */ b: 2,
};
const arr = [
  1, // first
  2, /* second */
  // third
  3,
];
call(a, /* skip */ b);
