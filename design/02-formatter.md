# Formatter

Since we can have subject identifiers of different specified types, we need to
have a way to display the format name of any given identifier.

**Implementation:**
- `format` method > Static String

## `format` method

Define `format` method on identifier that says which format it is.

### Static String

All the format names defined in the registry are simple strings and enum
variants are a sufficient way of knowing the format programmatically (for 
compiler). For human friendly representation of the format names, string
literals can be used.

```rust
impl Id {
    fn format(&self) -> &'static str {
        match self {
            Id::Account(_) => "account",
            Id::Email(_) => "email",
        }
    }
}
```
