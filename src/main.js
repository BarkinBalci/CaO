const { invoke } = window.__TAURI__.tauri;

let calculateInput;
let resultMsg;

async function calculate() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  calculateInput.value = await invoke("calculate", { input: calculateInput.value });
}

window.addEventListener("DOMContentLoaded", () => {
  calculateInput = document.querySelector("#calculate-input");
  resultMsg = document.querySelector("#result-msg");
  document
    .querySelector("#calculate-button")
    .addEventListener("click", () => calculate());
});
