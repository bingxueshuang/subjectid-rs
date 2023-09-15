//! A Subject Identifier is a JSON [RFC8259](https://www.rfc-editor.org/info/rfc8259) object whose
//! contents may be used to identify a subject within some context. An Identifier Format is a named
//! definition of a set of information that may be used to identify a subject, and the rules for
//! encoding that information as a Subject Identifier; they define the syntax and semantics of
//! Subject Identifiers. A Subject Identifier MUST conform to a specific Identifier Format,
//! and MUST contain a "format" member whose value is the name of that Identifier Format.
//!
//! Every Identifier Format MUST have a unique name registered in the IANA "Security Event
//! Identifier Formats" registry, or a Collision-Resistant Name as defined in
//! [RFC7519](https://www.rfc-editor.org/info/rfc7519). Identifier Formats that are expected to
//! be used broadly by a variety of parties SHOULD be registered in the "Security Event Identifier
//! Formats" registry.
//!
//! Identifier Format MAY describe more members than are strictly necessary to identify a subject,
//! and MAY describe conditions under which those members are required, optional, or prohibited.
//! The "format" member is reserved for use as described in this specification; Identifier Formats
//! MUST NOT declare any rules regarding the "format" member. Every member within a Subject
//! Identifier MUST match the rules specified for that member by this specification or by Subject
//! Identifier's Identifier Format. A Subject Identifier MUST NOT contain any members prohibited
//! or not described by its Identifier Format, and MUST contain all members required by its
//! Identifier Format.

/// An account is an arrangement or agreement through which a user gets access to a service and gets
/// a unique identity with the service provider. Subject Identifiers in this format MUST contain a
/// "uri" member whose value is the "acct" URI for the subject. The "uri" member is REQUIRED and
/// MUST NOT be null or empty. The Account Identifier Format is identified by a value of "account".
///
/// ```
/// use subjectid::{AccountId, Id};
/// # fn main() {
/// let subid = Id::Account(AccountId{
///     uri: "acct:example.user@service.example.com".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct AccountId {
    pub uri: String,
}

/// The Email Identifier Format identifies a subject using an email address. Subject Identifiers in
/// this format MUST contain an "email" member whose value is a string containing the email address
/// of the subject, formatted as an "addr-spec" as defined in Section 3.4.1 of
/// [RFC5322](https://www.rfc-editor.org/info/rfc5322). The "email" member is REQUIRED and MUST NOT
/// be null or empty. The value of the "email" member MUST identify a mailbox to which email may be
/// delivered, in accordance with [RFC5321](https://www.rfc-editor.org/info/rfc5321).
/// The Email Identifier Format is identified by the name "email".
///
/// Email canonicalization is not standardized, and there is no way for the event recipient to
/// determine the mail provider’s canonicalization method. Therefore, the recipient SHOULD apply
/// its own canonicalization algorithm to incoming events that reproduces the translation done by
/// the local email system.
///
/// ```
/// use subjectid::{EmailId, Id};
/// # fn main () {
/// let subid = Id::Email(EmailId{
///     email: "user@example.com".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct EmailId {
    pub email: String,
}

/// The Issuer and Subject Identifier Format identifies a subject using a pair of "iss" and "sub"
/// members, analogous to how subjects are identified using the "iss" and "sub" claims in OpenID
/// Connect [OpenID.Core](https://openid.net/specs/openid-connect-core-1_0.html) ID Tokens.
/// These members MUST follow the formats of the "iss" member and "sub" member defined by
/// [RFC7519](https://www.rfc-editor.org/info/rfc7519), respectively. Both the "iss" member and
/// the "sub" member are REQUIRED and MUST NOT be null or empty. The Issuer and Subject Identifier
/// Format is identified by the name "iss_sub".
///
/// ```
/// use subjectid::{Id, IssSubId};
/// # fn main() {
/// let subid = Id::IssuerSubject(IssSubId{
///     issuer: "https://issuer.example.com/".to_owned(),
///     subject: "145234573".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct IssSubId {
    pub issuer: String,
    pub subject: String,
}

