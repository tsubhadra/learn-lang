# Architecture Diagram: 2018 Rust S3 Lifecycle CLI Tool

## Overview
This document provides architectural diagrams for the AWS S3 Lifecycle and Archival Management CLI tool built with 2018-era Rust patterns.

---

## High-Level Architecture

```mermaid
graph TB
    User[User/CLI] --> Main[main fn]
    Main --> CLI[Clap CLI Parser]
    CLI --> Run[run fn]
    Run --> EventLoop[Tokio Core Event Loop]
    EventLoop --> S3Client[Rusoto S3 Client]
    S3Client --> AWS[AWS S3 API]
    
    Run --> List[list_lifecycle_rules]
    Run --> Create[create_lifecycle_rule]
    Run --> Delete[delete_lifecycle_rule]
    Run --> Show[show_lifecycle_rule]
    Run --> Archive[archive_objects]
    
    List --> S3Client
    Create --> S3Client
    Delete --> S3Client
    Show --> S3Client
    Archive --> S3Client
    
    List --> Print[print_rule]
    Show --> Print
    
    style Main fill:#e1f5ff
    style Run fill:#fff3e0
    style EventLoop fill:#f3e5f5
    style S3Client fill:#e8f5e9
    style AWS fill:#ffebee
```

---

## Command Flow Architecture

```mermaid
flowchart TD
    Start([Program Start]) --> Parse[Parse CLI Arguments<br/>with Clap]
    Parse --> Match{Match Subcommand}
    
    Match -->|list| ListCmd[List Command]
    Match -->|create| CreateCmd[Create Command]
    Match -->|delete| DeleteCmd[Delete Command]
    Match -->|show| ShowCmd[Show Command]
    Match -->|archive| ArchiveCmd[Archive Command]
    Match -->|none| Help[Show Help]
    
    ListCmd --> GetRules[Get Lifecycle Rules<br/>from S3]
    GetRules --> DisplayRules[Display Rules<br/>to Console]
    
    CreateCmd --> GetExisting[Get Existing Rules]
    GetExisting --> BuildRule[Build New Rule<br/>with Transitions]
    BuildRule --> PutRules[Put Updated Rules<br/>to S3]
    
    DeleteCmd --> GetForDelete[Get Current Rules]
    GetForDelete --> FilterRule[Filter Out<br/>Target Rule]
    FilterRule --> AllGone{All Rules<br/>Deleted?}
    AllGone -->|Yes| DeleteConfig[Delete Entire<br/>Configuration]
    AllGone -->|No| UpdateConfig[Update Configuration]
    
    ShowCmd --> GetForShow[Get All Rules]
    GetForShow --> FindRule[Find Specific Rule]
    FindRule --> DisplayRule[Display Rule Details]
    
    ArchiveCmd --> ValidateClass{Valid Storage<br/>Class?}
    ValidateClass -->|No| Error[Return Error]
    ValidateClass -->|Yes| ListObjects[List Objects<br/>with Prefix]
    ListObjects --> ProcessObjects[Process Each Object]
    ProcessObjects --> CopyObject[Copy Object to Self<br/>with New Storage Class]
    CopyObject --> MoreObjects{More Objects?}
    MoreObjects -->|Yes| ListObjects
    MoreObjects -->|No| Complete[Complete]
    
    DisplayRules --> End([Program End])
    PutRules --> End
    DeleteConfig --> End
    UpdateConfig --> End
    DisplayRule --> End
    Complete --> End
    Help --> End
    Error --> End
    
    style Start fill:#4caf50,color:#fff
    style End fill:#f44336,color:#fff
    style Match fill:#ff9800,color:#fff
    style AllGone fill:#2196f3,color:#fff
    style ValidateClass fill:#2196f3,color:#fff
    style MoreObjects fill:#2196f3,color:#fff
```

---

## Data Flow: Create Lifecycle Rule

