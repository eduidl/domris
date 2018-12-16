/* tslint:disable */
export enum Control {MoveLeft,MoveRight,MoveDown,MoveBottom,RotateLeft,RotateRight,}
export function draw(arg0: Tetris, arg1: any): void;

export class Tetris {
free(): void;

 constructor();

 update(arg0: number): void;

 enqueue_control(arg0: number): void;

}
