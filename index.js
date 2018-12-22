import("./wasm/domris")
  .then(wasm => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    let domris = new wasm.Domris;
    wasm.draw(domris, ctx);

    document.addEventListener('keydown', (e) => {
      switch(e.key) {
        case 'ArrowRight':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveRight);
          break;
        case 'ArrowLeft':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveLeft);
          break;
        case 'ArrowDown':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveDown);
          break;
        case 'ArrowUp':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveBottom);
          break;
        case ' ':
          e.preventDefault();
          const control = e.shiftKey ? wasm.Control.RotateLeft
                                     : wasm.Control.RotateRight;
          domris.enqueue_control(control);
          break;
      }
    });
    
    var last = now = Date.now();
    function frame() {
      now = Date.now();
      domris.update(now - last);
      wasm.draw(domris, ctx);
      last = now;
      requestAnimationFrame(frame, canvas);
    }
    frame();
  })
  .catch(console.error);
