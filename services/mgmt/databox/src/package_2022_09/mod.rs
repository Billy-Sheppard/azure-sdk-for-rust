#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets all the operations."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::OperationList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.DataBox/operations", this.client.endpoint(),))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the jobs available under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                skip_token: None,
            }
        }
        #[doc = "Request to mark devices for a given job as shipped"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `mark_devices_shipped_request`: Mark Devices Shipped Request"]
        pub fn mark_devices_shipped(
            &self,
            job_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            mark_devices_shipped_request: impl Into<models::MarkDevicesShippedRequest>,
        ) -> mark_devices_shipped::RequestBuilder {
            mark_devices_shipped::RequestBuilder {
                client: self.0.clone(),
                job_name: job_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                mark_devices_shipped_request: mark_devices_shipped_request.into(),
            }
        }
        #[doc = "Lists all the jobs available under the given resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                skip_token: None,
            }
        }
        #[doc = "Gets information about the specified job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
                expand: None,
            }
        }
        #[doc = "Creates a new job with the specified parameters. Existing job cannot be updated with this API and should instead be updated with the Update job API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `job_resource`: Job details from request body."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
            job_resource: impl Into<models::JobResource>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
                job_resource: job_resource.into(),
            }
        }
        #[doc = "Updates the properties of an existing job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `job_resource_update_parameter`: Job update parameters from request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
            job_resource_update_parameter: impl Into<models::JobResourceUpdateParameter>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
                job_resource_update_parameter: job_resource_update_parameter.into(),
                if_match: None,
            }
        }
        #[doc = "Deletes a job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Book shipment pick up."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `shipment_pick_up_request`: Details of shipment pick up request."]
        pub fn book_shipment_pick_up(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
            shipment_pick_up_request: impl Into<models::ShipmentPickUpRequest>,
        ) -> book_shipment_pick_up::RequestBuilder {
            book_shipment_pick_up::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
                shipment_pick_up_request: shipment_pick_up_request.into(),
            }
        }
        #[doc = "CancelJob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `cancellation_reason`: Reason for cancellation."]
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
            cancellation_reason: impl Into<models::CancellationReason>,
        ) -> cancel::RequestBuilder {
            cancel::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
                cancellation_reason: cancellation_reason.into(),
            }
        }
        #[doc = "This method gets the unencrypted secrets related to the job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_credentials(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            job_name: impl Into<String>,
        ) -> list_credentials::RequestBuilder {
            list_credentials::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                job_name: job_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JobResourceList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JobResourceList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "$skipToken is supported on Get list of jobs, which provides the next page in the list of jobs."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::JobResourceList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataBox/jobs",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod mark_devices_shipped {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) job_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) mark_devices_shipped_request: models::MarkDevicesShippedRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}/markDevicesShipped",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.mark_devices_shipped_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JobResourceList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JobResourceList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "$skipToken is supported on Get list of jobs, which provides the next page in the list of jobs."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::JobResourceList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JobResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JobResource = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "$expand is supported on details parameter for job, which provides details on the job stages."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::JobResource>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JobResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JobResource = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
            pub(crate) job_resource: models::JobResource,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.job_resource)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::JobResource>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JobResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JobResource = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The URI to poll for completion status."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
            pub(crate) job_resource_update_parameter: models::JobResourceUpdateParameter,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Defines the If-Match condition. The patch will be performed only if the ETag of the job on the server matches this value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.job_resource_update_parameter)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::JobResource>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod book_shipment_pick_up {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ShipmentPickUpResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ShipmentPickUpResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
            pub(crate) shipment_pick_up_request: models::ShipmentPickUpRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}/bookShipmentPickUp",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.shipment_pick_up_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::ShipmentPickUpResponse>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod cancel {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
            pub(crate) cancellation_reason: models::CancellationReason,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.cancellation_reason)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_credentials {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::UnencryptedCredentialsList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::UnencryptedCredentialsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) job_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}/listCredentials",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.job_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::UnencryptedCredentialsList>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
