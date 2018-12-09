const js = import("./wasm/tetris");

js.then(js => {
  js.greet("World!!");
});
