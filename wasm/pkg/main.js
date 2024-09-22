import init, { set_panic_hook, evaluate, evaluate_image } from "./wasm.js";

async function initialize() {
    await init();

    set_panic_hook();

    document.getElementById("run").addEventListener("click", () => run());
    document.getElementById("run-image").addEventListener("click", () => runImage());
}

function run() {
    const code = document.getElementById("input").value;
    const result = evaluate(code);

    const output = document.getElementById("output");
    output.value = result;
}

function runImage() {
    const code = document.getElementById("input").value;
    const result = evaluate_image(code);

    const blob = new Blob([result]);
    const image = document.getElementById("image");
    image.setAttribute("src", URL.createObjectURL(blob));
}

initialize();