```mermaid
sequenceDiagram
    participant User
    participant CLI as Clap Parser
    participant Main as main/run
    participant Core as Tokio Core
    participant Client as S3 Client
    participant AWS as AWS S3

    User->>CLI: cargo run -- create --bucket my-bucket --id rule1 --glacier-days 90
    CLI->>Main: Parse arguments
    Main->>Main: Extract parameters<br/>(bucket, id, glacier_days, etc.)
    Main->>Core: Create event loop
    Main->>Client: Initialize S3Client
    
    Main->>Client: get_bucket_lifecycle_configuration()
    Client->>Core: Execute async operation
    Core->>AWS: GET bucket lifecycle
    AWS-->>Core: Return existing rules
    Core-->>Main: Existing rules or empty vec
    
    Main->>Main: Build new rule structure<br/>(transitions, filter, expiration)
    Main->>Main: Merge with existing rules
    
    Main->>Client: put_bucket_lifecycle_configuration()
    Client->>Core: Execute async operation
    Core->>AWS: PUT bucket lifecycle
    AWS-->>Core: Success response
    Core-->>Main: Result
    
    Main->>User: Print success message
```

---

## Data Flow: Archive Objects

```mermaid
sequenceDiagram
    participant User
    participant CLI as Clap Parser
    participant Main as main/run
    participant Core as Tokio Core
    participant Client as S3 Client
    participant AWS as AWS S3

    User->>CLI: cargo run -- archive --bucket my-bucket --prefix logs/ --storage-class GLACIER
    CLI->>Main: Parse arguments
    Main->>Main: Validate storage class
    Main->>Core: Create event loop
    Main->>Client: Initialize S3Client
    
    loop For each page of objects
        Main->>Client: list_objects_v2(prefix, continuation_token)
        Client->>Core: Execute async operation
        Core->>AWS: LIST objects
        AWS-->>Core: Return object list
        Core-->>Main: Object list + continuation token
        
        loop For each object
            Main->>Client: copy_object(same key, new storage class)
            Client->>Core: Execute async operation
            Core->>AWS: COPY object
            AWS-->>Core: Success
            Core-->>Main: Result
            Main->>User: Print archived message
        end
        
        Main->>Main: Check is_truncated
    end
    
    Main->>User: Print total count
```

---

## Technology Stack

```mermaid
graph LR
    subgraph "Core Dependencies"
        Rust[Rust 2015 Edition]
        Tokio[tokio-core 0.1]
        Futures[futures 0.1]
    end
    
    subgraph "AWS SDK"
        Rusoto[rusoto 0.42]
        RCore[rusoto_core]
        RS3[rusoto_s3]
    end
    
    subgraph "CLI & Utilities"
        Clap[clap 2.32]
        Serde[serde 1.0]
        SerdeJSON[serde_json 1.0]
    end
    
    Rust --> Tokio
    Rust --> Futures
    Rust --> Clap
    Rust --> Serde
    
    Tokio --> Rusoto
    Futures --> Rusoto
    
    Rusoto --> RCore
    Rusoto --> RS3
    
    style Rust fill:#dea584
    style Tokio fill:#86cefa
    style Rusoto fill:#f9e79f
    style Clap fill:#abebc6
```

---

## Component Interaction Diagram

```mermaid
graph TB
    subgraph "User Interface Layer"
        CLI[CLI Arguments<br/>Clap Parser]
    end
    
    subgraph "Application Logic Layer"
        Main[main fn]
        Run[run fn]
        List[list_lifecycle_rules]
        Create[create_lifecycle_rule]
        Delete[delete_lifecycle_rule]
        Show[show_lifecycle_rule]
        Archive[archive_objects]
        Print[print_rule]
    end
    
    subgraph "2018 Async Runtime Layer"
        EventLoop[Tokio Core<br/>Event Loop]
        Futures[Futures 0.1<br/>Combinators]
    end
    
    subgraph "AWS SDK Layer"
        S3Client[Rusoto S3Client]
        S3Trait[S3 Trait]
        Requests[Request Types<br/>GetBucketLifecycle<br/>PutBucketLifecycle<br/>etc.]
    end
    
    subgraph "External Services"
        AWS[AWS S3 API<br/>HTTPS/REST]
    end
    
    CLI --> Main
    Main --> Run
    Run --> List
    Run --> Create
    Run --> Delete
    Run --> Show
    Run --> Archive
    
    List --> Print
    Show --> Print
    
    List --> EventLoop
    Create --> EventLoop
    Delete --> EventLoop
    Show --> EventLoop
    Archive --> EventLoop
    
    EventLoop --> S3Client
    S3Client --> S3Trait
    S3Trait --> Requests
    
    Requests --> AWS
    
    style CLI fill:#e3f2fd
    style Main fill:#fff9c4
    style EventLoop fill:#f3e5f5
    style S3Client fill:#e8f5e9
    style AWS fill:#ffebee
```

