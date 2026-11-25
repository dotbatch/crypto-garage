// AEAD playground. The key stays inside the Wasm module: this script sees nonces, ciphertexts and
// tags, never key material. Nothing leaves the page.
// pkg/ is built by wasm-pack from apps/aead-wasm (see its README); it isn't committed.
import init, { Playground, seal_reusing_nonce } from "./pkg/aead_wasm.js";

const TAG_LEN = 16;
const $ = (id) => document.getElementById(id);
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
const xor = (a, b) => a.slice(0, Math.min(a.length, b.length)).map((x, i) => x ^ b[i]);

let playground; // One algorithm and its key; see newPlayground.
let sealed; // { nonce, bytes }: the last message encrypted, as tampered with so far.

// A new algorithm gets a new key. free() zeroizes the old key in Wasm memory.
function newPlayground() {
  playground?.free();
  playground = new Playground($("aead-algorithm").value);
}

function seal() {
  const message = playground.seal($("aead-aad").value, $("aead-message").value);
  sealed = { nonce: message.nonce, bytes: message.ciphertext };
  message.free();
  $("aead-byte").max = sealed.bytes.length - 1;
  $("aead-result").textContent = "";
  show();
}

function show() {
  const body = sealed.bytes.length - TAG_LEN;
  $("aead-nonce").textContent = hex(sealed.nonce);
  $("aead-ciphertext").textContent = hex(sealed.bytes.subarray(0, body)) || "(empty)";
  $("aead-tag").textContent = hex(sealed.bytes.subarray(body));
}

function tamper() {
  const index = Number($("aead-byte").value);
  if (!Number.isInteger(index) || index < 0 || index >= sealed.bytes.length) {
    $("aead-result").textContent = `pick a byte from 0 to ${sealed.bytes.length - 1}`;
    return;
  }
  sealed.bytes[index] ^= 0x01;
  $("aead-result").textContent = "";
  show();
}

// Uses the associated data as it is now, so editing it after encrypting also fails the check.
function open() {
  try {
    const plain = playground.open(sealed.nonce, $("aead-aad").value, sealed.bytes);
    $("aead-result").textContent = `"${decoder.decode(plain)}"`;
  } catch (err) {
    $("aead-result").textContent = err;
  }
}

// The deliberately broken path. Each run uses a fresh throwaway key, which makes no difference:
// the XOR below cancels the keystream, whatever the key was.
function showNonceReuse() {
  const [p1, p2] = [$("reuse-p1").value, $("reuse-p2").value];
  const reused = seal_reusing_nonce($("aead-algorithm").value, p1, p2);
  const [c1, c2] = [reused.c1, reused.c2];
  reused.free();
  const leaked = xor(c1, c2);
  $("reuse-c1").textContent = hex(c1);
  $("reuse-c2").textContent = hex(c2);
  $("reuse-leaked").textContent = hex(leaked);
  $("reuse-plain-xor").textContent = hex(xor(encoder.encode(p1), encoder.encode(p2)));
  $("reuse-recovered").textContent = `"${decoder.decode(xor(leaked, encoder.encode(p1)))}"`;
}

try {
  await init();
  newPlayground();
  seal();
  $("aead-algorithm").addEventListener("change", () => {
    newPlayground();
    seal();
    if ($("reuse-toggle").checked) showNonceReuse();
  });
  $("aead-seal").addEventListener("click", seal);
  $("aead-tamper").addEventListener("click", tamper);
  $("aead-open").addEventListener("click", open);
  $("reuse-toggle").addEventListener("change", () => {
    $("reuse-panel").hidden = !$("reuse-toggle").checked;
    if ($("reuse-toggle").checked) showNonceReuse();
  });
  $("reuse-p1").addEventListener("input", showNonceReuse);
  $("reuse-p2").addEventListener("input", showNonceReuse);
} catch (err) {
  $("aead-nonce").textContent = "the WebAssembly module failed to load";
  throw err;
}
