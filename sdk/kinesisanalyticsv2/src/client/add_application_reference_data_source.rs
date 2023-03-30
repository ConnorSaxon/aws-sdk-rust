// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddApplicationReferenceDataSource`](crate::client::fluent_builders::AddApplicationReferenceDataSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::set_application_name): <p>The name of an existing application.</p>
    ///   - [`current_application_version_id(i64)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::current_application_version_id) / [`set_current_application_version_id(Option<i64>)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::set_current_application_version_id): <p>The version of the application for which you are adding the reference data source. You can use the <code>DescribeApplication</code> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    ///   - [`reference_data_source(ReferenceDataSource)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::reference_data_source) / [`set_reference_data_source(Option<ReferenceDataSource>)`](crate::client::fluent_builders::AddApplicationReferenceDataSource::set_reference_data_source): <p>The reference data source can be an object in your Amazon S3 bucket. Kinesis Data Analytics reads the object and copies the data into the in-application table that is created. You provide an S3 bucket, object key name, and the resulting in-application table that is created. </p>
                            /// - On success, responds with [`AddApplicationReferenceDataSourceOutput`](crate::output::AddApplicationReferenceDataSourceOutput) with field(s):
    ///   - [`application_arn(Option<String>)`](crate::output::AddApplicationReferenceDataSourceOutput::application_arn): <p>The application Amazon Resource Name (ARN).</p>
    ///   - [`application_version_id(Option<i64>)`](crate::output::AddApplicationReferenceDataSourceOutput::application_version_id): <p>The updated application version ID. Kinesis Data Analytics increments this ID when the application is updated.</p>
    ///   - [`reference_data_source_descriptions(Option<Vec<ReferenceDataSourceDescription>>)`](crate::output::AddApplicationReferenceDataSourceOutput::reference_data_source_descriptions): <p>Describes reference data sources configured for the application. </p>
                            /// - On failure, responds with [`SdkError<AddApplicationReferenceDataSourceError>`](crate::error::AddApplicationReferenceDataSourceError)
    pub fn add_application_reference_data_source(&self) -> crate::client::fluent_builders::AddApplicationReferenceDataSource {
                                crate::client::fluent_builders::AddApplicationReferenceDataSource::new(self.handle.clone())
                            }
}

