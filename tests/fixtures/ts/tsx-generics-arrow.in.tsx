const id = <T,>(x: T): T => x; // generic arrow in tsx
const g = <T extends object>(x: T) => x; /* constraint */
