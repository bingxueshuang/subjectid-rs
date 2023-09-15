---
title: Subject ID as Enum
description: Design decision of choosing enum over struct for various subject id formats
---
# Enum and Structs

Subject Identifiers are one of the few formats as defined in the registry.
For the design documentation below, consider subject identifiers have only 2
defined formats - "account" and "email".

Account identifier format specifies `acct` URI and email identifier format
specifies the email address.

**Implementation:**

- Named Fields on Enum Variants

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

## Named Fields on Enum Variants

The data associated with identifier formats is simple:

- No necessity for builder or constructor
- Identifier format cannot have behavior that Subject Identifier itself
  does not have. So, a format need not have its own associated methods.
- Information associated with an identifier format is an integral part of
  the subject identifier itself.

Due to the above reasons, it makes sense to define the information associated
with an identifier format in the enum variant itself as named field.

```rust
enum Id {
    Account { uri: String },
    Email { email: String },
}
```