/// The Opaque Identifier Format describes a subject that is identified with a string with no
/// semantics asserted beyond its usage as an identifier for the subject, such as a UUID or hash
/// used as a surrogate identifier for a record in a database. Subject Identifiers in this format
/// MUST contain an "id" member whose value is a JSON string containing the opaque string identifier
/// for the subject. The "id" member is REQUIRED and MUST NOT be null or empty. The Opaque
/// Identifier Format is identified by the name "opaque".
///
/// ```
/// use subjectid::{Id, OpaqueId};
/// # fn main() {
/// let subid = Id::Opaque(OpaqueId{
///     id: "11112222333344445555".to_owned(),
/// });
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OpaqueId {
    pub id: String,
}

/// The Phone Number Identifier Format identifies a subject using a telephone number.
/// Subject Identifiers in this format MUST contain a "phone_number" member whose value is a string
/// containing the full telephone number of the subject, including international dialing prefix,
/// formatted according to E.164 [E164](https://www.itu.int/rec/T-REC-E.164-201011-I/en).
/// The "phone_number" member is REQUIRED and MUST NOT be null or empty. The Phone Number Identifier
/// Format is identified by the name "phone_number".
///
/// ```
/// use subjectid::{Id, PhoneId};
/// # fn main() {
/// let subid = Id::PhoneNumber(PhoneId{
///     phone_number: "+12065550100".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PhoneId {
    pub phone_number: String,
}

/// The Decentralized Identifier Format identifies a subject using a Decentralized Identifier (DID)
/// URL as defined in [DID](https://www.w3.org/TR/did-core/). Subject Identifiers in this format
/// MUST contain a "URL" member whose value is a DID URL for the DID Subject being identified.
/// The value of the "url" member MUST be a valid DID URL and MAY be a bare DID. The "url" member
/// is REQUIRED and MUST NOT be null or empty. The Decentralized Identifier Format is identified
/// by the name "did".
///
/// ```
/// use subjectid::{DidId, Id};
/// # fn main() {
/// let subid = Id::DID(DidId{
///     url: "did:example:123456".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct DidId {
    pub url: String,
}

/// The Uniform Resource Identifier (URI) Format identifies a subject using a URI as defined in
/// [RFC3986](https://www.rfc-editor.org/info/rfc3986). This identifier format makes no assumptions
/// or guarantees with regard to the content, scheme, or reachability of the URI within the field.
/// Subject Identifiers in this format MUST contain a "uri" member whose value is a URI for the
/// subject being identified. The "uri" member is REQUIRED and MUST NOT be null or empty. The URI
/// format is identified by the name "uri".
///
/// ```
/// use subjectid::{Id, UriId};
/// # fn main() {
/// let subid = Id::URI(UriId{
///     uri: "urn:uuid:4e851e98-83c4-4743-a5da-150ecb53042f".to_owned(),
/// });
/// println!("{:?}", subid);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct UriId {
    pub uri: String,
}

/// [Id] defines the Subject Identifiers in the different formats.
#[derive(Debug, Clone)]
pub enum Id {
    /// The Account Identifier Format identifies a subject using an account at a service provider,
    /// identified with an "acct" URI as defined in [RFC7565](https://www.rfc-editor.org/info/rfc7565).
    Account(AccountId),
    /// The Email Identifier Format identifies a subject using an email address.
    Email(EmailId),
    /// The Issuer and Subject Identifier format identifies the subject using a pair of "iss", "sub"
    /// members analogous to OpenID Connect ID Tokens.
    IssuerSubject(IssSubId),
    /// The Opaque Identifier Format identifies the subject using a string with no semantics
    /// asserted beyond its usage as an identifier for the subject.
    Opaque(OpaqueId),
    /// The Phone Number Identifier Format identifies a subject using a telephone number.
    PhoneNumber(PhoneId),
    /// The DID Identifier Format identifies a subject using a Decentralized Identifier (DID) URL.
    DID(DidId),
    /// The URI Identifier Format identifies a subject using a URI (Uniform Resource Identifier).
    URI(UriId),
}

impl Id {
    /// The name of the Identifier Format as defined in Security Event Identifier Formats registry.
    pub fn format(&self) -> &'static str {
        match self {
            Id::Account(_) => "account",
            Id::Email(_) => "email",
            Id::IssuerSubject(_) => "iss_sub",
            Id::Opaque(_) => "opaque",
            Id::PhoneNumber(_) => "phone_number",
            Id::DID(_) => "did",
            Id::URI(_) => "uri",
        }
    }
}
