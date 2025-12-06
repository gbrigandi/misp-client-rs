use misp_client::{MispClientFactory, SearchBuilder};
use std::env;

#[tokio::main]
async fn main() -> misp_client::Result<()> {
    let misp = MispClientFactory::builder()
        .base_url(env::var("MISP_URL").expect("MISP_URL required"))
        .api_key(env::var("MISP_API_KEY").expect("MISP_API_KEY required"))
        .verify_ssl(false)
        .build();

    let events = misp.events();
    let attributes = misp.attributes();
    let search = SearchBuilder::new(&events, &attributes);

    println!("Recent IOCs (24h, to_ids=true):");
    let recent_iocs = search
        .recent("24h")
        .to_ids_only()
        .types(vec![
            "ip-dst", "ip-src", "domain", "hostname", "md5", "sha256",
        ])
        .limit(50)
        .attributes()
        .await?;

    println!("  {} IOCs", recent_iocs.len());
    for ioc in recent_iocs.iter().take(5) {
        println!("  - {}: {}", ioc.attr_type, ioc.value);
    }

    println!("\nRansomware tagged events:");
    let ransomware_events = search
        .by_tag("malware:ransomware")
        .published_only()
        .limit(10)
        .events()
        .await?;

    println!("  {} events", ransomware_events.len());
    for event in ransomware_events.iter().take(3) {
        println!("  - [{}] {}", event.id, event.info);
    }

    println!("\nAPT29 events:");
    let apt_events = search.threat_actor("APT29").limit(5).execute().await?;

    println!("  {} events", apt_events.len());
    for event in &apt_events {
        println!("  - [{}] {}", event.id, event.info);
    }

    println!("\nIP IOCs (7d, tlp:white/green):");
    let ip_iocs = search
        .recent("7d")
        .types(vec!["ip-dst", "ip-src"])
        .to_ids_only()
        .tags(vec!["tlp:white", "tlp:green"])
        .limit(100)
        .attributes()
        .await?;

    println!("  {} IPs", ip_iocs.len());
    for ip in ip_iocs.iter().take(10) {
        println!("  {}", ip.value);
    }

    Ok(())
}
