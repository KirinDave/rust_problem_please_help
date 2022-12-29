use google_cloud_gax::cancel::CancellationToken;
use google_cloud_gax::grpc::Status;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Status> {
    // Create pubsub client.
    // The default project is determined by credentials.
    // - If the GOOGLE_APPLICATION_CREDENTIALS is specified the project_id is from credentials.
    // - If the server is running on GCP the project_id is from metadata server
    // - If the PUBSUB_EMULATOR_HOST is specified the project_id is 'local-project'
    let client = Client::default().await.unwrap();

    // Get the topic to subscribe to.
    let topic = client.topic("test-topic");

    // Configure subscription.
    let mut config = SubscriptionConfig::default();
    // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
    config.enable_message_ordering = true;

    // Create subscription
    // If subscription name does not contain a "/", then the project is taken from client above. Otherwise, the
    // name will be treated as a fully qualified resource name
    let subscription = client.subscription("test-subscription");
    if !subscription.exists(None, None).await? {
        subscription
            .create(topic.fully_qualified_name(), config, None, None)
            .await?;
    }
    // Token for cancel.
    let cancel = CancellationToken::new();
    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        // Cancel after 10 seconds.
        tokio::time::sleep(Duration::from_secs(10)).await;
        cancel2.cancel();
    });

    let (sender, _receiver) = tokio::sync::mpsc::channel::<String>(64);
    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.

    subscription
        .receive(
            move |message, _cancel| {
                let sender = sender.clone();
                async move {
                    // Handle data.
                    let data = message.message.data.as_slice();
                    let data_string = String::from_utf8(message.message.data.clone()).unwrap();
                    sender.send(data_string).await.unwrap();
                    println!("{:?}", data);

                    // Ack or Nack message.
                    message.ack().await.unwrap();
                }
            },
            cancel.clone(),
            None,
        )
        .await
        .unwrap();

    // Delete subscription if needed.
    subscription.delete(None, None).await.unwrap();

    Ok(())
}
