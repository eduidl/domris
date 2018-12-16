import("./wasm/tetris")
  .then(wasm => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    let tetris = new wasm.Tetris;
    wasm.draw(tetris, ctx);

    document.addEventListener('keydown', (e) => {
      switch(e.key) {
        case 'ArrowRight':
          e.preventDefault();
          tetris.enqueue_control(wasm.Control.MoveRight);
          break;
        case 'ArrowLeft':
          e.preventDefault();
          tetris.enqueue_control(wasm.Control.MoveLeft);
          break;
        case 'ArrowDown':
          e.preventDefault();
          tetris.enqueue_control(wasm.Control.MoveDown);
          break;
        case 'ArrowUp':
          e.preventDefault();
          tetris.enqueue_control(wasm.Control.MoveBottom);
          break;
        case ' ':
          e.preventDefault();
          const control = e.shiftKey ? wasm.Control.RotateLeft
                                     : wasm.Control.RotateRight;
          tetris.enqueue_control(control);
          break;
      }
    });
    
    var last = now = Date.now();
    function frame() {
      now = Date.now();
      tetris.update(now - last);
      wasm.draw(tetris, ctx);
      last = now;
      requestAnimationFrame(frame, canvas);
    }
    frame();
  })
  .catch(console.error);

