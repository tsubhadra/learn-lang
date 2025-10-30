// Cargo.toml dependencies for Rust/Rusoto in July 2018:
// [dependencies]
// rusoto_core = "0.37"
// rusoto_s3 = "0.37"
// clap = "2.32"
// tokio-core = "0.1"
// futures = "0.1"
// serde = "1.0"
// serde_json = "1.0"
// serde_derive = "1.0"

extern crate rusoto_core;
extern crate rusoto_s3;
extern crate clap;
extern crate tokio_core;
extern crate futures;
extern crate serde_json;

use rusoto_core::Region;
use rusoto_s3::{
    S3, S3Client, GetBucketLifecycleConfigurationRequest, PutBucketLifecycleConfigurationRequest,
    DeleteBucketLifecycleRequest, BucketLifecycleConfiguration, LifecycleRule, LifecycleRuleFilter,
    Transition, LifecycleExpiration, ListObjectsV2Request, CopyObjectRequest,
};
use clap::{App, Arg, SubCommand, ArgMatches};
use tokio_core::reactor::Core;
use futures::Future;
use std::process;

fn main() {
    let matches = App::new("s3-lifecycle")
        .version("1.0")
        .about("AWS S3 Lifecycle and Archival Management CLI")
        .subcommand(
            SubCommand::with_name("list")
                .about("List all lifecycle rules for a bucket")
                .arg(
                    Arg::with_name("bucket")
                        .short("b")
                        .long("bucket")
                        .value_name("BUCKET")
                        .help("S3 bucket name")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new lifecycle rule")
                .arg(
                    Arg::with_name("bucket")
                        .short("b")
                        .long("bucket")
                        .value_name("BUCKET")
                        .help("S3 bucket name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .help("Rule ID")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("prefix")
                        .short("p")
                        .long("prefix")
                        .value_name("PREFIX")
                        .help("Prefix filter")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ia-days")
                        .long("ia-days")
                        .value_name("DAYS")
                        .help("Days until transition to STANDARD_IA")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("glacier-days")
                        .long("glacier-days")
                        .value_name("DAYS")
                        .help("Days until transition to GLACIER")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("deep-archive-days")
                        .long("deep-archive-days")
                        .value_name("DAYS")
                        .help("Days until transition to DEEP_ARCHIVE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("expiration-days")
                        .long("expiration-days")
                        .value_name("DAYS")
                        .help("Days until expiration (deletion)")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("enabled")
                        .long("enabled")
                        .help("Enable the rule (default: true)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a lifecycle rule")
                .arg(
                    Arg::with_name("bucket")
                        .short("b")
                        .long("bucket")
                        .value_name("BUCKET")
                        .help("S3 bucket name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .help("Rule ID to delete")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show lifecycle rule details")
                .arg(
                    Arg::with_name("bucket")
                        .short("b")
                        .long("bucket")
                        .value_name("BUCKET")
                        .help("S3 bucket name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .help("Rule ID to show")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("archive")
                .about("Archive objects with a specific prefix immediately")
                .arg(
                    Arg::with_name("bucket")
                        .short("b")
                        .long("bucket")
                        .value_name("BUCKET")
                        .help("S3 bucket name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("prefix")
                        .short("p")
                        .long("prefix")
                        .value_name("PREFIX")
                        .help("Prefix to archive")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("storage-class")
                        .short("s")
                        .long("storage-class")
                        .value_name("CLASS")
                        .help("Target storage class (GLACIER, DEEP_ARCHIVE)")
                        .default_value("GLACIER")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let mut core = Core::new().map_err(|e| format!("Failed to create event loop: {}", e))?;
    let client = S3Client::new(Region::default());

    match matches.subcommand() {
        ("list", Some(sub_m)) => {
            let bucket = sub_m.value_of("bucket").unwrap();
            list_lifecycle_rules(&mut core, &client, bucket)?;
        }
        ("create", Some(sub_m)) => {
            let bucket = sub_m.value_of("bucket").unwrap();
            let id = sub_m.value_of("id").unwrap();
            let prefix = sub_m.value_of("prefix");
            let ia_days = sub_m.value_of("ia-days").and_then(|d| d.parse::<i64>().ok());
            let glacier_days = sub_m.value_of("glacier-days").and_then(|d| d.parse::<i64>().ok());
            let deep_archive_days = sub_m.value_of("deep-archive-days").and_then(|d| d.parse::<i64>().ok());
            let expiration_days = sub_m.value_of("expiration-days").and_then(|d| d.parse::<i64>().ok());
            let enabled = sub_m.is_present("enabled");
            
            create_lifecycle_rule(
                &mut core,
                &client,
                bucket,
                id,
                prefix,
                ia_days,
                glacier_days,
                deep_archive_days,
                expiration_days,
                enabled,
            )?;
        }
        ("delete", Some(sub_m)) => {
            let bucket = sub_m.value_of("bucket").unwrap();
            let id = sub_m.value_of("id").unwrap();
            delete_lifecycle_rule(&mut core, &client, bucket, id)?;
        }
        ("show", Some(sub_m)) => {
            let bucket = sub_m.value_of("bucket").unwrap();
            let id = sub_m.value_of("id").unwrap();
            show_lifecycle_rule(&mut core, &client, bucket, id)?;
        }
        ("archive", Some(sub_m)) => {
            let bucket = sub_m.value_of("bucket").unwrap();
            let prefix = sub_m.value_of("prefix").unwrap();
            let storage_class = sub_m.value_of("storage-class").unwrap();
            archive_objects(&mut core, &client, bucket, prefix, storage_class)?;
        }
        _ => {
            println!("No subcommand provided. Use --help for usage information.");
        }
    }

    Ok(())
}

fn list_lifecycle_rules(
    core: &mut Core,
    client: &S3Client,
    bucket: &str,
) -> Result<(), String> {
    println!("Fetching lifecycle rules for bucket: {}", bucket);

    let request = GetBucketLifecycleConfigurationRequest {
        bucket: bucket.to_string(),
    };

    let result = core.run(client.get_bucket_lifecycle_configuration(request))
        .map_err(|e| format!("Failed to get lifecycle configuration: {}", e))?;

    if let Some(rules) = result.rules {
        if rules.is_empty() {
            println!("No lifecycle rules found.");
        } else {
            println!("\nLifecycle Rules:");
            println!("{:-<80}", "");
            for rule in rules {
                print_rule(&rule);
            }
        }
    } else {
        println!("No lifecycle rules found.");
    }

    Ok(())
}

fn create_lifecycle_rule(
    core: &mut Core,
    client: &S3Client,
    bucket: &str,
    id: &str,
    prefix: Option<&str>,
    ia_days: Option<i64>,
    glacier_days: Option<i64>,
    deep_archive_days: Option<i64>,
    expiration_days: Option<i64>,
    enabled: bool,
) -> Result<(), String> {
    // Get existing rules
    let get_request = GetBucketLifecycleConfigurationRequest {
        bucket: bucket.to_string(),
    };

    let mut existing_rules = match core.run(client.get_bucket_lifecycle_configuration(get_request)) {
        Ok(result) => result.rules.unwrap_or_default(),
        Err(_) => vec![],
    };

    // Build transitions
    let mut transitions = Vec::new();

    if let Some(days) = ia_days {
        transitions.push(Transition {
            days: Some(days),
            date: None,
            storage_class: Some("STANDARD_IA".to_string()),
        });
    }

    if let Some(days) = glacier_days {
        transitions.push(Transition {
            days: Some(days),
            date: None,
            storage_class: Some("GLACIER".to_string()),
        });
    }

    if let Some(days) = deep_archive_days {
        transitions.push(Transition {
            days: Some(days),
            date: None,
            storage_class: Some("DEEP_ARCHIVE".to_string()),
        });
    }

    // Build filter
    let filter = LifecycleRuleFilter {
        prefix: prefix.map(|p| p.to_string()),
        ..Default::default()
    };

    // Build expiration
    let expiration = expiration_days.map(|days| LifecycleExpiration {
        days: Some(days),
        date: None,
        expired_object_delete_marker: None,
    });

    // Build rule
    let new_rule = LifecycleRule {
        id: Some(id.to_string()),
        status: if enabled { "Enabled".to_string() } else { "Disabled".to_string() },
        filter: Some(filter),
        transitions: if transitions.is_empty() { None } else { Some(transitions) },
        expiration,
        ..Default::default()
    };

    // Remove existing rule with same ID
    existing_rules.retain(|r| r.id.as_ref().map(|i| i.as_str()) != Some(id));
    existing_rules.push(new_rule);

    // Apply configuration
    let put_request = PutBucketLifecycleConfigurationRequest {
        bucket: bucket.to_string(),
        lifecycle_configuration: Some(BucketLifecycleConfiguration {
            rules: existing_rules,
        }),
    };

    core.run(client.put_bucket_lifecycle_configuration(put_request))
        .map_err(|e| format!("Failed to create lifecycle rule: {}", e))?;

    println!("✓ Lifecycle rule '{}' created successfully for bucket '{}'", id, bucket);

    Ok(())
}

fn delete_lifecycle_rule(
    core: &mut Core,
    client: &S3Client,
    bucket: &str,
    id: &str,
) -> Result<(), String> {
    let get_request = GetBucketLifecycleConfigurationRequest {
        bucket: bucket.to_string(),
    };

    let result = core.run(client.get_bucket_lifecycle_configuration(get_request))
        .map_err(|e| format!("Failed to get lifecycle configuration: {}", e))?;

    let mut rules = result.rules.unwrap_or_default();
    let original_len = rules.len();
    rules.retain(|r| r.id.as_ref().map(|i| i.as_str()) != Some(id));

    if rules.len() == original_len {
        println!("Rule '{}' not found.", id);
        return Ok(());
    }

    if rules.is_empty() {
        let delete_request = DeleteBucketLifecycleRequest {
            bucket: bucket.to_string(),
        };
        core.run(client.delete_bucket_lifecycle(delete_request))
            .map_err(|e| format!("Failed to delete lifecycle configuration: {}", e))?;
        println!("✓ All lifecycle rules deleted from bucket '{}'", bucket);
    } else {
        let put_request = PutBucketLifecycleConfigurationRequest {
            bucket: bucket.to_string(),
            lifecycle_configuration: Some(BucketLifecycleConfiguration { rules }),
        };
        core.run(client.put_bucket_lifecycle_configuration(put_request))
            .map_err(|e| format!("Failed to update lifecycle configuration: {}", e))?;
        println!("✓ Lifecycle rule '{}' deleted from bucket '{}'", id, bucket);
    }

    Ok(())
}

fn show_lifecycle_rule(
    core: &mut Core,
    client: &S3Client,
    bucket: &str,
    id: &str,
) -> Result<(), String> {
    let request = GetBucketLifecycleConfigurationRequest {
        bucket: bucket.to_string(),
    };

    let result = core.run(client.get_bucket_lifecycle_configuration(request))
        .map_err(|e| format!("Failed to get lifecycle configuration: {}", e))?;

    if let Some(rules) = result.rules {
        if let Some(rule) = rules.iter().find(|r| r.id.as_ref().map(|i| i.as_str()) == Some(id)) {
            println!("\nLifecycle Rule Details:");
            println!("{:-<80}", "");
            print_rule(rule);
        } else {
            println!("Rule '{}' not found.", id);
        }
    } else {
        println!("No lifecycle rules found.");
    }

    Ok(())
}

fn archive_objects(
    core: &mut Core,
    client: &S3Client,
    bucket: &str,
    prefix: &str,
    storage_class: &str,
) -> Result<(), String> {
    println!("Archiving objects with prefix '{}' to {}", prefix, storage_class);

    let storage_class = match storage_class.to_uppercase().as_str() {
        "GLACIER" | "DEEP_ARCHIVE" => storage_class.to_uppercase(),
        _ => {
            return Err("Invalid storage class. Use: GLACIER or DEEP_ARCHIVE".to_string());
        }
    };

    let mut continuation_token = None;
    let mut total_objects = 0;

    loop {
        let list_request = ListObjectsV2Request {
            bucket: bucket.to_string(),
            prefix: Some(prefix.to_string()),
            max_keys: Some(1000),
            continuation_token: continuation_token.clone(),
            ..Default::default()
        };

        let list_result = core.run(client.list_objects_v2(list_request))
            .map_err(|e| format!("Failed to list objects: {}", e))?;

        if let Some(contents) = list_result.contents {
            for object in contents {
                if let Some(key) = object.key {
                    let copy_request = CopyObjectRequest {
                        bucket: bucket.to_string(),
                        key: key.clone(),
                        copy_source: format!("{}/{}", bucket, key),
                        storage_class: Some(storage_class.clone()),
                        metadata_directive: Some("COPY".to_string()),
                        ..Default::default()
                    };

                    core.run(client.copy_object(copy_request))
                        .map_err(|e| format!("Failed to archive object {}: {}", key, e))?;

                    total_objects += 1;
                    println!("  ✓ Archived: {}", key);
                }
            }
        }

        if list_result.is_truncated == Some(true) {
            continuation_token = list_result.next_continuation_token;
        } else {
            break;
        }
    }

    println!("\n✓ Archived {} objects to {}", total_objects, storage_class);
    Ok(())
}

fn print_rule(rule: &LifecycleRule) {
    println!("\nRule ID: {}", rule.id.as_ref().unwrap_or(&"N/A".to_string()));
    println!("Status: {}", rule.status);

    if let Some(ref filter) = rule.filter {
        if let Some(ref prefix) = filter.prefix {
            if !prefix.is_empty() {
                println!("Prefix: {}", prefix);
            }
        }
    }

    if let Some(ref transitions) = rule.transitions {
        println!("\nTransitions:");
        for t in transitions {
            if let Some(days) = t.days {
                println!(
                    "  - After {} days → {}",
                    days,
                    t.storage_class.as_ref().unwrap_or(&"N/A".to_string())
                );
            }
        }
    }

    if let Some(ref expiration) = rule.expiration {
        if let Some(days) = expiration.days {
            println!("\nExpiration: {} days", days);
        }
    }

    println!("{:-<80}", "");
}
