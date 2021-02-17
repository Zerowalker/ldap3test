use std::{net::Ipv4Addr, str::FromStr};

use ldap3::{LdapConnAsync, LdapConnSettings, LdapError};

#[derive(Debug, PartialEq)]
enum ConnType {
    Ldap = 0,
    StartTls = 1,
    Ldaps = 2,
}

async fn test(
    user: &str,
    pass: &str,
    ip: &Ipv4Addr,
    conn_type: &ConnType,
) -> std::result::Result<(), LdapError> {
    let url = {
        match conn_type {
            // If connection type is Ldap or StartTls, connect via "ldap://"
            ConnType::Ldap | ConnType::StartTls => format!("ldap://{}", ip),
            // If connection type is Ldaps, connect via "ldaps://"
            ConnType::Ldaps => format!("ldaps://{}", ip),
        }
    };
    // Always ignore certificate verification to simplify testing
    // Only enable starttls when the connection type specify it (just to make it clearer)
    let ldap_conn_settings = LdapConnSettings::new()
        .set_starttls(conn_type == &ConnType::StartTls)
        .set_no_tls_verify(true);

    println!("url: {}, StartTls: {}", url, ldap_conn_settings.starttls());
    // Create ocnnection with the previous settings
    let (conn, mut ldap) = LdapConnAsync::with_settings(ldap_conn_settings, &url).await?;
    ldap3::drive!(conn);

    //
    let _res = match ldap.simple_bind(user, pass).await {
        // On result check for success (not sure if it served a point or not as you can do it forever?)
        Ok(o) => match o.success() {
            Ok(o) => o,
            Err(e) => {
                println!("simple_bind failed on success()?");
                return Err(e);
            }
        },
        Err(e) => {
            println!("simple_bind failed");
            return Err(e);
        }
    };

    match ldap.unbind().await {
        Ok(_) => {}
        Err(e) => {
            println!("unbind failed");
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
    let ip = Ipv4Addr::from_str(&options[3]).unwrap();
    let mut conn_type = ConnType::Ldap;
    println!("Testing ConnType: {:?}", conn_type);

    for i in 0..5 {
        println!("-----Test [{:?}]: #{}-----", conn_type, i);
        test(&user, &pass, &ip, &conn_type).await.unwrap();
        println!("-----Test Done-----");
    }
    conn_type = ConnType::StartTls;
    for i in 0..5 {
        println!("-----Test [{:?}]: #{}-----", conn_type, i);
        test(&user, &pass, &ip, &conn_type).await.unwrap();
        println!("-----Test Done-----");
    }
    conn_type = ConnType::Ldaps;
    for i in 0..5 {
        println!("-----Test [{:?}]: #{}-----", conn_type, i);
        test(&user, &pass, &ip, &conn_type).await.unwrap();
        println!("-----Test Done-----");
    }

    println!("All tests completed successfully!")
}
