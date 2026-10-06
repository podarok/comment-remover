interface Props {
  // the id
  id: number; // trailing
  /** name */
  name: string;
}
type Pair<T /* key */, U /* value */> = [T, U];
enum Color {
  Red, // r
  /* g */ Green,
}
function over(a: string): string; // overload one
function over(a: number): number; /* overload two */
function over(a: any): any {
  return a;
}
abstract class Base {
  // field
  abstract run(): void; // abstract
}
