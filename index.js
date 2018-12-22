import("./wasm/domris")
  .then(wasm => {
    id = (id) => document.getElementById(id);

    let level = 1;
    const level_btns = document.querySelectorAll('#levelBtnGroup>button');

    const canvas = id('canvas');
    const ctx = canvas.getContext('2d');
    let domris = new wasm.Domris;

    const game_toggle_btn = id('gameToggleBtn');
    const point = id('point');

    document.addEventListener('keydown', (e) => {
      switch(e.key) {
        case 'ArrowRight':
        case 'l':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveRight);
          break;
        case 'ArrowLeft':
        case 'h':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveLeft);
          break;
        case 'ArrowDown':
        case 'j':
          e.preventDefault();
          domris.enqueue_control(wasm.Control.MoveDown);
          break;
        case 'ArrowUp':
        case 'k':
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

    level_btns.forEach((btn_clicked) => {
      btn_clicked.addEventListener('click', (e) => {
        if (domris.playing()) return;
        level_btns.forEach((btn, i) => {
          if (btn == btn_clicked) {
            level = i + 1;
            btn_clicked.classList.add('active');
          } else {
            btn.classList.remove('active');
          }
        });
      });
    });

    var last = now = Date.now();
    function frame() {
      now = Date.now();
      result = domris.update(now - last);
      wasm.draw(domris, ctx, result);
      if (result) {
        point.innerHTML = domris.point();
      }
      last = now;
      let request_id = requestAnimationFrame(frame, canvas);
      if (!domris.playing()) {
        cancelAnimationFrame(request_id);
        game_toggle_btn.innerText = 'Play!';
        game_toggle_btn.classList.remove('disabled');
        level_btns.forEach((btn) => {
          btn.classList.remove('disabled');
        });
      }
    }

    game_toggle_btn.addEventListener('click', (e) => {
      if (domris.playing()) return;
      game_toggle_btn.innerText = 'Playing';
      game_toggle_btn.classList.add('disabled');
      level_btns.forEach((btn) => {
        if (!btn.classList.contains('active')) {
          btn.classList.add('disabled');
        }
      });
      domris.start(level);
      wasm.draw(domris, ctx, true);
      last = Date.now();
      frame();
    });
  })
  .catch(console.error);
