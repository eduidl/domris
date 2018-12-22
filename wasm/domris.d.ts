/* tslint:disable */
export enum Control {MoveLeft,MoveRight,MoveDown,MoveBottom,RotateLeft,RotateRight,}
export function draw(arg0: Domris, arg1: any, arg2: boolean): void;

export class Domris {
free(): void;

 constructor();

 start(arg0: number): void;

 update(arg0: number): boolean;

 playing(): boolean;

 point(): number;

 gameover(): boolean;

 enqueue_control(arg0: number): void;

}
