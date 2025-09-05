// @ts-check

export function Size(width = 80, height = 60) {
  this.width = width;
  this.height = height;
}

Size.prototype.resize = function(newWidth, newHeight) {
  this.width = newWidth;
  this.height = newHeight;
}

export function Position(x = 0, y = 0) {
  this.x = x;
  this.y = y;
}

Position.prototype.move = function(newX, newY) {
  this.x = newX;
  this.y = newY;
}

export class ProgramWindow {
  screenSize = new Size(800, 600);
  size = new Size;
  position = new Position;

  resize(size) {
    let width = size.width < 1 ? 1 : size.width;
    let height = size.height < 1 ? 1 : size.height;
    if (this.position.x + width > this.screenSize.width) {
      width = this.screenSize.width - this.position.x;
    }
    if (this.position.y + height > this.screenSize.height) {
      height = this.screenSize.height - this.position.y;
    }
    this.size.resize(width, height);
  }

  move(position) {
    let x = position.x < 0 ? 0 : position.x;
    let y = position.y < 0 ? 0 : position.y;
    if (this.size.width + x > this.screenSize.width) {
      x = this.screenSize.width - this.size.width;
    }
    if (this.size.height + y > this.screenSize.height) {
      y = this.screenSize.height - this.size.height;
    }
    this.position.move(x, y);
  }
}

export function changeWindow(programWindow) {
  programWindow.size = new Size(400, 300);
  programWindow.position = new Position(100, 150);
  return programWindow;
}
