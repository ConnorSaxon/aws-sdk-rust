#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]

#![warn(missing_docs)]
//! <p>Amazon Lightsail is the easiest way to get started with Amazon Web Services (Amazon Web Services) for developers who need to build websites or web applications. It includes
//! everything you need to launch your project quickly - instances (virtual private servers),
//! container services, storage buckets, managed databases, SSD-based block storage, static IP
//! addresses, load balancers, content delivery network (CDN) distributions, DNS management of
//! registered domains, and resource snapshots (backups) - for a low, predictable monthly
//! price.</p>
//! <p>You can manage your Lightsail resources using the Lightsail console, Lightsail API,
//! Command Line Interface (CLI), or SDKs. For more information about Lightsail
//! concepts and tasks, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/lightsail-how-to-set-up-access-keys-to-use-sdk-api-cli">Amazon Lightsail Developer Guide</a>.</p>
//! <p>This API Reference provides detailed information about the actions, data types,
//! parameters, and errors of the Lightsail service. For more information about the supported
//! Amazon Web Services Regions, endpoints, and service quotas of the Lightsail service, see
//! <a href="https://docs.aws.amazon.com/general/latest/gr/lightsail.html">Amazon Lightsail Endpoints
//! and Quotas</a> in the <i>Amazon Web Services General Reference</i>.</p>
//! 
//! # Crate Organization
//! 
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//! 
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//! 
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//! 
//! The other modules within this crate are not required for normal usage.


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub(crate) static API_METADATA: aws_http::user_agent::ApiMetadata =
                    aws_http::user_agent::ApiMetadata::new("lightsail", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

pub mod middleware;

mod no_credentials;

pub(crate) mod protocol_serde;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

#[doc(inline)]
pub use client::Client;

