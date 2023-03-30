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
//! <p>AWS Data Pipeline configures and manages a data-driven workflow called a pipeline. AWS Data Pipeline 
//! handles the details of scheduling and ensuring that data dependencies are met so that your application 
//! can focus on processing the data.</p>
//! 
//! <p>AWS Data Pipeline provides a JAR implementation of a task runner called AWS Data Pipeline Task Runner. 
//! AWS Data Pipeline Task Runner provides logic for common data management scenarios, such as performing 
//! database queries and running data analysis using Amazon Elastic MapReduce (Amazon EMR). You can use 
//! AWS Data Pipeline Task Runner as your task runner, or you can write your own task runner to provide 
//! custom data management.</p>
//! 
//! <p>AWS Data Pipeline implements two main sets of functionality. Use the first set to create a pipeline 
//! and define data sources, schedules, dependencies, and the transforms to be performed on the data. 
//! Use the second set in your task runner application to receive the next task ready for processing. 
//! The logic for performing the task, such as querying the data, running data analysis, or converting 
//! the data from one format to another, is contained within the task runner. The task runner performs 
//! the task assigned to it by the web service, reporting progress to the web service as it does so. 
//! When the task is done, the task runner reports the final success or failure of the task to the web service.</p>
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
                    aws_http::user_agent::ApiMetadata::new("datapipeline", crate::PKG_VERSION);

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

/// Paginators for the service
pub mod paginator;

/// Generated accessors for nested fields
mod lens;

pub(crate) mod protocol_serde;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

#[doc(inline)]
pub use client::Client;

