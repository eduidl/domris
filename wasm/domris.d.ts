/* tslint:disable */
/**
* @param {Domris} game 
* @param {any} ctx 
* @param {boolean} partial_update 
*/
export function draw(game: Domris, ctx: any, partial_update: boolean): void;
export enum Control {
  MoveLeft,
  MoveRight,
  MoveDown,
  MoveBottom,
  RotateLeft,
  RotateRight,
}
/**
*/
/**
*/
export class Domris {
  free(): void;
/**
* @returns {Domris} 
*/
  constructor();
/**
* @param {number} level 
*/
  start(level: number): void;
/**
* @param {number} interval 
* @returns {boolean} 
*/
  update(interval: number): boolean;
/**
* @returns {boolean} 
*/
  playing(): boolean;
/**
* @param {number} control 
*/
  enqueue_control(control: number): void;
  readonly point: number;
}
