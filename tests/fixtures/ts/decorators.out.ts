@Component({ selector: "x" })
class A {
  @Input() value = 1;
  @Output() change = new EventEmitter();
}
