//! `single` module defines subject identifier formats that does not nest other subject identifiers
//! themselves (unlike `aliases` format). So, these are atomic subject identifiers and aliases
//! format is composed of such atomic subject identifiers.

use serde::{Deserialize, Serialize};

use crate::SubjectId;

/// Atomic defines atomic subject identifier formats. They are 'atomic' because (unlike aliases)
/// these are not composed of other subject identifiers themselves.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "format")]
pub enum Atomic {
    /// The Account Identifier Format identifies a subject using an account at a service provider,
    /// identified with an "acct" URI as defined in [`RFC7565`]. An account is an arrangement or
    /// agreement through which a user gets access to a service and gets a unique identity with
    /// the service provider. The URI scheme that could be used to generically identify a user's
    /// account at a service provider, irrespective of the particular application protocols used to
    /// interact with the account.
    ///
    /// The Account Identifier Format is identified by a value of "account" in the "format" member,
    /// that corresponds to [`FORMAT_ACCOUNT`].
    ///
    /// [`RFC7565`]: https://www.rfc-editor.org/info/rfc7565
    /// [`FORMAT_ACCOUNT`]: crate::SubjectId::FORMAT_ACCOUNT
    ///
    /// ```
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::Account{
    ///     uri: "acct:example.user@service.example.com".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
    /// ```
    Account {
        /// The "acct" URI for the subject. The "uri" member is REQUIRED and MUST NOT be null or
        /// empty.
        ///
        /// Note that "acct" URI points to an account at the service provider. The account holder
        /// need not necessarily be human; it could be automated bot, a role-based alias or a
        /// representative account for a community or organization of individuals.
        uri: String,
    },
    /// The Email Identifier Format identifies a subject using an email address. The value of the
    /// "email" member MUST identify a mailbox to which email may be delivered, in accordance with
    /// [`RFC5321`].
    ///
    /// Email canonicalization is not standardized, and there is no way for the event recipient to
    /// determine the mail provider’s canonicalization method. Therefore, the recipient SHOULD apply
    /// its own canonicalization algorithm to incoming events that reproduces the translation done
    /// by the local email system.
    ///
    /// The Email Identifier Format is identified by the name "email", that corresponds to
    /// [`FORMAT_EMAIL`].
    ///
    /// [`RFC5321`]: https://www.rfc-editor.org/info/rfc5321
    /// [`FORMAT_EMAIL`]: crate::SubjectId::FORMAT_EMAIL
    ///
    /// ```
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::Email{
    ///     email: "user@example.com".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
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
    /// and the "sub" member are REQUIRED and MUST NOT be null or empty.
    ///
    /// The Issuer and Subject Identifier Format is identified by the name "iss_sub", that
    /// corresponds to [`FORMAT_ISSUER_SUBJECT`].
    ///
    /// [`OpenID.Core`]: https://openid.net/specs/openid-connect-core-1_0.html
    /// [`RFC7519`]: https://www.rfc-editor.org/info/rfc7519
    /// [`FORMAT_ISSUER_SUBJECT`]: crate::SubjectId::FORMAT_ISSUER_SUBJECT
    ///
    /// ```
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::IssuerSubject{
    ///     issuer: "https://issuer.example.com/".to_owned(),
    ///     subject: "145234573".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
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
    /// used as a surrogate identifier for a record in a database.
    ///
    /// The Opaque Identifier Format is identified by the name "opaque", that corresponds to
    /// [`FORMAT_OPAQUE`].
    ///
    /// [`FORMAT_OPAQUE`]: crate::SubjectId::FORMAT_OPAQUE
    ///
    /// ```
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::Opaque{
    ///     id: "11112222333344445555".to_owned(),
    /// };
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
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::PhoneNumber{
    ///     phone_number: "+12065550100".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
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
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::Did{
    ///     url: "did:example:123456".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
    /// ```
    Did {
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
    /// use subjectid::Atomic;
    /// let sub_id = Atomic::Uri{
    ///     uri: "urn:uuid:4e851e98-83c4-4743-a5da-150ecb53042f".to_owned(),
    /// };
    /// println!("{:?}", sub_id);
    /// ```
    Uri {
        /// URI for the subject being identified. The "uri" member is REQUIRED and MUST NOT
        /// be null or empty.
        uri: String,
    },
}

impl Atomic {
    /// Given an Atomic subject identifier, [`format`] returns the subject identifier format of the
    /// atomic identifier.
    ///
    /// ```
    /// use subjectid::{Atomic, SubjectId};
    /// let sub_id = Atomic::Opaque { id: "lqk23j5l".to_owned() };
    /// let fmt = sub_id.format();
    /// assert_eq!(fmt, SubjectId::FORMAT_OPAQUE);
    /// ```
    pub fn format(&self) -> &'static str {
        match self {
            Atomic::Account { .. } => SubjectId::FORMAT_ACCOUNT,
            Atomic::Email { .. } => SubjectId::FORMAT_EMAIL,
            Atomic::IssuerSubject { .. } => SubjectId::FORMAT_ISSUER_SUBJECT,
            Atomic::Opaque { .. } => SubjectId::FORMAT_OPAQUE,
            Atomic::PhoneNumber { .. } => SubjectId::FORMAT_PHONE_NUMBER,
            Atomic::Did { .. } => SubjectId::FORMAT_DID,
            Atomic::Uri { .. } => SubjectId::FORMAT_URI,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        let cases = vec![
            Atomic::Account {
                uri: "acct:someone@example.com".to_owned(),
            },
            Atomic::Email {
                email: "someone@example.com".to_owned(),
            },
            Atomic::IssuerSubject {
                issuer: "example.com".to_owned(),
                subject: "2kj34hk".to_owned(),
            },
            Atomic::Opaque {
                id: "khj23dj5k".to_owned(),
            },
            Atomic::PhoneNumber {
                phone_number: "+68482245895".to_owned(),
            },
            Atomic::Did {
                url: "did:example:1234".to_owned(),
            },
            Atomic::Uri {
                uri: "urn:ietf:rfc:2648".to_owned(),
            },
        ];
        let expected = vec![
            SubjectId::FORMAT_ACCOUNT,
            SubjectId::FORMAT_EMAIL,
            SubjectId::FORMAT_ISSUER_SUBJECT,
            SubjectId::FORMAT_OPAQUE,
            SubjectId::FORMAT_PHONE_NUMBER,
            SubjectId::FORMAT_DID,
            SubjectId::FORMAT_URI,
        ];
        for (got, want) in cases.iter().map(Atomic::format).zip(expected) {
            assert_eq!(got, want, "format values do not match");
        }
    }
}
