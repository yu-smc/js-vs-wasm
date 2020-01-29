const js = import("../wasm/pkg/assembly.js");

js.then(js => {
  console.log(js.greet(6.0))
})
