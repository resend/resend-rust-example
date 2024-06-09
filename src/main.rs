use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let resend = Resend::new("re_123456789");

    let from = "Acme <onboarding@resend.dev>";
    let to = ["delivered@resend.dev"];
    let subject = "Hello World";

    let email = CreateEmailBaseOptions::new(from, to, subject)
        .with_html("<strong>It works!</strong>");

    let _email = resend.emails.send(email).await?;
    println!("{:?}", _email);

    Ok(())
}
