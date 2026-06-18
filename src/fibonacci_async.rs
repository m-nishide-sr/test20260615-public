use std::future::Future;
use std::pin::Pin;

// フィボナッチ(非同期再帰)
pub fn fibonacci_async(n: u64) -> Pin<Box<dyn Future<Output = u64> + Send>> {
    Box::pin(async move {
        match n {
            0 => 1,
            1 => 1,
            n => {
                // 再帰呼び出しのたびに、Box::pin を通してヒープに退避させる
                fibonacci_async(n - 1).await + fibonacci_async(n - 2).await
            }
        }
    })
}