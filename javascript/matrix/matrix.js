export class Matrix {
  constructor(raw) {
    this.raw = raw;
  }

  get rows() {
    return this.raw.split('\n').map(row => row.split(' ').map(Number))
  }

  get columns() { // transpose
    return this.rows[0].map((_, i) => this.rows.map(row => row[i]));
  }
}
