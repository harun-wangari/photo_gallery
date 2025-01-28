use lettre::{message::{MultiPart, SinglePart}, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use dotenvy_macro::dotenv;

use super::errors::DataError;

pub async fn send_email(email_address:String,message:String,subject:String) -> Result<bool,DataError> {
    let html = r#"
    <h4>Hello, </h4>
    <p>click the reset password link below to reset  your password</p>
    <a href='' style='height:60px;background-color: #ffa510;color:#000011;padding:10px;border-radius:4px;text-decoration:none'>Reset Password</a>
    <p> If you did not request a password request Please ingore this email.Please note that this link will expire in 30 minutes</p>
    "#;
    let sender_address = dotenv!("MAIL_USERNAME").to_string();
    let email = Message::builder()
        .from(sender_address.parse().unwrap())
        .to(email_address.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative().singlepart(SinglePart::html(html.to_string())),
        )
        .unwrap();

    let credentials = Credentials::new(sender_address,dotenv!("MAIL_PASSWORD").to_string());
    let mailer = SmtpTransport::relay(dotenv!("MAIL_SERVER"))
        .unwrap()
        .credentials(credentials)
        .build();

    match mailer.send(&email){
        Ok(_) => Ok(true),
        Err(e) => Err(DataError::QueryError(e.to_string())),
    }
}