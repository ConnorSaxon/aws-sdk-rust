// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteApplicationReferenceDataSource`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::set_application_name): <p>The name of an existing application.</p>
    ///   - [`current_application_version_id(i64)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::current_application_version_id) / [`set_current_application_version_id(Option<i64>)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::set_current_application_version_id): <p>The current application version. You can use the <code>DescribeApplication</code> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    ///   - [`reference_id(impl Into<String>)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::reference_id) / [`set_reference_id(Option<String>)`](crate::client::fluent_builders::DeleteApplicationReferenceDataSource::set_reference_id): <p>The ID of the reference data source. When you add a reference data source to your application using the <code>AddApplicationReferenceDataSource</code>, Kinesis Data Analytics assigns an ID. You can use the <code>DescribeApplication</code> operation to get the reference ID. </p>
                            /// - On success, responds with [`DeleteApplicationReferenceDataSourceOutput`](crate::output::DeleteApplicationReferenceDataSourceOutput) with field(s):
    ///   - [`application_arn(Option<String>)`](crate::output::DeleteApplicationReferenceDataSourceOutput::application_arn): <p>The application Amazon Resource Name (ARN).</p>
    ///   - [`application_version_id(Option<i64>)`](crate::output::DeleteApplicationReferenceDataSourceOutput::application_version_id): <p>The updated version ID of the application.</p>
                            /// - On failure, responds with [`SdkError<DeleteApplicationReferenceDataSourceError>`](crate::error::DeleteApplicationReferenceDataSourceError)
    pub fn delete_application_reference_data_source(&self) -> crate::client::fluent_builders::DeleteApplicationReferenceDataSource {
                                crate::client::fluent_builders::DeleteApplicationReferenceDataSource::new(self.handle.clone())
                            }
}

