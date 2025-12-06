use misp_client::MispClientFactory;
use std::env;

#[tokio::main]
async fn main() -> misp_client::Result<()> {
    let misp = MispClientFactory::builder()
        .base_url(env::var("MISP_URL").expect("MISP_URL required"))
        .api_key(env::var("MISP_API_KEY").expect("MISP_API_KEY required"))
        .verify_ssl(false)
        .build();

    let info = misp.test_connection().await?;
    println!("MISP {}\n", info.version);

    let ioc = "192.168.1.100";
    println!("Lookup: {}", ioc);

    let attributes = misp.attributes();
    let results = attributes.search_by_value(ioc).await?;

    if results.is_empty() {
        println!("  no matches");
    } else {
        println!("  {} matches:", results.len());
        for attr in &results {
            println!(
                "  - Event #{}: {} ({})",
                attr.event_id, attr.value, attr.attr_type
            );
            if !attr.tags.is_empty() {
                let tags: Vec<_> = attr.tags.iter().map(|t| t.name.as_str()).collect();
                println!("    tags: {}", tags.join(", "));
            }
        }
    }

    let count = misp.sightings().count_for_value(ioc).await?;
    println!(
        "\nSightings: {} total ({} pos, {} neg)",
        count.total, count.positive, count.negative
    );

    let warninglists = misp.warninglists();
    if warninglists.is_whitelisted(ioc).await? {
        let matches = warninglists.get_matching_lists(ioc).await?;
        println!("\nWarninglist matches:");
        for m in matches {
            println!("  - {}", m.name);
        }
    } else {
        println!("\nNot on any warninglist");
    }

    let galaxies = misp.galaxies();
    if let Some(actor) = galaxies.search_threat_actor("APT28").await?.first() {
        println!("\nThreat actor: {}", actor.value);
        if let Some(desc) = &actor.description {
            println!("  {}", desc);
        }
    }

    Ok(())
}
