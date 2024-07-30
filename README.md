#HOW TO USE

```rust
#[tokio::main]
use brevo::{Brevo, Mailer, TransactionalBody};

async fn main() -> Result<(), Box<dyn std::error::Eror>> {
    // Email sender.
    let sender = Mailer::new("full name", "sender@example.org");

    // Email receiver.
    let receiver = Mailer::new("full name", "reveiver@example.org");

    // Email body.
    let body = TransactionalBody::builder()
        .set_sender(sender)
        .add_to_mailer(receiver)
        .reply_to(sender)
        .subject("Subject".to_string())
        .template_id(1)
        .add_params("param_name", "param_value".to_string())
        .create();

    // Email client.
    let client = Brevo::production("API_KEY".to_string());

    // Send email.
    client.send_transactional_email(body).await?;
}
```

