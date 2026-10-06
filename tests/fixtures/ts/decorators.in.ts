// before decorator
@Component({ selector: "x" }) // trailing decorator
class A {
  @Input() /* inline */ value = 1;
  // between
  @Output() change = new EventEmitter();
}