---

## Lifecycle Rule Structure

```mermaid
classDiagram
    class LifecycleRule {
        +Option~String~ id
        +String status
        +Option~LifecycleRuleFilter~ filter
        +Option~Vec~Transition~~ transitions
        +Option~LifecycleExpiration~ expiration
    }
    
    class LifecycleRuleFilter {
        +Option~String~ prefix
        +Option~Tag~ tag
        +Option~LifecycleRuleAndOperator~ and
    }
    
    class Transition {
        +Option~i64~ days
        +Option~String~ date
        +Option~String~ storage_class
    }
    
    class LifecycleExpiration {
        +Option~i64~ days
        +Option~String~ date
        +Option~bool~ expired_object_delete_marker
    }
    
    class BucketLifecycleConfiguration {
        +Vec~LifecycleRule~ rules
    }
    
    BucketLifecycleConfiguration "1" --> "*" LifecycleRule
    LifecycleRule "1" --> "1" LifecycleRuleFilter
    LifecycleRule "1" --> "*" Transition
    LifecycleRule "1" --> "0..1" LifecycleExpiration
```

---

## Async Execution Model (2018 Era)

```mermaid
sequenceDiagram
    participant App as Application Code
    participant Core as Tokio Core
    participant Handle as Reactor Handle
    participant Future as Future 0.1
    participant IO as I/O Operations

    App->>Core: Core::new()
    Core->>Handle: Create reactor handle
    
    App->>Future: Create future chain<br/>(client.operation())
    App->>Core: core.run(future)
    
    Core->>Future: Poll future
    Future->>IO: Start I/O operation
    IO-->>Future: Not Ready
    Future-->>Core: Not Ready
    
    Core->>Core: Block and wait
    
    IO->>Future: I/O complete
    Future->>Future: Process result
    Future-->>Core: Ready(result)
    
    Core-->>App: Return result
    
    Note over App,IO: This is the 2018 pattern<br/>Modern Rust uses async/await
```

---

## Error Handling Flow

```mermaid
graph TD
    Start[Function Call] --> Try{Try Operation}
    
    Try -->|Success| Transform[Transform Result]
    Try -->|Error| MapErr[.map_err to String]
    
    Transform --> Question{Use ? operator}
    Question -->|Ok| Continue[Continue Execution]
    Question -->|Err| Propagate[Propagate Error Up]
    
    MapErr --> Question
    
    Propagate --> Caller[Caller Function]
    Caller --> CallerHandle{Caller Handles?}
    
    CallerHandle -->|Yes| Handle[Handle Error]
    CallerHandle -->|No| PropagateUp[Propagate Further]
    
    PropagateUp --> Main[Reaches main/run]
    Main --> Print[Print Error]
    Print --> Exit[process::exit 1]
    
    Handle --> Recover[Recover or Display]
    Continue --> Success[Return Ok]
    
    style Try fill:#fff3e0
    style MapErr fill:#ffebee
    style Question fill:#e1f5fe
    style CallerHandle fill:#e1f5fe
```

---

## S3 Storage Classes Flow

```mermaid
graph LR
    Standard[STANDARD<br/>Frequent Access] -->|After N days| IA[STANDARD_IA<br/>Infrequent Access]
    IA -->|After N days| Glacier[GLACIER<br/>Archive]
    Glacier -->|After N days| DeepArchive[DEEP_ARCHIVE<br/>Long-term Archive]
    DeepArchive -->|After N days| Expire[EXPIRATION<br/>Deleted]
    
    Standard -.->|Direct Archive| Glacier
    Standard -.->|Direct Archive| DeepArchive
    
    style Standard fill:#4caf50,color:#fff
    style IA fill:#8bc34a
    style Glacier fill:#03a9f4,color:#fff
    style DeepArchive fill:#2196f3,color:#fff
    style Expire fill:#f44336,color:#fff
```

