# Enum and Structs

Subject Identifiers are one of the few formats as defined in the registry.
For the design documentation below, consider subject identifiers have only 2
defined formats - "account" and "email".

Account identifier format specifies `acct` URI and email identifier format
specifies the email address.

**Implementation:**

- Struct-Associated Enum

## Enum

Since Subject Identifier can have one of several variant of formats, it is
more idiomatic to represent it as an enum.

```rust
enum Id {
    Account,
    Email,
}
```

## Struct-associated Enum

But each format has certain fields, so define structs separately for every format
and associate it with the enum variant:

```rust
struct AccountId {
    uri: String,
}

struct EmailId {
    email: String,
}

enum Id {
    Account(AccountId),
    Email(EmailId),
}
```
