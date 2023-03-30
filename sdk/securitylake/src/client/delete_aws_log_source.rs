// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAwsLogSource`](crate::client::fluent_builders::DeleteAwsLogSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`input_order(Vec<Dimension>)`](crate::client::fluent_builders::DeleteAwsLogSource::input_order) / [`set_input_order(Option<Vec<Dimension>>)`](crate::client::fluent_builders::DeleteAwsLogSource::set_input_order): <p>This is a mandatory input. Specify the input order to disable dimensions in Security Lake, namely Region (Amazon Web Services Region code, source type, and member (account ID of a specific Amazon Web Services account). </p>
    ///   - [`disable_all_dimensions(HashMap<String, HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::DeleteAwsLogSource::disable_all_dimensions) / [`set_disable_all_dimensions(Option<HashMap<String, HashMap<String, Vec<String>>>>)`](crate::client::fluent_builders::DeleteAwsLogSource::set_disable_all_dimensions): <p>Removes the specific Amazon Web Services sources from specific accounts and specific Regions.</p>
    ///   - [`disable_two_dimensions(HashMap<String, Vec<String>>)`](crate::client::fluent_builders::DeleteAwsLogSource::disable_two_dimensions) / [`set_disable_two_dimensions(Option<HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::DeleteAwsLogSource::set_disable_two_dimensions): <p>Remove a specific Amazon Web Services source from specific accounts or Regions.</p>
    ///   - [`disable_single_dimension(Vec<String>)`](crate::client::fluent_builders::DeleteAwsLogSource::disable_single_dimension) / [`set_disable_single_dimension(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteAwsLogSource::set_disable_single_dimension): <p>Removes all Amazon Web Services sources from specific accounts or Regions.</p>
                            /// - On success, responds with [`DeleteAwsLogSourceOutput`](crate::output::DeleteAwsLogSourceOutput) with field(s):
    ///   - [`processing(Option<Vec<String>>)`](crate::output::DeleteAwsLogSourceOutput::processing): <p>Deletion of the Amazon Web Services sources is in progress.</p>
    ///   - [`failed(Option<Vec<String>>)`](crate::output::DeleteAwsLogSourceOutput::failed): <p>Deletion of the Amazon Web Services sources failed as the account is not a part of the organization.</p>
                            /// - On failure, responds with [`SdkError<DeleteAwsLogSourceError>`](crate::error::DeleteAwsLogSourceError)
    pub fn delete_aws_log_source(&self) -> crate::client::fluent_builders::DeleteAwsLogSource {
                                crate::client::fluent_builders::DeleteAwsLogSource::new(self.handle.clone())
                            }
}

