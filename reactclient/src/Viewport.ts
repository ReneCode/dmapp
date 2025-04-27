//

export class Viewport {
  x: number;
  y: number;
  width: number;
  height: number;
  zoom: number;
  canvas_width: number;
  canvas_height: number;

  constructor() {
    this.x = 0;
    this.y = 0;
    this.width = 100;
    this.height = 100;
    this.zoom = 1;
    this.canvas_width = 400;
    this.canvas_height = 400;
  }

  zoomViewport(deltaY: number, clientX: number, clientY: number) {
    const x = this.x + clientX / this.zoom;
    const y = this.y + clientY / this.zoom;

    const MAX_DELTA = 10;
    let delta = Math.min(Math.abs(deltaY), MAX_DELTA);
    const sign = -Math.sign(deltaY);
    delta = delta * sign;

    const oldZoom = this.zoom;
    const newZoom = oldZoom * (1 + delta / 100);

    const newX = x - (oldZoom / newZoom) * (x - this.x);
    const newY = y - (oldZoom / newZoom) * (y - this.y);

    this.x = Math.round(newX);
    this.y = Math.round(newY);
    this.zoom = Math.round(newZoom);
    this.width = Math.round(this.canvas_width / newZoom);
    this.height = Math.round(this.canvas_height / newZoom);
  }
}
