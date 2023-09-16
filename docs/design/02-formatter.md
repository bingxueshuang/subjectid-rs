---
title: Subject ID Format
description: Design decision for providing API to obtain the format name corresponding to a subject identifier.
editUrl: https://github.com/bingxueshuang/subjectid-rs/edit/main/docs/design/02-formatter.md
---

# Formatter

Since we can have subject identifiers of different specified types, we need to
have a way to display the format name of any given identifier.

**Implementation:**

- `format` method > Associated Constants
- Format Constants

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

### Global Constants

Use the globally defined constants as format name return value. This is more
developer friendly than using string literals (say, for checking equality).

```rust
impl Id {
    fn format(&self) -> &'static str {
        match self {
            Id::Account { .. } => ACCOUNT,
            Id::Email { .. } => EMAIL,
        }
    }
}
```

### Associated constants

Each of the constants can be made associated with the subject identifier, thus
no cluttering global namespace (module-level).

```rust
impl Id {
    const ACCOUNT: &'static str = "account";
    const EMAIL: &'static str = "email";
    fn format(&self) -> &'static str {
        match self {
            Id::Account { .. } => Id::ACCOUNT,
            Id::Email { .. } => Id::EMAIL,
        }
    }
}
```

## Format Constants

For every format name defined in the registry, declare a constant corresponding
to it. The `format` method implementation returns the constants instead of
literal strings.

```rust
const ACCOUNT: &'static str = "account";
const EMAIL: &'static str = "email";
```
