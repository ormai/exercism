export class Triangle {
  constructor(...sides) {
    [this.a, this.b, this.c] = sides;
    this.sides = new Set(sides);
  }

  get isValid() {
    return this.a + this.b >= this.c && this.b + this.c >= this.a && this.a + this.c >= this.b;
  }

  get isEquilateral() {
    return this.isValid && this.sides.size === 1 && !this.sides.has(0);
  }

  get isIsosceles() {
    return this.isValid && this.sides.size <= 2;
  }

  get isScalene() {
    return this.isValid && this.sides.size === 3;
  }
}
