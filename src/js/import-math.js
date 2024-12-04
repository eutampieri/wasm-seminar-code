let fact;
WebAssembly.instantiateStreaming(fetch("target/wasm32-unknown-unknown/release/wasm_demo.wasm"), {}).then(
    (obj) => {
        console.log(obj);
        fact = (x) => obj.instance.exports.fact(BigInt(x))
    },
);

function calc() {
    document.getElementById("fact").innerText = fact(parseInt(document.getElementById("n").value)).toString();
}
