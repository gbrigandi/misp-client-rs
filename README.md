# MISP Client

Rust client for the MISP threat intelligence platform API.

## Usage

```rust
use misp_client::MispClientFactory;

let misp = MispClientFactory::builder()
    .base_url("https://misp.local")
    .api_key("your-api-key")
    .verify_ssl(false)
    .build();

// search attributes
let attrs = misp.attributes().search_by_value("8.8.8.8").await?;

// get event details
let event = misp.events().get("123").await?;

// check warninglists
let on_list = misp.warninglists().is_whitelisted("8.8.8.8").await?;
```

## Clients

`MispClientFactory` provides access to:

* `events()` for event queries
* `attributes()` for IOC searches
* `galaxies()` for threat actors, MITRE ATT&CK, malware
* `sightings()` for observation tracking
* `warninglists()` for false positive checks

## Example

IOC enrichment with context lookup, sighting history, and warninglist check:

```rust
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

    let results = misp.attributes().search_by_value(ioc).await?;

    if results.is_empty() {
        println!("  no matches");
    } else {
        println!("  {} matches:", results.len());
        for attr in &results {
            println!("  Event #{}: {} ({})", attr.event_id, attr.value, attr.attr_type);
        }
    }

    let count = misp.sightings().count_for_value(ioc).await?;
    println!("\nSightings: {} total ({} pos, {} neg)",
        count.total, count.positive, count.negative);

    if misp.warninglists().is_whitelisted(ioc).await? {
        println!("\nOn warninglist");
    } else {
        println!("\nNot on any warninglist");
    }

    Ok(())
}
```

Run with:

```
MISP_URL=https://misp.local MISP_API_KEY=xxx cargo run --example enrichment
```

See `examples/` for more.

## License

MIT
