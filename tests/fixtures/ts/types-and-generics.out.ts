interface Props {
  id: number;
  name: string;
}
type Pair<T, U> = [T, U];
enum Color {
  Red,
  Green,
}
function over(a: string): string;
function over(a: number): number;
function over(a: any): any {
  return a;
}
abstract class Base {
  abstract run(): void;
}
