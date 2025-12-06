use misp_client::{MispClientFactory, SearchBuilder};
use std::env;

#[tokio::main]
async fn main() -> misp_client::Result<()> {
    let base_url = env::var("MISP_URL").unwrap_or_else(|_| "https://localhost".to_string());
    let api_key = env::var("MISP_API_KEY").expect("MISP_API_KEY required");

    let misp = MispClientFactory::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .verify_ssl(false)
        .build();

    let suspicious_ip = "192.168.100.50";
    println!("Enriching: {}\n", suspicious_ip);

    let attrs = misp.attributes().search_by_value(suspicious_ip).await?;

    if !attrs.is_empty() {
        println!("MISP CONTEXT FOUND:");
        for attr in &attrs {
            println!("  Event ID: {}", attr.event_id);
            println!("  Type: {}", attr.attr_type);
            println!("  Category: {}", attr.category);
            println!("  To IDS: {}", attr.to_ids);
            if let Some(comment) = &attr.comment {
                println!("  Comment: {}", comment);
            }

            let event = misp.events().get(&attr.event_id).await?;
            println!("  Campaign: {}", event.info);
            println!("  Threat Level: {:?}", event.threat_level_id);
            println!("  Analysis: {:?}", event.analysis);

            let related_attrs = misp.events().get_attributes(&attr.event_id).await?;
            println!("  Related IOCs in event: {}", related_attrs.len());
            for related in related_attrs.iter().take(5) {
                if related.id != attr.id {
                    println!("    - {} ({}): {}", related.attr_type, related.category, related.value);
                }
            }
        }
    } else {
        println!("  No context found for {} in MISP", suspicious_ip);
    }

    println!("\n  Warninglist check...");
    if misp.warninglists().is_whitelisted(suspicious_ip).await? {
        println!("  On warninglist - possible false positive");
    } else {
        println!("  Not on any warninglist");
    }

    println!("\n  Sightings...");
    let sighting_count = misp.sightings().count_for_value(suspicious_ip).await?;
    println!("  Total sightings: {}", sighting_count.total);
    println!("  Positive: {}, Negative: {}", sighting_count.positive, sighting_count.negative);

    println!("\n--- Threat Hunting ---\n");

    let events = misp.events();
    let attributes = misp.attributes();
    let _search = SearchBuilder::new(&events, &attributes);

    println!("IP IOCs (to_ids=true):");

    let ip_iocs = attributes
        .search(
            misp_client::AttributeSearchQuery::new()
                .attr_type("ip-dst")
                .to_ids(true)
                .limit(100),
        )
        .await?;

    println!("  {} ip-dst", ip_iocs.len());
    for ioc in ip_iocs.iter().take(5) {
        println!("    {} (Event: {})", ioc.value, ioc.event_id);
    }

    println!("\nFile hashes:");

    let md5_iocs = attributes.search_by_type("md5").await?;
    let sha256_iocs = attributes.search_by_type("sha256").await?;

    println!("  {} md5, {} sha256", md5_iocs.len(), sha256_iocs.len());

    let domain_iocs = attributes.search_by_type("domain").await?;
    println!("  {} domains", domain_iocs.len());

    println!("\n--- Galaxies ---\n");
    let galaxies = misp.galaxies();
    if let Some(ta_galaxy) = galaxies.get_threat_actors().await? {
        println!("  - Threat Actors: {} clusters", ta_galaxy.clusters.len());
    }
    if let Some(attack_galaxy) = galaxies.get_mitre_attack().await? {
        println!("  - MITRE ATT&CK: {} techniques", attack_galaxy.clusters.len());
    }
    if let Some(malware_galaxy) = galaxies.get_malware().await? {
        println!("  - Malware: {} families", malware_galaxy.clusters.len());
    }

    Ok(())
}
