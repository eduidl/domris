const id = (id: string): HTMLElement => document.getElementById(id);

const canvas = id("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");
const levelBtns = document.querySelectorAll("#levelBtnGroup>button");
const gameToggleBtn = id("gameToggleBtn");
const point = id("point");

const enableBtns = (): void => {
  gameToggleBtn.innerText = "Play!";
  gameToggleBtn.classList.remove("disabled");
  for (const btn of levelBtns) {
    btn.classList.remove("disabled");
  }
};

const disableBtns = (): void => {
  gameToggleBtn.innerText = "Playing";
  gameToggleBtn.classList.add("disabled");
  for (const btn of levelBtns) {
    if (!btn.classList.contains("active")) {
      btn.classList.add("disabled");
    }
  }
};

class Timer {
  last: number;
  now: number;

  constructor() {
    this.now = Date.now();
    this.last = -1;
  }

  tick(): void {
    this.last = this.now;
    this.now = Date.now();
  }

  elapsed(): number {
    return this.now - this.last;
  }
}

const wasm = import("../wasm/pkg/domris");
wasm
  .then(mod => {
    let level = 1;
    const domris = new mod.Domris();

    document.addEventListener("keydown", (e: KeyboardEvent): void => {
      if (e.key === "ArrowRight" || e.key === "l") {
        domris.enqueue_control(mod.Control.MoveRight);
      } else if (e.key === "ArrowLeft" || e.key === "h") {
        domris.enqueue_control(mod.Control.MoveLeft);
      } else if (e.key === "ArrowDown" || e.key === "j") {
        domris.enqueue_control(mod.Control.MoveDown);
      } else if (e.key === "ArrowUp" || e.key === "k") {
        domris.enqueue_control(mod.Control.MoveBottom);
      } else if (e.key === " ") {
        domris.enqueue_control(
          e.shiftKey ? mod.Control.RotateLeft : mod.Control.RotateRight
        );
      } else {
        return;
      }
      e.preventDefault();
    });

    for (const btnClicked of levelBtns) {
      btnClicked.addEventListener("click", (): void => {
        if (domris.playing()) return;
        for (const [i, btn] of levelBtns.entries()) {
          if (btn === btnClicked) {
            level = i + 1;
            btnClicked.classList.add("active");
          } else {
            btn.classList.remove("active");
          }
        }
      });
    }

    let timer: Timer | null;
    const frame = (): void => {
      timer.tick();
      const result = domris.update(timer.elapsed());
      mod.draw(domris, ctx, result);
      if (result) {
        point.innerHTML = String(domris.point);
      }
      const requestId = requestAnimationFrame(frame);
      if (domris.playing()) return;
      enableBtns();
      cancelAnimationFrame(requestId);
    };

    gameToggleBtn.addEventListener("click", (): void => {
      if (domris.playing()) return;
      disableBtns();
      domris.start(level);
      mod.draw(domris, ctx, true);
      timer = new Timer();
      frame();
    });
  })
  .catch(console.error);
