#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-08-01-preview")]
pub mod package_2022_08_01_preview;
#[cfg(all(feature = "package-2022-08-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_08_01_preview::*;
#[cfg(feature = "package-2022-02-01")]
pub mod package_2022_02_01;
#[cfg(all(feature = "package-2022-02-01", not(feature = "no-default-tag")))]
pub use package_2022_02_01::*;
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(all(feature = "package-2021-10-01", not(feature = "no-default-tag")))]
pub use package_2021_10_01::*;
#[cfg(feature = "package-2021-09-01-preview")]
pub mod package_2021_09_01_preview;
#[cfg(all(feature = "package-2021-09-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_09_01_preview::*;
#[cfg(feature = "package-2021-06-01-preview")]
pub mod package_2021_06_01_preview;
#[cfg(all(feature = "package-2021-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_01_preview::*;
