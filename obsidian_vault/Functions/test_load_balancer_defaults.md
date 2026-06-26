---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_load_balancer_defaults

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L411)

## Signature
```rust
fn test_load_balancer_defaults()
```

## Implementation
```rust
fn test_load_balancer_defaults() {
        let cfg = AppConfig::default();
        assert_eq!(
            cfg.network.load_balancer.strategy,
            LoadBalancerStrategy::RoundRobin
        );
    }
```

## Calls & References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

