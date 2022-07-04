use super::mock_response::MockResponse;
use super::mock_transaction::MockTransaction;
use crate::error::{Error, ErrorKind};
use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, TransportOptions};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportPlayerPolicy {
    #[allow(unused)]
    pub(crate) transport_options: TransportOptions,
    transaction: MockTransaction,
}

impl MockTransportPlayerPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        let transaction = MockTransaction::new(transport_options.transaction_name.clone());
        Self {
            transport_options,
            transaction,
        }
    }
}

#[async_trait::async_trait]
impl Policy for MockTransportPlayerPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        // deserialize to file both the request and the response
        let (expected_request, expected_response) = {
            let mut request_path = self.transaction.file_path(false)?;
            let mut response_path = request_path.clone();

            let number = self.transaction.number();
            request_path.push(format!("{}_request.json", number));
            response_path.push(format!("{}_response.json", number));

            let request = std::fs::read_to_string(&request_path)?;
            let response = std::fs::read_to_string(&response_path)?;

            (request, response)
        };

        let expected_request: Request = serde_json::from_str(&expected_request)?;
        let expected_response = serde_json::from_str::<MockResponse>(&expected_response)?;

        let expected_uri = expected_request.path_and_query();
        let actual_uri = request.path_and_query();
        if expected_uri != actual_uri {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched request uri. Actual '{0}', Expected: '{1}'",
                    actual_uri, expected_uri
                )
            }));
        }

        // check if the passed request matches the one read from disk
        // We will ignore some headers that are bound to change every time
        // We'll probabily want to make the exclusion list dynamic at some point.
        const SKIPPED_HEADERS: &[&str] = &["Date", "x-ms-date", "authorization", "user-agent"];
        let actual_headers = request
            .headers()
            .iter()
            .filter(|(h, _)| !SKIPPED_HEADERS.contains(&h.as_str()))
            .collect::<HashMap<_, _>>();

        let expected_headers = expected_request
            .headers()
            .iter()
            .filter(|(h, _)| !SKIPPED_HEADERS.contains(&h.as_str()))
            .collect::<HashMap<_, _>>();
        let more_headers = if expected_headers.len() > actual_headers.len() {
            expected_headers.iter()
        } else {
            actual_headers.iter()
        };

        // In order to accept a request, we make sure that:
        // 1. There are no extra headers (in both the received and read request).
        // 2. Each header has the same value.
        for (name, _) in more_headers {
            match (expected_headers.get(name), actual_headers.get(name)) {
                (Some(_), None) => {
                    return Err(Error::with_message(ErrorKind::MockFramework, || {
                        format!(
                            "actual request does not have header '{0}' but it was expected",
                            name.as_str(),
                        )
                    }));
                }
                (None, Some(_)) => {
                    return Err(Error::with_message(ErrorKind::MockFramework, || {
                        format!(
                            "actual request has header '{0}' but it was not expected",
                            name.as_str(),
                        )
                    }));
                }
                (Some(exp), Some(act)) if exp != act => {
                    return Err(Error::with_message(ErrorKind::MockFramework, || {
                        format!(
                            "request header '{}' is different. Actual: {}, Expected: {}",
                            name.as_str(),
                            act.as_str(),
                            exp.as_str()
                        )
                    }));
                }
                _ => {}
            }
        }

        if expected_request.method() != request.method() {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched HTTP request method. Actual: {0}, Expected: {1}",
                    expected_request.method(),
                    request.method(),
                )
            }));
        }

        let actual_body = match request.body() {
            crate::Body::Bytes(bytes) => bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        let expected_body = match expected_request.body() {
            crate::Body::Bytes(bytes) => bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        if actual_body != expected_body {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched request body. Actual: {0:?}, Expected: {1:?}",
                    actual_body.to_vec(),
                    expected_body.to_vec(),
                )
            }));
        }

        self.transaction.increment_number();
        Ok(expected_response.into())
    }
}
