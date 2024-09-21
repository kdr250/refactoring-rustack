import init, { set_panic_hook, evaluate } from "./wasm.js";

async function initialize() {
    await init();

    set_panic_hook();

    document.getElementById("run").addEventListener("click", () => run());
}

function run() {
    const code = document.getElementById("input").value;
    const result = evaluate(code);

    const output = document.getElementById("output");
    output.value = result;
}

initialize();
