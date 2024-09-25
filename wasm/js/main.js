import { set_panic_hook, evaluate, evaluate_image } from "../pkg/index.js";

initialize();

function initialize() {
    set_panic_hook();

    generateSamples();

    document.getElementById("run").addEventListener("click", () => run());
    document.getElementById("run-image").addEventListener("click", () => runImage());
}

function generateSamples() {
    const samples = document.getElementById("samples");

    ["while_gray.txt", "mandel.txt"]
        .forEach(fileName => {
            const link = document.createElement("a");
            link.href = "#";
            link.addEventListener("click", () => {
                fetch("scripts/" + fileName)
                    .then(file => file.text())
                    .then(text => {
                        document.getElementById("input").value = text
                    });
            });
            link.innerHTML = fileName;
            samples.appendChild(link);
            samples.append(" ");
        })
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
