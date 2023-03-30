// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFHIRDatastore`](crate::client::fluent_builders::CreateFHIRDatastore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`datastore_name(impl Into<String>)`](crate::client::fluent_builders::CreateFHIRDatastore::datastore_name) / [`set_datastore_name(Option<String>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_datastore_name): <p>The user generated name for the Data Store.</p>
    ///   - [`datastore_type_version(FhirVersion)`](crate::client::fluent_builders::CreateFHIRDatastore::datastore_type_version) / [`set_datastore_type_version(Option<FhirVersion>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_datastore_type_version): <p>The FHIR version of the Data Store. The only supported version is R4.</p>
    ///   - [`sse_configuration(SseConfiguration)`](crate::client::fluent_builders::CreateFHIRDatastore::sse_configuration) / [`set_sse_configuration(Option<SseConfiguration>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_sse_configuration): <p> The server-side encryption key configuration for a customer provided encryption key specified for creating a Data Store. </p>
    ///   - [`preload_data_config(PreloadDataConfig)`](crate::client::fluent_builders::CreateFHIRDatastore::preload_data_config) / [`set_preload_data_config(Option<PreloadDataConfig>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_preload_data_config): <p>Optional parameter to preload data upon creation of the Data Store. Currently, the only supported preloaded data is synthetic data generated from Synthea.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateFHIRDatastore::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_client_token): <p>Optional user provided token used for ensuring idempotency.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateFHIRDatastore::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateFHIRDatastore::set_tags): <p> Resource tags that are applied to a Data Store when it is created. </p>
                            /// - On success, responds with [`CreateFhirDatastoreOutput`](crate::output::CreateFhirDatastoreOutput) with field(s):
    ///   - [`datastore_id(Option<String>)`](crate::output::CreateFhirDatastoreOutput::datastore_id): <p>The AWS-generated Data Store id. This id is in the output from the initial Data Store creation call.</p>
    ///   - [`datastore_arn(Option<String>)`](crate::output::CreateFhirDatastoreOutput::datastore_arn): <p>The datastore ARN is generated during the creation of the Data Store and can be found in the output from the initial Data Store creation call.</p>
    ///   - [`datastore_status(Option<DatastoreStatus>)`](crate::output::CreateFhirDatastoreOutput::datastore_status): <p>The status of the FHIR Data Store. Possible statuses are ‘CREATING’, ‘ACTIVE’, ‘DELETING’, ‘DELETED’.</p>
    ///   - [`datastore_endpoint(Option<String>)`](crate::output::CreateFhirDatastoreOutput::datastore_endpoint): <p>The AWS endpoint for the created Data Store. For preview, only US-east-1 endpoints are supported.</p>
                            /// - On failure, responds with [`SdkError<CreateFHIRDatastoreError>`](crate::error::CreateFHIRDatastoreError)
    pub fn create_fhir_datastore(&self) -> crate::client::fluent_builders::CreateFHIRDatastore {
                                crate::client::fluent_builders::CreateFHIRDatastore::new(self.handle.clone())
                            }
}

