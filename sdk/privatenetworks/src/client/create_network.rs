// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNetwork`](crate::client::fluent_builders::CreateNetwork) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_name(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::network_name) / [`set_network_name(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_network_name): <p>The name of the network. You can't change the name after you create the network.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_description): <p>The description of the network.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateNetwork::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateNetwork::set_tags): <p> The tags to apply to the network. </p>
                            /// - On success, responds with [`CreateNetworkOutput`](crate::output::CreateNetworkOutput) with field(s):
    ///   - [`network(Option<Network>)`](crate::output::CreateNetworkOutput::network): <p>Information about the network.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateNetworkOutput::tags): <p> The network tags. </p>
                            /// - On failure, responds with [`SdkError<CreateNetworkError>`](crate::error::CreateNetworkError)
    pub fn create_network(&self) -> crate::client::fluent_builders::CreateNetwork {
                                crate::client::fluent_builders::CreateNetwork::new(self.handle.clone())
                            }
}