impl Client {
    #[doc = "Request to mitigate for a given job"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `job_name`: The name of the job Resource within the specified resource group. job names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
    #[doc = "* `subscription_id`: The Subscription Id"]
    #[doc = "* `resource_group_name`: The Resource Group Name"]
    #[doc = "* `mitigate_job_request`: Mitigation Request"]
    pub fn mitigate(
        &self,
        job_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        mitigate_job_request: impl Into<models::MitigateJobRequest>,
    ) -> mitigate::RequestBuilder {
        mitigate::RequestBuilder {
            client: self.clone(),
            job_name: job_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            mitigate_job_request: mitigate_job_request.into(),
        }
    }
}
pub mod mitigate {
    use super::models;
    pub struct Response(azure_core::Response);
    #[derive(Clone)]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) job_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) mitigate_job_request: models::MitigateJobRequest,
    }
    impl RequestBuilder {
        #[doc = "Send the request and returns the response."]
        pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/jobs/{}/mitigate",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.job_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.mitigate_job_request)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method provides the list of available skus for the given subscription, resource group and location."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `available_sku_request`: Filters for showing the available skus."]
        pub fn list_available_skus_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            location: impl Into<String>,
            available_sku_request: impl Into<models::AvailableSkuRequest>,
        ) -> list_available_skus_by_resource_group::RequestBuilder {
            list_available_skus_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                location: location.into(),
                available_sku_request: available_sku_request.into(),
            }
        }
        #[doc = "[DEPRECATED NOTICE: This operation will soon be removed]. This method validates the customer shipping address and provide alternate addresses if any."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `validate_address`: Shipping address of the customer."]
        pub fn validate_address(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
            validate_address: impl Into<models::ValidateAddress>,
        ) -> validate_address::RequestBuilder {
            validate_address::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                validate_address: validate_address.into(),
            }
        }
        #[doc = "This method does all necessary pre-job creation validation under resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `validation_request`: Inputs of the customer."]
        pub fn validate_inputs_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            location: impl Into<String>,
            validation_request: impl Into<models::ValidationRequest>,
        ) -> validate_inputs_by_resource_group::RequestBuilder {
            validate_inputs_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                location: location.into(),
                validation_request: validation_request.into(),
            }
        }
        #[doc = "This method does all necessary pre-job creation validation under subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `validation_request`: Inputs of the customer."]
        pub fn validate_inputs(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
            validation_request: impl Into<models::ValidationRequest>,
        ) -> validate_inputs::RequestBuilder {
            validate_inputs::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                validation_request: validation_request.into(),
            }
        }
        #[doc = "This API provides configuration details specific to given region/location at Subscription level."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `region_configuration_request`: Request body to get the configuration for the region."]
        pub fn region_configuration(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
            region_configuration_request: impl Into<models::RegionConfigurationRequest>,
        ) -> region_configuration::RequestBuilder {
            region_configuration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                region_configuration_request: region_configuration_request.into(),
            }
        }
        #[doc = "This API provides configuration details specific to given region/location at Resource group level."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `location`: The location of the resource"]
        #[doc = "* `region_configuration_request`: Request body to get the configuration for the region at resource group level."]
        pub fn region_configuration_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            location: impl Into<String>,
            region_configuration_request: impl Into<models::RegionConfigurationRequest>,
        ) -> region_configuration_by_resource_group::RequestBuilder {
            region_configuration_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                location: location.into(),
                region_configuration_request: region_configuration_request.into(),
            }
        }
    }
    pub mod list_available_skus_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AvailableSkusResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AvailableSkusResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) location: String,
            pub(crate) available_sku_request: models::AvailableSkuRequest,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::AvailableSkusResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/locations/{}/availableSkus",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.location
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                                req.insert_header("content-type", "application/json");
                                let req_body = azure_core::to_json(&this.available_sku_request)?;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod validate_address {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AddressValidationOutput> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AddressValidationOutput = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) validate_address: models::ValidateAddress,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataBox/locations/{}/validateAddress",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validate_address)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::AddressValidationOutput>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod validate_inputs_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ValidationResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ValidationResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) location: String,
            pub(crate) validation_request: models::ValidationRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/locations/{}/validateInputs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.location
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validation_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::ValidationResponse>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod validate_inputs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ValidationResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ValidationResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) validation_request: models::ValidationRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataBox/locations/{}/validateInputs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validation_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::ValidationResponse>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod region_configuration {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RegionConfigurationResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RegionConfigurationResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) region_configuration_request: models::RegionConfigurationRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataBox/locations/{}/regionConfiguration",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.region_configuration_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::RegionConfigurationResponse>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod region_configuration_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RegionConfigurationResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RegionConfigurationResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) location: String,
            pub(crate) region_configuration_request: models::RegionConfigurationRequest,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataBox/locations/{}/regionConfiguration",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.location
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-09-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.region_configuration_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::RegionConfigurationResponse>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
