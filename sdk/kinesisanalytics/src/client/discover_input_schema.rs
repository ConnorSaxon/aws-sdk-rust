// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DiscoverInputSchema`](crate::client::fluent_builders::DiscoverInputSchema) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::DiscoverInputSchema::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::DiscoverInputSchema::set_resource_arn): <p>Amazon Resource Name (ARN) of the streaming source.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::DiscoverInputSchema::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::DiscoverInputSchema::set_role_arn): <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf.</p>
    ///   - [`input_starting_position_configuration(InputStartingPositionConfiguration)`](crate::client::fluent_builders::DiscoverInputSchema::input_starting_position_configuration) / [`set_input_starting_position_configuration(Option<InputStartingPositionConfiguration>)`](crate::client::fluent_builders::DiscoverInputSchema::set_input_starting_position_configuration): <p>Point at which you want Amazon Kinesis Analytics to start reading records from the specified streaming source discovery purposes.</p>
    ///   - [`s3_configuration(S3Configuration)`](crate::client::fluent_builders::DiscoverInputSchema::s3_configuration) / [`set_s3_configuration(Option<S3Configuration>)`](crate::client::fluent_builders::DiscoverInputSchema::set_s3_configuration): <p>Specify this parameter to discover a schema from data in an Amazon S3 object.</p>
    ///   - [`input_processing_configuration(InputProcessingConfiguration)`](crate::client::fluent_builders::DiscoverInputSchema::input_processing_configuration) / [`set_input_processing_configuration(Option<InputProcessingConfiguration>)`](crate::client::fluent_builders::DiscoverInputSchema::set_input_processing_configuration): <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to use to preprocess the records before discovering the schema of the records.</p>
                            /// - On success, responds with [`DiscoverInputSchemaOutput`](crate::output::DiscoverInputSchemaOutput) with field(s):
    ///   - [`input_schema(Option<SourceSchema>)`](crate::output::DiscoverInputSchemaOutput::input_schema): <p>Schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>
    ///   - [`parsed_input_records(Option<Vec<Vec<String>>>)`](crate::output::DiscoverInputSchemaOutput::parsed_input_records): <p>An array of elements, where each element corresponds to a row in a stream record (a stream record can have more than one row).</p>
    ///   - [`processed_input_records(Option<Vec<String>>)`](crate::output::DiscoverInputSchemaOutput::processed_input_records): <p>Stream data that was modified by the processor specified in the <code>InputProcessingConfiguration</code> parameter.</p>
    ///   - [`raw_input_records(Option<Vec<String>>)`](crate::output::DiscoverInputSchemaOutput::raw_input_records): <p>Raw stream data that was sampled to infer the schema.</p>
                            /// - On failure, responds with [`SdkError<DiscoverInputSchemaError>`](crate::error::DiscoverInputSchemaError)
    pub fn discover_input_schema(&self) -> crate::client::fluent_builders::DiscoverInputSchema {
                                crate::client::fluent_builders::DiscoverInputSchema::new(self.handle.clone())
                            }
}

