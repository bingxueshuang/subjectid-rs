//! A Subject Identifier is a JSON [`RFC8259`] object whose contents may be used to identify a
//! subject within some context. An Identifier Format is a named definition of a set of information
//! that may be used to identify a subject, and the rules for encoding that information as a
//! Subject Identifier; they define the syntax and semantics of Subject Identifiers. A Subject
//! Identifier MUST conform to a specific Identifier Format, and MUST contain a "format" member
//! whose value is the name of that Identifier Format.
//!
//! [`RFC8259`]: https://www.rfc-editor.org/info/rfc8259
//!
//! Every Identifier Format MUST have a unique name registered in the IANA "Security Event
//! Identifier Formats" registry, or a Collision-Resistant Name as defined in [`RFC7519`].
//! Identifier Formats that are expected to be used broadly by a variety of parties SHOULD be
//! registered in the "Security Event Identifier Formats" registry.
//!
//! [`RFC7519`]: https://www.rfc-editor.org/info/rfc7519
//!
//! Identifier Format MAY describe more members than are strictly necessary to identify a subject,
//! and MAY describe conditions under which those members are required, optional, or prohibited.
//! The "format" member is reserved for use as described in this specification; Identifier Formats
//! MUST NOT declare any rules regarding the "format" member. Every member within a Subject
//! Identifier MUST match the rules specified for that member by this specification or by Subject
//! Identifier's Identifier Format. A Subject Identifier MUST NOT contain any members prohibited
//! or not described by its Identifier Format, and MUST contain all members required by its
//! Identifier Format.
//!
//! See: [`SubjectID`]
//!
//! [`SubjectID`]: https://datatracker.ietf.org/doc/html/draft-ietf-secevent-subject-identifiers

use ::serde::{Deserialize, Serialize};
mod single;

pub use single::Atomic;

/// SubjectID is the core type of the crate that defines subject identifier for Security Event Token
/// (SET). Either a subject identifier has to be [Atomic] or [Aliases].
///
/// ```
/// use subjectid::{Atomic, SubjectId};
/// let subid = SubjectId::Atomic(
///     Atomic::Opaque { id: "1i3j4l".to_owned() },
/// );
/// println!("{:?}", subid);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubjectId {
    Atomic(Atomic),
    Aliases(Aliases),
}

/// The Aliases Identifier Format describes a subject that is identified with a list of different
/// Subject Identifiers. It is intended for use when a variety of identifiers have been shared with
/// the party that will be interpreting the Subject Identifier, and it is unknown which of those
/// identifiers they will recognize or support. This format is identified by the name "aliases".
/// "aliases" Subject Identifiers MUST NOT be nested; i.e., the "identifiers" member of an "aliases"
/// Subject Identifier MUST NOT contain a Subject Identifier in the "aliases" format.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "format")]
#[serde(rename = "aliases")]
pub struct Aliases {
    /// Member "identifiers" denotes JSON array containing one or more Subject Identifiers.
    /// Each Subject Identifier in the array MUST identify the same entity. "identifiers" member is
    /// REQUIRED and MUST NOT be null or empty. It MAY contain multiple instances of the same
    /// Identifier Format (e.g., multiple Email Subject Identifiers),
    /// but SHOULD NOT contain exact duplicates.
    pub identifiers: Vec<Atomic>,
}

/// Associate the available format options with the [SubjectId].  Every format below corresponds
/// to one of the subject identifier formats defined in the registry.
#[allow(unused)]
impl SubjectId {
    pub const FORMAT_ACCOUNT: &'static str = "account";
    pub const FORMAT_EMAIL: &'static str = "email";
    pub const FORMAT_ISSUER_SUBJECT: &'static str = "iss_sub";
    pub const FORMAT_OPAQUE: &'static str = "opaque";
    pub const FORMAT_PHONE_NUMBER: &'static str = "phone_number";
    pub const FORMAT_DID: &'static str = "did";
    pub const FORMAT_URI: &'static str = "uri";
    pub const FORMAT_ALIASES: &'static str = "aliases";
}

impl SubjectId {
    /// Given a [SubjectId], [format] reports the subject identifier format that defines it. Return
    /// value should be one of the constants of the form `FORMAT_*` associated with [SubjectId].
    pub fn format(&self) -> &'static str {
        match self {
            Self::Atomic(id) => id.format(),
            Self::Aliases(..) => Self::FORMAT_ALIASES,
        }
    }
}
