# Seminario su WebAssembly

Slide disponibili [qui](https://eutampieri.gitlab.io/wasm-seminar)

## Esercizio

Realizzare un programma che calcoli l'n-esimo numero di Fibonacci.

L'esercizio può essere realizzato in Rust o in AssemblyScript.

### Svolgimento in AssemblyScript

1. Aprire `src/assemblyscript/fib`
1. `npm i`
1. Implementare la funzione in `assembly/index.ts`
1. Scrivere i test in `tests/index.js` ed eseguirli tramite `npm run test` (opzionale)
1. Compilare eseguendo `npm run asbuild`
1. Testare con `npm start`

### Svolgimento in Rust

1. Visitare [rustup.rs](https://rustup.rs) e seguire le istruzioni
1. Completare la funzione in `src/rust/fib/src/lib.rs`
1. Scrivere i test in `src/rust/fib/src/test.rs` ed eseguirli tramite `cargo test --lib` (opzionale)
1. Compilare: `cargo build --release --target=wasm32-unknown-unknown`
1. Troverai il file `fib.wasm` in `src/rust/fib/target/wasm32-unknown-unknown/release/simple.wasm`
1. Eseguire `npx serve .` (o `python -m http.server`)
1. Testare tramite l'`index.html` fornito

#### Consigli
1. In Rust, le funzioni e le closure ritornano sempre (se si omette il valore da ritornare allora ritornano lo unit type `()`). Sotto alcuni esempi di codice legale.
   ```rust
   let a = if foo() {
     1
   } else {
     2
   };
   ```
   ```rust
   fn answer() -> u8 {
     42
   }
   ```
   ```rust
   fn answer_1(years: u8) -> u8 {
     match years {
       0 | 1 => 1,
       _ => 42,
     }
   }
   ```
1. Utilizzare il [costrutto `match`](https://doc.rust-lang.org/book/ch06-02-match.html), [altri esempi](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

## Integrazione con Vue

Nella sottocartella `vue` si può trovare un esempio di integrazione con Vue.

### Workflow:
- Per compilare il codice Rust si usa `wasm-pack build --target web`
- Copiare `hello_world/pkg/hello_world_bg.wasm` in `hello-vue/node_modules/.vite/deps/`
