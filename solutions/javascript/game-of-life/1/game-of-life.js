export class GameOfLife {
  static #directions = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
  #matrix

  constructor(matrix) {
    this.#matrix = matrix
  }

  #liveNeighbors = (i, j) => GameOfLife.#directions.reduce((count, [di, dj], _) => {
    const ni = di + i, nj = dj + j;
    if (0 <= ni && ni < this.#matrix.length && 0 <= nj && nj < this.#matrix[ni].length) {
      if (this.#matrix[ni][nj] == 1) {
        return count + 1;
      }
    }
    return count;
  }, 0);

  tick() {
    let edit_matrix = JSON.parse(JSON.stringify(this.#matrix));
    for (let i = 0; i < this.#matrix.length; i++) {
      for (let j = 0; j < this.#matrix[i].length; j++) {
        const neighbors = this.#liveNeighbors(i, j);
        if (this.#matrix[i][j] == 1) {
          if (neighbors <= 1) {
            edit_matrix[i][j] = 0;
          } else if (neighbors >= 4) {
            edit_matrix[i][j] = 0;
          }
        } else if (neighbors === 3) {
          edit_matrix[i][j] = 1;
        }
      }
    }
    this.#matrix = edit_matrix;
  }

  state() {
    return this.#matrix;
  }
}
