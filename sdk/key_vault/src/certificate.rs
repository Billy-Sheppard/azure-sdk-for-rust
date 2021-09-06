use crate::client::API_VERSION_PARAM;
use crate::Error;
use crate::KeyClient;
use crate::RecoveryLevel;

use azure_core::TokenCredential;
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use const_format::formatcp;
use getset::Getters;
use reqwest::Url;
use serde::Deserialize;
use serde_json::{Map, Value};

const DEFAULT_MAX_RESULTS: usize = 25;

const API_VERSION_MAX_RESULTS_PARAM: &str =
    formatcp!("{}&maxresults={}", API_VERSION_PARAM, DEFAULT_MAX_RESULTS);

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierAttributedRaw {
    enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierRaw {
    id: String,
    x5t: String,
    attributes: KeyVaultCertificateBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificatesResponse {
    value: Vec<KeyVaultCertificateBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponse {
    kid: String,
    sid: String,
    x5t: String,
    cer: String,
    id: String,
    attributes: KeyVaultGetCertificateResponseAttributes,
    policy: KeyVaultGetCertificateResponsePolicy,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponseAttributes {
    enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
    #[serde(rename = "recoveryLevel")]
    recovery_level: String,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicy {
    id: String,
    key_props: KeyVaultGetCertificateResponsePolicyKeyProperties,
    secret_props: KeyVaultGetCertificateResponsePolicySecretProperties,
    x509_props: KeyVaultGetCertificateResponsePolicyX509Properties,
    issuer: KeyVaultGetCertificateResponsePolicyIssuer,
    attributes: KeyVaultGetCertificateResponsePolicyAttributes,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyKeyProperties {
    exportable: bool,
    kty: String,
    key_size: u64,
    reuse_key: bool,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicySecretProperties {
    #[serde(rename = "contentType")]
    content_type: String,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyX509Properties {
    subject: String,
    validity_months: u64,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyIssuer {
    name: String,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyAttributes {
    enabled: bool,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBackupResponseRaw {
    value: String,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultCertificateBackupBlob {
    value: String,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultCertificateBaseIdentifier {
    id: String,
    name: String,
    enabled: bool,
    time_created: DateTime<Utc>,
    time_updated: DateTime<Utc>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultCertificate {
    id: String,
    kid: String,
    sid: String,
    x5t: String,
    cer: String,
    content_type: String,
    enabled: bool,
    not_before: Option<DateTime<Utc>>,
    expires_on: Option<DateTime<Utc>>,
    time_created: DateTime<Utc>,
    time_updated: DateTime<Utc>,
}

impl<'a, T: TokenCredential> KeyClient<'a, T> {
    /// Gets a certificate from the Key Vault.
    /// Note that the latest version is fetched. For a specific version, use `get_certificate_with_version`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     let certificate = client.get_certificate(&"CERTIFICATE_NAME").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_certificate(
        &mut self,
        certificate_name: &'a str,
    ) -> Result<KeyVaultCertificate, Error> {
        Ok(self
            .get_certificate_with_version(certificate_name, "")
            .await?)
    }

    /// Gets a certificate from the Key Vault with a specific version.
    /// If you need the latest version, use `get_certificate`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    /// let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     let certificate = client.get_certificate_with_version(&"CERTIFICATE_NAME", &"CERTIFICATE_VERSION").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_certificate_with_version(
        &mut self,
        certificate_name: &'a str,
        certificate_version_name: &'a str,
    ) -> Result<KeyVaultCertificate, Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!(
            "certificates/{}/{}",
            certificate_name, certificate_version_name
        ));
        uri.set_query(Some(API_VERSION_PARAM));

        let response_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<KeyVaultGetCertificateResponse>(&response_body)
            .map_err(|error| Error::BackupSecretParseError {
                error,
                secret_name: certificate_name.to_string(),
                response_body,
            })?;
        Ok(KeyVaultCertificate {
            id: response.id,
            kid: response.kid,
            sid: response.sid,
            x5t: response.x5t,
            cer: response.cer,
            content_type: response.policy.secret_props.content_type,
            enabled: response.attributes.enabled,
            not_before: response.attributes.nbf,
            expires_on: response.attributes.exp,
            time_created: response.attributes.created,
            time_updated: response.attributes.updated,
        })
    }

    /// Lists all the certificates in the Key Vault.
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     let certificates = client.list_certificates().await.unwrap();
    ///     dbg!(&certificates);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn list_certificates(
        &mut self,
    ) -> Result<Vec<KeyVaultCertificateBaseIdentifier>, Error> {
        let mut certificates = Vec::<KeyVaultCertificateBaseIdentifier>::new();

        let mut uri = self.vault_url.clone();
        uri.set_path("certificates");
        uri.set_query(Some(API_VERSION_MAX_RESULTS_PARAM));

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let response =
                serde_json::from_str::<KeyVaultGetCertificatesResponse>(&resp_body).unwrap();

            certificates.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| KeyVaultCertificateBaseIdentifier {
                        id: s.id.to_owned(),
                        name: s.id.split('/').last().unwrap().to_owned(),
                        enabled: s.attributes.enabled,
                        time_created: s.attributes.created,
                        time_updated: s.attributes.updated,
                    })
                    .collect::<Vec<KeyVaultCertificateBaseIdentifier>>(),
            );

            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        Ok(certificates)
    }

    /// Gets all the versions for a certificate in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     let certificate_versions = client.get_certificate_versions(&"CERTIFICATE_NAME").await.unwrap();
    ///     dbg!(&certificate_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_certificate_versions(
        &mut self,
        certificate_name: &'a str,
    ) -> Result<Vec<KeyVaultCertificateBaseIdentifier>, Error> {
        let mut certificate_versions = Vec::<KeyVaultCertificateBaseIdentifier>::new();

        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/versions", certificate_name));
        uri.set_query(Some(API_VERSION_MAX_RESULTS_PARAM));

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            dbg!(&resp_body);
            let response =
                serde_json::from_str::<KeyVaultGetCertificatesResponse>(&resp_body).unwrap();

            certificate_versions.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| KeyVaultCertificateBaseIdentifier {
                        id: s.id.to_owned(),
                        name: s.id.split('/').last().unwrap().to_owned(),
                        enabled: s.attributes.enabled,
                        time_created: s.attributes.created,
                        time_updated: s.attributes.updated,
                    })
                    .collect::<Vec<KeyVaultCertificateBaseIdentifier>>(),
            );
            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        // Return the certificate versions sorted by the time modified in descending order.
        certificate_versions.sort_by(|a, b| {
            if a.time_updated > b.time_updated {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        Ok(certificate_versions)
    }

    /// Updates whether a certificate version is enabled or not.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - Name of the certificate
    /// * `certificate_version` - Version of the certificate. Use an empty string for the latest version
    /// * `enabled` - New `enabled` value of the certificate
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.update_certificate_enabled(&"CERTIFICATE_NAME", &"", true).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_certificate_enabled(
        &mut self,
        certificate_name: &'a str,
        certificate_version: &'a str,
        enabled: bool,
    ) -> Result<(), Error> {
        let mut attributes = Map::new();
        attributes.insert("enabled".to_owned(), Value::Bool(enabled));

        self.update_certificate_attributes(certificate_name, certificate_version, attributes)
            .await?;

        Ok(())
    }

    /// Updates the [`RecoveryLevel`](RecoveryLevel) of a certificate version.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - Name of the certificate
    /// * `certificate_version` - Version of the certificate. Use an empty string for the latest version
    /// * `recovery_level` - New `RecoveryLevel`(RecoveryLevel) value of the certificate
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::{KeyClient, RecoveryLevel};
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.update_certificate_recovery_level(&"CERTIFICATE_NAME", &"", RecoveryLevel::Purgeable).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_certificate_recovery_level(
        &mut self,
        certificate_name: &'a str,
        certificate_version: &'a str,
        recovery_level: RecoveryLevel,
    ) -> Result<(), Error> {
        let mut attributes = Map::new();
        attributes.insert(
            "enabled".to_owned(),
            Value::String(recovery_level.to_string()),
        );

        self.update_certificate_attributes(certificate_name, certificate_version, attributes)
            .await?;

        Ok(())
    }

    /// Updates the expiration time of a certificate version.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - Name of the certificate
    /// * `certificate_version` - Version of the certificate. Use an empty string for the latest version
    /// * `expiration_time - New expiration time of the certificate
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::{KeyClient, RecoveryLevel};
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    /// use chrono::{Utc, Duration};
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.update_certificate_expiration_time(&"CERTIFICATE_NAME", &"", Utc::now() + Duration::days(14)).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_certificate_expiration_time(
        &mut self,
        certificate_name: &'a str,
        certificate_version: &'a str,
        expiration_time: DateTime<Utc>,
    ) -> Result<(), Error> {
        let mut attributes = Map::new();
        attributes.insert(
            "exp".to_owned(),
            Value::Number(serde_json::Number::from(expiration_time.timestamp())),
        );

        self.update_certificate_attributes(certificate_name, certificate_version, attributes)
            .await?;

        Ok(())
    }

    async fn update_certificate_attributes(
        &mut self,
        certificate_name: &'a str,
        certificate_version: &'a str,
        attributes: Map<String, Value>,
    ) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!(
            "certificates/{}/{}",
            certificate_name, certificate_version
        ));
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert("attributes".to_owned(), Value::Object(attributes));

        self.patch_authed(uri.to_string(), Value::Object(request_body).to_string())
            .await?;

        Ok(())
    }

    async fn _update_certificate_policy(
        &mut self,
        certificate_name: &'a str,
        policy: Map<String, Value>,
    ) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/policy", certificate_name));
        uri.set_query(Some(API_VERSION_PARAM));

        self.patch_authed(uri.to_string(), Value::Object(policy).to_string())
            .await?;

        Ok(())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.restore_certificate(&"KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn restore_certificate(&mut self, backup_blob: &'a str) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path("certificates/restore");
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert("value".to_owned(), Value::String(backup_blob.to_owned()));

        self.post_authed(
            uri.to_string(),
            Some(Value::Object(request_body).to_string()),
        )
        .await?;

        Ok(())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.backup_certificate(&"CERTIFICATE_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn backup_certificate(
        &mut self,
        certificate_name: &'a str,
    ) -> Result<KeyVaultCertificateBackupBlob, Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/backup", certificate_name));
        uri.set_query(Some(API_VERSION_PARAM));

        let response_body = self.post_authed(uri.to_string(), None).await?;
        let backup_blob = serde_json::from_str::<KeyVaultCertificateBackupResponseRaw>(
            &response_body,
        )
        .map_err(|error| Error::BackupSecretParseError {
            error,
            secret_name: certificate_name.to_string(),
            response_body,
        })?;

        Ok(KeyVaultCertificateBackupBlob {
            value: backup_blob.value,
        })
    }

    /// Deletes a certificate in the Key Vault.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - Name of the certificate
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::{KeyClient, RecoveryLevel};
    /// use azure_identity::token_credentials::DefaultCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultCredential::default();
    ///     let mut client = KeyClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.delete_certificate(&"CERTIFICATE_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn delete_certificate(&mut self, certificate_name: &'a str) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}", certificate_name));
        uri.set_query(Some(API_VERSION_PARAM));

        self.delete_authed(uri.to_string()).await?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::client::API_VERSION;
    use crate::mock_client;
    use crate::tests::MockCredential;

    fn diff(first: DateTime<Utc>, second: DateTime<Utc>) -> Duration {
        if first > second {
            first - second
        } else {
            second - first
        }
    }

    #[tokio::test]
    async fn get_certificate() {
        let time_created = Utc::now() - Duration::days(7);
        let time_updated = Utc::now();
        let _m = mock("GET", "/certificates/test-certificate/")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                    "kid": "https://myvault.vault.azure.net/keys/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "sid": "https://myvault.vault.azure.net/secrets/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "cer": "MIICODCCAeagAwIBAgIQqHmpBAv+CY9IJFoUhlbziTAJBgUrDgMCHQUAMBYxFDASBgNVBAMTC1Jvb3QgQWdlbmN5MB4XDTE1MDQyOTIxNTM0MVoXDTM5MTIzMTIzNTk1OVowFzEVMBMGA1UEAxMMS2V5VmF1bHRUZXN0MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5bVAT73zr4+N4WVv2+SvTunAw08ksS4BrJW/nNliz3S9XuzMBMXvmYzU5HJ8TtEgluBiZZYd5qsMJD+OXHSNbsLdmMhni0jYX09h3XlC2VJw2sGKeYF+xEaavXm337aZZaZyjrFBrrUl51UePaN+kVFXNlBb3N3TYpqa7KokXenJQuR+i9Gv9a77c0UsSsDSryxppYhKK7HvTZCpKrhVtulF5iPMswWe9np3uggfMamyIsK/0L7X9w9B2qN7993RR0A00nOk4H6CnkuwO77dSsD0KJsk6FyAoZBzRXDZh9+d9R76zCL506NcQy/jl0lCiQYwsUX73PG5pxOh02OwKwIDAQABo0swSTBHBgNVHQEEQDA+gBAS5AktBh0dTwCNYSHcFmRjoRgwFjEUMBIGA1UEAxMLUm9vdCBBZ2VuY3mCEAY3bACqAGSKEc+41KpcNfQwCQYFKw4DAh0FAANBAGqIjo2geVagzuzaZOe1ClGKhZeiCKfWAxklaGN+qlGUbVS4IN4V1lot3VKnzabasmkEHeNxPwLn1qvSD0cX9CE=",
                    "attributes": {
                        "enabled": true,
                        "created": time_created.timestamp(),
                        "updated": time_updated.timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                    },
                    "policy": {
                        "id": "https://myvault.vault.azure.net/certificates/selfSignedCert01/policy",
                        "key_props": {
                            "exportable": true,
                            "kty": "RSA",
                            "key_size": 2048,
                            "reuse_key": false
                        },
                        "secret_props": {
                            "contentType": "application/x-pkcs12"
                        },
                        "x509_props": {
                            "subject": "CN=KeyVaultTest",
                            "ekus": [],
                            "key_usage": [],
                            "validity_months": 297
                        },
                        "issuer": {
                            "name": "Unknown"
                        },
                        "attributes": {
                            "enabled": true,
                            "created": 1493938289,
                            "updated": 1493938291
                        }
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential;
        dbg!(mockito::server_url());
        let mut client = mock_client!(&"test-keyvault", &creds,);

        let certificate: KeyVaultCertificate =
            client.get_certificate(&"test-certificate").await.unwrap();

        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/002ade539442463aba45c0efb42e3e84",
            certificate.id()
        );
        assert_eq!(true, *certificate.enabled());
        assert!(diff(time_created, *certificate.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated, *certificate.time_updated()) < Duration::seconds(1));
    }

    #[tokio::test]
    async fn get_certificate_versions() {
        let time_created_1 = Utc::now() - Duration::days(7);
        let time_updated_1 = Utc::now();
        let time_created_2 = Utc::now() - Duration::days(9);
        let time_updated_2 = Utc::now() - Duration::days(2);

        let _m1 = mock("GET", "/certificates/test-certificate/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                Matcher::UrlEncoded("maxresults".into(), DEFAULT_MAX_RESULTS.to_string()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_1",
                        "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_1.timestamp(),
                            "updated": time_updated_1.timestamp(),
                        }
                    }],
                    "nextLink": format!("{}/certificates/text-certificate/versions?api-version={}&maxresults=1&$skiptoken=SKIP_TOKEN_MOCK", mockito::server_url(), API_VERSION)
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let _m2 = mock("GET", "/certificates/text-certificate/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                Matcher::UrlEncoded("maxresults".into(), "1".into()),
                Matcher::UrlEncoded("$skiptoken".into(), "SKIP_TOKEN_MOCK".into()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_2",
                        "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_2.timestamp(),
                            "updated": time_updated_2.timestamp(),
                        }
                    }],
                    "nextLink": null
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential;
        let mut client = mock_client!(&"test-keyvault", &creds,);

        let certificate_versions = client
            .get_certificate_versions(&"test-certificate")
            .await
            .unwrap();

        let certificate_1 = &certificate_versions[0];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_1",
            certificate_1.id()
        );
        assert!(diff(time_created_1, *certificate_1.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated_1, *certificate_1.time_updated()) < Duration::seconds(1));

        let certificate_2 = &certificate_versions[1];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_2",
            certificate_2.id()
        );
        assert!(diff(time_created_2, *certificate_2.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated_2, *certificate_2.time_updated()) < Duration::seconds(1));
    }
}
