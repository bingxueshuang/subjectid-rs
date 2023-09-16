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

/// [Id] defines the Subject Identifiers in the different formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
#[serde(rename_all = "snake_case")]
pub enum Id {
    /// The Account Identifier Format identifies a subject using an account at a service provider,
    /// identified with an "acct" URI as defined in [`RFC7565`]. An account is an arrangement or
    /// agreement through which a user gets access to a service and gets a unique identity with
    /// the service provider. The Account Identifier Format is identified by a value of "account".
    ///
    /// [`RFC7565`]: https://www.rfc-editor.org/info/rfc7565
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::Account{
    ///     uri: "acct:example.user@service.example.com".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    Account {
        /// The "acct" URI for the subject. The "uri" member is REQUIRED and MUST NOT be null or
        /// empty.
        uri: String,
    },
    /// The Email Identifier Format identifies a subject using an email address. The value of the
    /// "email" member MUST identify a mailbox to which email may be delivered, in accordance with
    /// [`RFC5321`]. The Email Identifier Format is identified by the name "email".
    ///
    /// [`RFC5321`]: https://www.rfc-editor.org/info/rfc5321
    ///
    /// Email canonicalization is not standardized, and there is no way for the event recipient to
    /// determine the mail provider’s canonicalization method. Therefore, the recipient SHOULD apply
    /// its own canonicalization algorithm to incoming events that reproduces the translation done by
    /// the local email system.
    ///
    /// ```
    /// # fn main () {
    /// use subjectid::Id;
    /// let subid = Id::Email{
    ///     email: "user@example.com".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    Email {
        /// A string containing the email address of the subject, formatted as an "addr-spec" as
        /// defined in Section 3.4.1 of [`RFC5322`]. The "email" member is REQUIRED and MUST NOT be
        /// null or empty. The value of the "email" member MUST identify a mailbox to which email
        /// may be delivered, in accordance with [`RFC5321`].
        ///
        /// [`RFC5321`]: https://www.rfc-editor.org/info/rfc5321
        /// [`RFC5322`]: https://www.rfc-editor.org/info/rfc5322
        email: String,
    },
    /// The Issuer and Subject Identifier Format identifies a subject using a pair of "iss" and
    /// "sub" members, analogous to how subjects are identified using the "iss" and "sub" claims
    /// in OpenID Connect [[`OpenID.Core`]] ID Tokens. These members MUST follow the formats of the
    /// "iss" member and "sub" member defined by [`RFC7519`], respectively. Both the "iss" member
    /// and the "sub" member are REQUIRED and MUST NOT be null or empty. The Issuer and Subject
    /// Identifier Format is identified by the name "iss_sub".
    ///
    /// [`OpenID.Core`]: https://openid.net/specs/openid-connect-core-1_0.html
    /// [`RFC7519`]: https://www.rfc-editor.org/info/rfc7519
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::IssuerSubject{
    ///     issuer: "https://issuer.example.com/".to_owned(),
    ///     subject: "145234573".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    #[serde(rename = "iss_sub")]
    IssuerSubject {
        /// The "iss" (issuer) member identifies the principal that issued the JWT. The processing
        /// of this claim is generally application specific. The "iss" value is a case-sensitive
        /// string containing a `StringOrURI` value.
        issuer: String,
        /// The "sub" (subject) member identifies the principal that is the subject of the
        /// identifier. The subject value MUST either be scoped to be locally unique in the context
        /// of the issuer or be globally unique. The processing of this claim is generally
        /// application specific. The "sub" value is a case-sensitive string containing a
        /// `StringOrURI` value.
        subject: String,
    },
    /// The Opaque Identifier Format describes a subject that is identified with a string with no
    /// semantics asserted beyond its usage as an identifier for the subject, such as a UUID or hash
    /// used as a surrogate identifier for a record in a database. The Opaque Identifier Format is
    /// identified by the name "opaque".
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::Opaque{
    ///     id: "11112222333344445555".to_owned(),
    /// };
    /// # }
    /// ```
    Opaque {
        /// JSON string containing the opaque string identifier for the subject.
        /// The "id" member is REQUIRED and MUST NOT be null or empty.
        id: String,
    },
    /// The Phone Number Identifier Format identifies a subject using a telephone number.
    /// The Phone Number Identifier Format is identified by the name "phone_number".
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::PhoneNumber{
    ///     phone_number: "+12065550100".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    PhoneNumber {
        /// String containing the full telephone number of the subject, including international
        /// dialing prefix, formatted according to E.164 [[`E164`]].
        /// The "phone_number" member is REQUIRED and MUST NOT be null or empty.
        ///
        /// [`E164`]: https://www.itu.int/rec/T-REC-E.164-201011-I/en
        phone_number: String,
    },
    /// The Decentralized Identifier Format identifies a subject using a Decentralized Identifier
    /// (DID) URL as defined in [`DID`].
    /// The Decentralized Identifier Format is identified by the name "did".
    ///
    /// [`DID`]: https://www.w3.org/TR/did-core/
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::DID{
    ///     url: "did:example:123456".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    #[serde(rename = "did")]
    DID {
        /// A DID URL for the DID Subject being identified. The value of the "url" member MUST be
        /// a valid DID URL and MAY be a bare DID.
        /// The "url" member is REQUIRED and MUST NOT be null or empty.
        url: String,
    },
    /// The Uniform Resource Identifier (URI) Format identifies a subject using a URI as defined in
    /// [`RFC3986`]. This identifier format makes no assumptions or guarantees with regard to
    /// the content, scheme, or reachability of the URI within the field.
    /// The URI format is identified by the name "uri".
    ///
    /// [`RFC3986`]: https://www.rfc-editor.org/info/rfc3986
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::URI{
    ///     uri: "urn:uuid:4e851e98-83c4-4743-a5da-150ecb53042f".to_owned(),
    /// };
    /// println!("{:?}", subid);
    /// # }
    /// ```
    #[serde(rename = "uri")]
    URI {
        /// URI for the subject being identified. The "uri" member is REQUIRED and MUST NOT
        /// be null or empty.
        uri: String,
    },
}

impl Id {
    const FORMAT_ACCOUNT: &'static str = "account";
    const FORMAT_EMAIL: &'static str = "email";
    const FORMAT_ISSUER_SUBJECT: &'static str = "iss_sub";
    const FORMAT_OPAQUE: &'static str = "opaque";
    const FORMAT_PHONE_NUMBER: &'static str = "phone_number";
    const FORMAT_DID: &'static str = "did";
    const FORMAT_URI: &'static str = "uri";

    /// The name of the Identifier Format as defined in Security Event Identifier Formats registry.
    ///
    /// ```
    /// # fn main() {
    /// use subjectid::Id;
    /// let subid = Id::Opaque { id: "f5a8887c6be6".to_owned() };
    /// println!("{}", subid.format());
    /// # }
    /// ```
    pub fn format(&self) -> &'static str {
        match self {
            Id::Account { .. } => Id::FORMAT_ACCOUNT,
            Id::Email { .. } => Id::FORMAT_EMAIL,
            Id::IssuerSubject { .. } => Id::FORMAT_ISSUER_SUBJECT,
            Id::Opaque { .. } => Id::FORMAT_OPAQUE,
            Id::PhoneNumber { .. } => Id::FORMAT_PHONE_NUMBER,
            Id::DID { .. } => Id::FORMAT_DID,
            Id::URI { .. } => Id::FORMAT_URI,
        }
    }
}
