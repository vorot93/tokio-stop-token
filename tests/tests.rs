use std::time::Duration;
use stop_token::StopSource;
use tokio::stream::StreamExt;

#[tokio::test]
async fn smoke() {
    let (mut sender, receiver) = tokio::sync::mpsc::channel::<i32>(10);
    let stop_source = StopSource::new();
    let task = tokio::spawn({
        let stop_token = stop_source.stop_token();
        async move {
            let mut xs = Vec::new();
            let mut stream = stop_token.stop_stream(receiver);
            while let Some(x) = stream.next().await {
                xs.push(x)
            }
            xs
        }
    });
    sender.send(1).await.unwrap();
    sender.send(2).await.unwrap();
    sender.send(3).await.unwrap();

    tokio::time::delay_for(Duration::from_millis(250)).await;
    drop(stop_source);
    tokio::time::delay_for(Duration::from_millis(250)).await;

    sender.send(4).await.unwrap();
    sender.send(5).await.unwrap();
    sender.send(6).await.unwrap();
    assert_eq!(task.await.unwrap(), vec![1, 2, 3]);
}