---

## Project File Structure

```
2018-july-rust-s3lifecycle-project/
│
├── Cargo.toml              # Dependencies (rusoto 0.42, tokio-core 0.1)
├── Cargo.lock              # Locked dependency versions
├── usage.txt               # Usage examples
│
└── src/
    ├── main.rs             # Main application code (538 lines)
    ├── explain.txt         # Line-by-line explanation
    └── architecture.md     # This file - architectural diagrams
```

---

## Key Architectural Decisions

### 1. **2018 Async Pattern**
- Uses `tokio-core::reactor::Core` instead of `#[tokio::main]`
- Explicit event loop creation and management
- `core.run(future)` blocks until completion
- No `.await` syntax (pre-Rust 1.39)

### 2. **Rust 2015 Edition**
- Requires `extern crate` declarations
- Compatible with 2018-era toolchains
- Maintains historical accuracy

### 3. **Error Handling Strategy**
- All functions return `Result<(), String>`
- `.map_err()` for error conversion
- `?` operator for error propagation
- User-friendly error messages

### 4. **CLI Design Pattern**
- Builder pattern with Clap 2.x
- Subcommand-based interface
- Fluent API for argument definition
- Pre-derive macro style

### 5. **S3 Archival Technique**
- Copy object to itself with new storage class
- Standard AWS pattern for storage class changes
- Pagination for large object lists
- Continuation tokens for next pages

### 6. **Rusoto 0.42 API**
- `BucketLifecycleConfiguration` (not `LifecycleConfiguration`)
- Request/Response pattern
- Async operations via futures 0.1
- AWS credential chain support

---

## Execution Flow Summary

1. **CLI Parsing**: Clap parses command-line arguments
2. **Event Loop Creation**: Tokio Core creates async runtime
3. **S3 Client Initialization**: Rusoto client with AWS credentials
4. **Command Dispatch**: Match on subcommand and route to handler
5. **Async Execution**: `core.run()` executes AWS operations
6. **Result Display**: Format and print results to console
7. **Error Handling**: Convert and display errors, exit with code

---

## Integration Points

### AWS S3 API
- **Authentication**: AWS credentials from environment, file, or IAM role
- **Region**: Configurable via AWS_REGION or default
- **Operations**: GET/PUT/DELETE lifecycle configurations, LIST/COPY objects
- **Error Handling**: AWS SDK errors converted to user-friendly strings

### Terminal/Shell
- **Input**: Command-line arguments parsed by Clap
- **Output**: Formatted text to stdout
- **Errors**: Error messages to stderr
- **Exit Codes**: 0 for success, 1 for error

---

## Concurrency Model

```mermaid
graph LR
    SingleThread[Single Thread<br/>Execution] --> EventLoop[Tokio Core<br/>Event Loop]
    EventLoop --> IO[I/O Multiplexing<br/>epoll/kqueue]
    IO --> NonBlocking[Non-blocking<br/>I/O Operations]
    NonBlocking --> Async[Async Futures<br/>Polled to Completion]
    
    style SingleThread fill:#e1f5fe
    style EventLoop fill:#f3e5f5
    style IO fill:#fff3e0
    style Async fill:#e8f5e9
```

**Note**: This is a single-threaded async model. All operations run on one thread using an event loop, unlike modern Tokio which uses work-stealing thread pools.

---

## Version History Context

| Component | 2018 Version | Modern Equivalent |
|-----------|--------------|-------------------|
| Rust Edition | 2015 | 2021 |
| Tokio | tokio-core 0.1 | tokio 1.x with #[tokio::main] |
| Futures | futures 0.1 | futures 0.3 with async/await |
| Clap | 2.32 (builder) | 4.x with derive macros |
| Rusoto | 0.42 | AWS SDK for Rust |
| Async Syntax | .and_then(), .map_err() | async/await |

---

## Conclusion

This architecture represents a faithful reproduction of 2018-era Rust async programming patterns. The design emphasizes:
- Explicit event loop management
- Manual future composition
- Builder patterns for CLIs
- Pre-async/await ergonomics

While more verbose than modern Rust, this pattern was the foundation that led to today's async/await syntax and demonstrates the evolution of Rust's async ecosystem.

