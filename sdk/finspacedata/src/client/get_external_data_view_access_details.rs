// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetExternalDataViewAccessDetails`](crate::client::fluent_builders::GetExternalDataViewAccessDetails) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`data_view_id(impl Into<String>)`](crate::client::fluent_builders::GetExternalDataViewAccessDetails::data_view_id) / [`set_data_view_id(Option<String>)`](crate::client::fluent_builders::GetExternalDataViewAccessDetails::set_data_view_id): <p>The unique identifier for the Dataview that you want to access.</p>
    ///   - [`dataset_id(impl Into<String>)`](crate::client::fluent_builders::GetExternalDataViewAccessDetails::dataset_id) / [`set_dataset_id(Option<String>)`](crate::client::fluent_builders::GetExternalDataViewAccessDetails::set_dataset_id): <p>The unique identifier for the Dataset.</p>
                            /// - On success, responds with [`GetExternalDataViewAccessDetailsOutput`](crate::output::GetExternalDataViewAccessDetailsOutput) with field(s):
    ///   - [`credentials(Option<AwsCredentials>)`](crate::output::GetExternalDataViewAccessDetailsOutput::credentials): <p>The credentials required to access the external Dataview from the S3 location.</p>
    ///   - [`s3_location(Option<S3Location>)`](crate::output::GetExternalDataViewAccessDetailsOutput::s3_location): <p>The location where the external Dataview is stored.</p>
                            /// - On failure, responds with [`SdkError<GetExternalDataViewAccessDetailsError>`](crate::error::GetExternalDataViewAccessDetailsError)
    pub fn get_external_data_view_access_details(&self) -> crate::client::fluent_builders::GetExternalDataViewAccessDetails {
                                crate::client::fluent_builders::GetExternalDataViewAccessDetails::new(self.handle.clone())
                            }
}

