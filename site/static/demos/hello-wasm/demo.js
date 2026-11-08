// Hashes the text box with the Rust SHA-256 module on every keystroke. Nothing leaves the page.
// pkg/ is built by wasm-pack from apps/hello-wasm (see its README); it isn't committed.
import init, { sha256_hex } from "./pkg/hello_wasm.js";

const input = document.getElementById("hello-input");
const output = document.getElementById("hello-output");

try {
  await init();
  const update = () => {
    output.textContent = sha256_hex(input.value);
  };
  input.addEventListener("input", update);
  update();
} catch (err) {
  output.textContent = "the WebAssembly module failed to load";
  throw err;
}
