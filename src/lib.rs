mod client;
mod mailer;
mod transactional;

pub use mailer::Mailer;
pub use transactional::*;

pub type Brevo = client::Client;
