// Cargo.toml dependencies needed:
// [dependencies]
// aws-config = "1.1"
// aws-sdk-s3 = "1.13"
// clap = { version = "4.4", features = ["derive"] }
// tokio = { version = "1.35", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// anyhow = "1.0"

use anyhow::{Context, Result};
use aws_sdk_s3::types::{
    LifecycleConfiguration, LifecycleRule, LifecycleRuleFilter, ExpirationStatus,
    Transition, TransitionStorageClass, LifecycleExpiration, NoncurrentVersionTransition,
    NoncurrentVersionExpiration,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "s3-lifecycle")]
#[command(about = "AWS S3 Lifecycle and Archival Management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all lifecycle rules for a bucket
    List {
        /// S3 bucket name
        #[arg(short, long)]
        bucket: String,
    },
    /// Create a new lifecycle rule
    Create {
        /// S3 bucket name
        #[arg(short, long)]
        bucket: String,
        /// Rule ID
        #[arg(short, long)]
        id: String,
        /// Prefix filter (optional)
        #[arg(short, long)]
        prefix: Option<String>,
        /// Days until transition to STANDARD_IA
        #[arg(long)]
        ia_days: Option<i32>,
        /// Days until transition to GLACIER
        #[arg(long)]
        glacier_days: Option<i32>,
        /// Days until transition to DEEP_ARCHIVE
        #[arg(long)]
        deep_archive_days: Option<i32>,
        /// Days until expiration (deletion)
        #[arg(long)]
        expiration_days: Option<i32>,
        /// Enable the rule
        #[arg(long, default_value = "true")]
        enabled: bool,
    },
    /// Delete a lifecycle rule
    Delete {
        /// S3 bucket name
        #[arg(short, long)]
        bucket: String,
        /// Rule ID to delete
        #[arg(short, long)]
        id: String,
    },
    /// Show lifecycle rule details
    Show {
        /// S3 bucket name
        #[arg(short, long)]
        bucket: String,
        /// Rule ID to show
        #[arg(short, long)]
        id: String,
    },
    /// Archive objects with a specific prefix immediately
    Archive {
        /// S3 bucket name
        #[arg(short, long)]
        bucket: String,
        /// Prefix to archive
        #[arg(short, long)]
        prefix: String,
        /// Target storage class (GLACIER, DEEP_ARCHIVE, GLACIER_IR)
        #[arg(short, long, default_value = "GLACIER")]
        storage_class: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    match cli.command {
        Commands::List { bucket } => list_lifecycle_rules(&client, &bucket).await?,
        Commands::Create {
            bucket,
            id,
            prefix,
            ia_days,
            glacier_days,
            deep_archive_days,
            expiration_days,
            enabled,
        } => {
            create_lifecycle_rule(
                &client,
                &bucket,
                &id,
                prefix.as_deref(),
                ia_days,
                glacier_days,
                deep_archive_days,
                expiration_days,
                enabled,
            )
            .await?
        }
        Commands::Delete { bucket, id } => delete_lifecycle_rule(&client, &bucket, &id).await?,
        Commands::Show { bucket, id } => show_lifecycle_rule(&client, &bucket, &id).await?,
        Commands::Archive {
            bucket,
            prefix,
            storage_class,
        } => archive_objects(&client, &bucket, &prefix, &storage_class).await?,
    }

    Ok(())
}

