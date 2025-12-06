use misp_client::{MispClientFactory, SearchBuilder};
use std::env;

#[tokio::main]
async fn main() -> misp_client::Result<()> {
    let base_url = env::var("MISP_URL").unwrap_or_else(|_| "https://localhost".to_string());
    let api_key = env::var("MISP_API_KEY").expect("MISP_API_KEY required");

    println!("MISP integration test\n");
    println!("URL: {}", base_url);

    let misp = MispClientFactory::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .verify_ssl(false)
        .build();

    println!("\nConnection...");
    match misp.test_connection().await {
        Ok(info) => {
            println!("  ok: MISP {}", info.version);
            println!(
                "  perm_sync: {}, perm_sighting: {}",
                info.perm_sync, info.perm_sighting
            );
        }
        Err(e) => {
            println!("  err: {}", e);
            return Err(e);
        }
    }

    println!("\nEvents...");
    let events_client = misp.events();
    match events_client.index(None).await {
        Ok(events) => {
            println!("  {} events", events.len());
            for event in events.iter().take(3) {
                println!("  - [{}] {}", event.id, event.info);
            }
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nAttribute types...");
    let attributes_client = misp.attributes();
    match attributes_client.describe_types().await {
        Ok(types) => {
            println!(
                "  {} types, {} categories",
                types.types.len(),
                types.categories.len()
            );
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nGalaxies...");
    let galaxies_client = misp.galaxies();
    match galaxies_client.list().await {
        Ok(galaxies) => {
            println!("  {} galaxies", galaxies.len());
            for galaxy in galaxies.iter().take(3) {
                println!("  - {} ({})", galaxy.name, galaxy.galaxy_type);
            }
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nWarninglists...");
    let warninglists_client = misp.warninglists();
    match warninglists_client.list().await {
        Ok(wls) => {
            let enabled = wls.iter().filter(|w| w.enabled == Some(true)).count();
            println!("  {} total, {} enabled", wls.len(), enabled);
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nWarninglist check (8.8.8.8)...");
    match warninglists_client.check_value("8.8.8.8").await {
        Ok(result) => {
            if result.matched {
                println!("  matched {} list(s)", result.warninglists.len());
            } else {
                println!("  no match");
            }
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nThreat actors galaxy...");
    match galaxies_client.get_threat_actors().await {
        Ok(Some(galaxy)) => {
            println!("  {} clusters", galaxy.clusters.len());
        }
        Ok(None) => println!("  not found"),
        Err(e) => println!("  err: {}", e),
    }

    println!("\nRecent attributes (30d)...");
    let search = SearchBuilder::new(&events_client, &attributes_client);
    match search.recent("30d").limit(10).attributes().await {
        Ok(attrs) => {
            println!("  {} attributes", attrs.len());
            for attr in attrs.iter().take(3) {
                println!("  - {}: {}", attr.attr_type, attr.value);
            }
        }
        Err(e) => println!("  err: {}", e),
    }

    println!("\nDone");
    Ok(())
}
