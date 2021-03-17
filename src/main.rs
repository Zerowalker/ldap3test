use ldap3::{LdapConnAsync, LdapConnSettings, LdapError};
#[derive(Debug, PartialEq)]
enum ConnType {
    Ldap = 0,
    StartTls = 1,
    Ldaps = 2,
}

fn get_conn_settings(server: &str, conn_type: &ConnType) -> (LdapConnSettings, String) {
    let url = {
        match conn_type {
            // If connection type is Ldap or StartTls, connect via "ldap://"
            ConnType::Ldap | ConnType::StartTls => format!("ldap://{}", server),
            // If connection type is Ldaps, connect via "ldaps://"
            ConnType::Ldaps => format!("ldaps://{}", server),
        }
    };
    // Always ignore certificate verification to simplify testing
    // Only enable starttls when the connection type specify it (just to make it clearer)
    (
        LdapConnSettings::new()
            .set_starttls(conn_type == &ConnType::StartTls)
            .set_no_tls_verify(true),
        url,
    )
}

async fn test(
    user: &str,
    pass: &str,
    url: &str,
    ldap_conn_settings: LdapConnSettings,
) -> std::result::Result<(), LdapError> {
    // Create ocnnection with the previous settings
    let (conn, mut ldap) = LdapConnAsync::with_settings(ldap_conn_settings, &url).await?;
    ldap3::drive!(conn);
    //
    let _res = match ldap.simple_bind(user, pass).await {
        // On result check for success (not sure if it served a point or not as you can do it forever?)
        Ok(o) => match o.success() {
            Ok(o) => o,
            Err(e) => {
                // println!("simple_bind failed on success()?");
                return Err(e);
            }
        },
        Err(e) => {
            // println!("simple_bind failed");
            return Err(e);
        }
    };

    match ldap.unbind().await {
        Ok(_) => {}
        Err(e) => {
            // println!("unbind failed");
            return Err(e);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let options: Vec<String> = std::env::args().collect();
    //example: cargo run user@domain.local secret123 127.0.0.1
    println!("Arguments: {:?}", options);
    let user: String = options[1].clone();
    let pass: String = options[2].clone();
    let server: String = options[3].clone();

    for t in 0..3 {
        let conn_type: ConnType = match t {
            0 => ConnType::Ldap,
            1 => ConnType::StartTls,
            2 => ConnType::Ldaps,
            _ => panic!("invalid match"),
        };
        for i in 0..10 {
            let (conn_settings, url) = get_conn_settings(&server, &conn_type);
            let res = match test(&user, &pass, &url, conn_settings.clone()).await {
                Ok(_) => "SUCCEDED",
                Err(_) => " FAILED ",
            };
            let spec = format!("url: {}, StartTls: {}", url, conn_settings.starttls());
            println!("[{:?}]: #{} - [{}] - [{}]", conn_type, i, res, spec);
        }
    }
}
