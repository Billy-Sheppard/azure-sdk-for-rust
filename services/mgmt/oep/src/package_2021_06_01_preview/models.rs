#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of Energy services resource's Data Partition Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPartitionNames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DataPartitionNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition used for updating a tracked ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnergyResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl EnergyResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnergyService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnergyServiceProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Geo-location where the resource lives."]
    pub location: String,
}
impl EnergyService {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            properties: None,
            tags: None,
            location,
        }
    }
}
#[doc = "The list of oep resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnergyServiceList {
    #[doc = "The link used to get the next page of oep resources list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of oep resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnergyService>,
}
impl azure_core::Continuable for EnergyServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnergyServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnergyServiceProperties {
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<energy_service_properties::ProvisioningState>,
    #[serde(rename = "authAppId", default, skip_serializing_if = "Option::is_none")]
    pub auth_app_id: Option<String>,
    #[serde(
        rename = "dataPartitionNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_partition_names: Vec<DataPartitionNames>,
}
impl EnergyServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod energy_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Succeeded,
        Failed,
        Canceled,
        Creating,
        Deleting,
        Updating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
