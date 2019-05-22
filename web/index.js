const js = import("./target/wasm32-unknown-unknown/web");

js.then(js => {
  let moveUp = false;
  let moveDown = false;

  document.addEventListener('keydown', function (event) {
    if (event.key == 'w') {
      moveUp = true;
    } else  if (event.key == 's') {
      moveDown = true;
    }
  });

  document.addEventListener('keyup', function (event) {
    if (event.key == 'w') {
      moveUp = false;
    } else  if (event.key == 's'){
      moveDown = false;
    }
  });

  let lastTimestamp = null;
  function update(timestamp) {
    let deltaTime = 0;
    if (lastTimestamp != null) {
      deltaTime = (timestamp - lastTimestamp) / 1000;
    }

    lastTimestamp = timestamp;

    js.run(deltaTime, moveUp, moveDown);
    window.requestAnimationFrame(update);
  }
  update();
});