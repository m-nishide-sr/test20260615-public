use hello_cargo::{fibonacci, fibonacci_loop, fibonacci_async}; // 👈 ライブラリからインポート

#[tokio::main]
async fn main() {
   println!("Hello, world!");
   println!("fibonacci_async(30):{}", fibonacci_loop(30));
   println!("fibonacci_async(30):{}", fibonacci(30));
   println!("fibonacci_async(30):{}", fibonacci_async(30).await);
}
