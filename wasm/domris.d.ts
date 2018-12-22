/* tslint:disable */
export enum Control {MoveLeft,MoveRight,MoveDown,MoveBottom,RotateLeft,RotateRight,}
export function draw(arg0: Domris, arg1: any): void;

export class Domris {
free(): void;

 constructor();

 update(arg0: number): void;

 enqueue_control(arg0: number): void;

}
