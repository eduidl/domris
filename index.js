import("./wasm/tetris")
  .then(wasm => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    let tetris = new wasm.Tetris;
    wasm.draw(tetris, ctx);
    
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

