---
title: Aliases Format
description: Design decision for exposing aliases format through the API.
editUrl: https://github.com/bingxueshuang/subjectid-rs/edit/main/docs/design/03-aliases.md
---

# Aliases Subject Identifier

Aliases format allows to specify subject identifiers in several formats all
of which identify the same subject. In cases of not knowing what format the
receiver expects subject identifiers, sending subject details in multiple
formats allows the receiver to choose the suitable one out of it.

## No Nesting

As defined in the document, aliases identifier cannot be nested. So, we
cannot just allow all subject identifiers inside aliases. Denying aliases
identifiers inside itself via code is complicated.

```rust
#[derive(Clone, Debug)]
enum Id {
    Account {
        uri: String,
    },
    Aliases {
        identifiers: Vec<Id>,
    },
}

fn main() {
    let nested = Id::Aliases {
        identifiers: vec![
            Id::Account { uri: "acct:someone@example.com".to_owned() },
            Id::Aliases {
                identifiers: vec![],
            },
        ],
    };
    println!("{:?}", nested);
}
```

## Core Subject ID enum

Define an Enum for normal subject identifier, and Aliases identifier. Now,
aliases can only contain normal subject identifiers. Thus, this API would
be more ergonomic.

```rust
enum Id {
    Opaque {
        id: String,
    },
}

enum SubjectID {
    Identifier(Id),
    Aliases {
        identifiers: Vec<Id>,
    },
}

fn main() {
    let subid = SubjectId::Aliases {
        identifiers: vec![
            Id::Opaque { id: "2k3j4hjk".to_owned() },
        ],
    };
    let single = SubjectID::Identifier(
        Id::Opaque { id: "23kj234".to_owned() },
    );
    println!("{:?}", subid);
    println!("{:?}", single);
}
```
