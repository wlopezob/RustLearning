use std::time::Duration;

use tokio::{time::sleep};

#[tokio::main]
async fn hilo() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2: String = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB result".to_owned()
}