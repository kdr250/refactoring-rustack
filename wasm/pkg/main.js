import init, { greet } from "./wasm.js";

async function run() {
    await init();

    greet("Taro from main.js");
}

run();
