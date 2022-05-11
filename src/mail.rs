use anyhow::Result;

use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

use crate::config;

fn get_smtp_password(_pass_command: &str) -> Result<String> {
    let sh = xshell::Shell::new()?;
    let password = sh.cmd("pass").arg("purelymail.com").read()?;
    dbg!(&password);

    Ok(password)
}

pub fn new_transport(config: config::Smtp) -> Result<SmtpTransport> {
    let creds = Credentials::new(config.user, get_smtp_password(&config.pass_command)?);

    Ok(SmtpTransport::relay(&config.host)
        .unwrap()
        .credentials(creds)
        .build())
}
