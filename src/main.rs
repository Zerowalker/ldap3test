use ldap3::{LdapConnAsync, LdapConnSettings, LdapError};

async fn test() -> std::result::Result<(), LdapError> {
    let (conn, mut ldap) = LdapConnAsync::with_settings(
        LdapConnSettings::new()
            .set_starttls(true)
            .set_no_tls_verify(true),
        &format!("ldaps://{}", "10.0.0.1"),
    )
    .await?;
    println!("binding");
    ldap3::drive!(conn);

    let _res = ldap.simple_bind("test@test.test", "123").await?.success()?;

    println!("{:?}", _res);

    println!("ldap ended normally");
    ldap.unbind().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    test().await.unwrap();
    return;
}