async fn list_lifecycle_rules(client: &aws_sdk_s3::Client, bucket: &str) -> Result<()> {
    println!("Fetching lifecycle rules for bucket: {}", bucket);
    
    match client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket)
        .send()
        .await
    {
        Ok(output) => {
            if let Some(rules) = output.rules {
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
        }
        Err(e) => {
            if e.to_string().contains("NoSuchLifecycleConfiguration") {
                println!("No lifecycle configuration found for this bucket.");
            } else {
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}

async fn create_lifecycle_rule(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    id: &str,
    prefix: Option<&str>,
    ia_days: Option<i32>,
    glacier_days: Option<i32>,
    deep_archive_days: Option<i32>,
    expiration_days: Option<i32>,
    enabled: bool,
) -> Result<()> {
    // Get existing rules
    let mut existing_rules = match client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket)
        .send()
        .await
    {
        Ok(output) => output.rules.unwrap_or_default(),
        Err(_) => vec![],
    };

    // Build transitions
    let mut transitions = Vec::new();
    
    if let Some(days) = ia_days {
        transitions.push(
            Transition::builder()
                .days(days)
                .storage_class(TransitionStorageClass::StandardIa)
                .build(),
        );
    }
    
    if let Some(days) = glacier_days {
        transitions.push(
            Transition::builder()
                .days(days)
                .storage_class(TransitionStorageClass::Glacier)
                .build(),
        );
    }
    
    if let Some(days) = deep_archive_days {
        transitions.push(
            Transition::builder()
                .days(days)
                .storage_class(TransitionStorageClass::DeepArchive)
                .build(),
        );
    }

    // Build filter
    let filter = if let Some(p) = prefix {
        LifecycleRuleFilter::Prefix(p.to_string())
    } else {
        LifecycleRuleFilter::Prefix(String::new())
    };

    // Build rule
    let mut rule_builder = LifecycleRule::builder()
        .id(id)
        .filter(filter)
        .status(if enabled {
            ExpirationStatus::Enabled
        } else {
            ExpirationStatus::Disabled
        });

    for transition in transitions {
        rule_builder = rule_builder.transitions(transition);
    }

    if let Some(days) = expiration_days {
        rule_builder = rule_builder.expiration(
            LifecycleExpiration::builder().days(days).build(),
        );
    }

    let new_rule = rule_builder.build()?;

    // Remove existing rule with same ID if it exists
    existing_rules.retain(|r| r.id.as_deref() != Some(id));
    existing_rules.push(new_rule);

    // Apply configuration
    let lifecycle_config = LifecycleConfiguration::builder()
        .set_rules(Some(existing_rules))
        .build()?;

    client
        .put_bucket_lifecycle_configuration()
        .bucket(bucket)
        .lifecycle_configuration(lifecycle_config)
        .send()
        .await
        .context("Failed to create lifecycle rule")?;

    println!("✓ Lifecycle rule '{}' created successfully for bucket '{}'", id, bucket);
    
    Ok(())
}

async fn delete_lifecycle_rule(client: &aws_sdk_s3::Client, bucket: &str, id: &str) -> Result<()> {
    // Get existing rules
    let output = client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket)
        .send()
        .await
        .context("Failed to get lifecycle configuration")?;

    let mut rules = output.rules.unwrap_or_default();
    
    let original_len = rules.len();
    rules.retain(|r| r.id.as_deref() != Some(id));

    if rules.len() == original_len {
        println!("Rule '{}' not found.", id);
        return Ok(());
    }

    if rules.is_empty() {
        // Delete entire lifecycle configuration if no rules left
        client
            .delete_bucket_lifecycle()
            .bucket(bucket)
            .send()
            .await
            .context("Failed to delete lifecycle configuration")?;
        println!("✓ All lifecycle rules deleted from bucket '{}'", bucket);
    } else {
        // Update with remaining rules
        let lifecycle_config = LifecycleConfiguration::builder()
            .set_rules(Some(rules))
            .build()?;

        client
            .put_bucket_lifecycle_configuration()
            .bucket(bucket)
            .lifecycle_configuration(lifecycle_config)
            .send()
            .await
            .context("Failed to update lifecycle configuration")?;
        
        println!("✓ Lifecycle rule '{}' deleted from bucket '{}'", id, bucket);
    }

    Ok(())
}

async fn show_lifecycle_rule(client: &aws_sdk_s3::Client, bucket: &str, id: &str) -> Result<()> {
    let output = client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket)
        .send()
        .await
        .context("Failed to get lifecycle configuration")?;

    if let Some(rules) = output.rules {
        if let Some(rule) = rules.iter().find(|r| r.id.as_deref() == Some(id)) {
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

async fn archive_objects(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    prefix: &str,
    storage_class: &str,
) -> Result<()> {
    println!("Archiving objects with prefix '{}' to {}", prefix, storage_class);

    let storage_class_enum = match storage_class.to_uppercase().as_str() {
        "GLACIER" => TransitionStorageClass::Glacier,
        "DEEP_ARCHIVE" => TransitionStorageClass::DeepArchive,
        "GLACIER_IR" => TransitionStorageClass::GlacierIr,
        _ => {
            println!("Invalid storage class. Use: GLACIER, DEEP_ARCHIVE, or GLACIER_IR");
            return Ok(());
        }
    };

    // List objects with prefix
    let mut continuation_token = None;
    let mut total_objects = 0;

    loop {
        let mut list_req = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .max_keys(1000);

        if let Some(token) = continuation_token {
            list_req = list_req.continuation_token(token);
        }

        let list_output = list_req
            .send()
            .await
            .context("Failed to list objects")?;

        if let Some(contents) = list_output.contents {
            for object in contents {
                if let Some(key) = object.key {
                    // Copy object to same location with new storage class
                    client
                        .copy_object()
                        .bucket(bucket)
                        .key(&key)
                        .copy_source(format!("{}/{}", bucket, key))
                        .storage_class(storage_class_enum.clone())
                        .metadata_directive(aws_sdk_s3::types::MetadataDirective::Copy)
                        .send()
                        .await
                        .context(format!("Failed to archive object: {}", key))?;

                    total_objects += 1;
                    println!("  ✓ Archived: {}", key);
                }
            }
        }

        if list_output.is_truncated == Some(true) {
            continuation_token = list_output.next_continuation_token;
        } else {
            break;
        }
    }

    println!("\n✓ Archived {} objects to {}", total_objects, storage_class);
    Ok(())
}

fn print_rule(rule: &LifecycleRule) {
    println!("\nRule ID: {}", rule.id().unwrap_or("N/A"));
    println!("Status: {:?}", rule.status());
    
    if let Some(filter) = &rule.filter {
        match filter {
            LifecycleRuleFilter::Prefix(p) => {
                if !p.is_empty() {
                    println!("Prefix: {}", p);
                }
            }
            _ => println!("Filter: {:?}", filter),
        }
    }

    if let Some(transitions) = &rule.transitions {
        println!("\nTransitions:");
        for t in transitions {
            if let Some(days) = t.days {
                println!("  - After {} days → {:?}", days, t.storage_class);
            }
        }
    }

    if let Some(expiration) = &rule.expiration {
        if let Some(days) = expiration.days {
            println!("\nExpiration: {} days", days);
        }
    }

    println!("{:-<80}", "");
}