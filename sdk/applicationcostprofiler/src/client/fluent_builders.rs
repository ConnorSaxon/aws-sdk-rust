// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Fluent builder constructing a request to `DeleteReportDefinition`.
/// 
/// <p>Deletes the specified report definition in AWS Application Cost Profiler. This stops the report from being generated.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteReportDefinition {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::delete_report_definition_input::Builder
            }
impl DeleteReportDefinition  {
    /// Creates a new `DeleteReportDefinition`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::DeleteReportDefinition, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::DeleteReportDefinitionOutput, aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>Required. ID of the report to delete.</p>
    pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_id(input.into());
        self
    }
    /// <p>Required. ID of the report to delete.</p>
    pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_id(input);
        self
    }
}

/// Fluent builder constructing a request to `GetReportDefinition`.
/// 
/// <p>Retrieves the definition of a report already configured in AWS Application Cost Profiler.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetReportDefinition {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::get_report_definition_input::Builder
            }
impl GetReportDefinition  {
    /// Creates a new `GetReportDefinition`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::GetReportDefinition, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::GetReportDefinitionError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::GetReportDefinitionOutput, aws_smithy_http::result::SdkError<crate::error::GetReportDefinitionError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>ID of the report to retrieve.</p>
    pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_id(input.into());
        self
    }
    /// <p>ID of the report to retrieve.</p>
    pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_id(input);
        self
    }
}

/// Fluent builder constructing a request to `ImportApplicationUsage`.
/// 
/// <p>Ingests application usage data from Amazon Simple Storage Service (Amazon S3).</p> 
/// <p>The data must already exist in the S3 location. As part of the action, AWS Application Cost Profiler copies the object from your S3 bucket to an S3 bucket owned by Amazon for processing asynchronously.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportApplicationUsage {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::import_application_usage_input::Builder
            }
impl ImportApplicationUsage  {
    /// Creates a new `ImportApplicationUsage`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ImportApplicationUsage, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ImportApplicationUsageError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ImportApplicationUsageOutput, aws_smithy_http::result::SdkError<crate::error::ImportApplicationUsageError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>Amazon S3 location to import application usage data from.</p>
    pub fn source_s3_location(mut self, input: crate::model::SourceS3Location) -> Self {
        self.inner = self.inner.source_s3_location(input);
        self
    }
    /// <p>Amazon S3 location to import application usage data from.</p>
    pub fn set_source_s3_location(mut self, input: std::option::Option<crate::model::SourceS3Location>) -> Self {
        self.inner = self.inner.set_source_s3_location(input);
        self
    }
}

/// Fluent builder constructing a request to `ListReportDefinitions`.
/// 
/// <p>Retrieves a list of all reports and their configurations for your AWS account.</p> 
/// <p>The maximum number of reports is one.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListReportDefinitions {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_report_definitions_input::Builder
            }
impl ListReportDefinitions  {
    /// Creates a new `ListReportDefinitions`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListReportDefinitions, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListReportDefinitionsError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ListReportDefinitionsOutput, aws_smithy_http::result::SdkError<crate::error::ListReportDefinitionsError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// Create a paginator for this request
                        ///
                        /// Paginators are used by calling [`send().await`](crate::paginator::ListReportDefinitionsPaginator::send) which returns a `Stream`.
                        pub fn into_paginator(self) -> crate::paginator::ListReportDefinitionsPaginator {
                            crate::paginator::ListReportDefinitionsPaginator::new(self.handle, self.inner)
                        }
    /// <p>The token value from a previous call to access the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token value from a previous call to access the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}

/// Fluent builder constructing a request to `PutReportDefinition`.
/// 
/// <p>Creates the report definition for a report in Application Cost Profiler.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutReportDefinition {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::put_report_definition_input::Builder
            }
impl PutReportDefinition  {
    /// Creates a new `PutReportDefinition`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::PutReportDefinition, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::PutReportDefinitionOutput, aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>Required. ID of the report. You can choose any valid string matching the pattern for the ID.</p>
    pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_id(input.into());
        self
    }
    /// <p>Required. ID of the report. You can choose any valid string matching the pattern for the ID.</p>
    pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_id(input);
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_description(input.into());
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn set_report_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_description(input);
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
        self.inner = self.inner.report_frequency(input);
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn set_report_frequency(mut self, input: std::option::Option<crate::model::ReportFrequency>) -> Self {
        self.inner = self.inner.set_report_frequency(input);
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn format(mut self, input: crate::model::Format) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
        self.inner = self.inner.destination_s3_location(input);
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn set_destination_s3_location(mut self, input: std::option::Option<crate::model::S3Location>) -> Self {
        self.inner = self.inner.set_destination_s3_location(input);
        self
    }
}

/// Fluent builder constructing a request to `UpdateReportDefinition`.
/// 
/// <p>Updates existing report in AWS Application Cost Profiler.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateReportDefinition {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::update_report_definition_input::Builder
            }
impl UpdateReportDefinition  {
    /// Creates a new `UpdateReportDefinition`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::UpdateReportDefinition, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::UpdateReportDefinitionError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::UpdateReportDefinitionOutput, aws_smithy_http::result::SdkError<crate::error::UpdateReportDefinitionError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>Required. ID of the report to update.</p>
    pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_id(input.into());
        self
    }
    /// <p>Required. ID of the report to update.</p>
    pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_id(input);
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_description(input.into());
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn set_report_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_report_description(input);
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
        self.inner = self.inner.report_frequency(input);
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn set_report_frequency(mut self, input: std::option::Option<crate::model::ReportFrequency>) -> Self {
        self.inner = self.inner.set_report_frequency(input);
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn format(mut self, input: crate::model::Format) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
        self.inner = self.inner.destination_s3_location(input);
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn set_destination_s3_location(mut self, input: std::option::Option<crate::model::S3Location>) -> Self {
        self.inner = self.inner.set_destination_s3_location(input);
        self
    }
}

