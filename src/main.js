const { invoke } = window.__TAURI__.tauri;

let calculateInput;
let resultMsg;
let testButton;

async function calculate() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  resultMsg.textContent = await invoke("calculate", { input: calculateInput.value });
}

async function display(val) {
  calculateInput.value += val;
}

window.addEventListener("DOMContentLoaded", () => {
  calculateInput = document.querySelector("#calculate-input");
  resultMsg = document.querySelector("#result-msg");
  document
    .querySelector("#calculate-button")
    .addEventListener("click", () => calculate());
});
