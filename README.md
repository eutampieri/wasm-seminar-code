# Seminario su WebAssembly

Slide disponibili [qui](https://eutampieri.gitlab.io/wasm-seminar)

## Esercizio

Realizzare un programma che calcoli l'n-esimo numero di Fibonacci.

L'esercizio può essere realizzato in Rust o in AssemblyScript.

### Setup di AssemblyScript

### Svolgimento in Rust

#### Setup di Rust

1. Visitare [rustup.rs](https://rustup.rs) e seguire le istruzioni
1. Completare la funzione in `src/rust/fib/src/lib.rs`
1. Compilare: `cargo build --release --target=wasm32-unknown-unknown`
1. Troverai il file `fib.wasm` in `src/rust/fib/target/wasm32-unknown-unknown/release/simple.wasm`

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

