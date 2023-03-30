// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNetwork`](crate::client::fluent_builders::CreateNetwork) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_client_request_token): <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than once. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the Amazon Web Services CLI. </p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_name): <p>The name of the network.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_description): <p>An optional description for the network.</p>
    ///   - [`framework(Framework)`](crate::client::fluent_builders::CreateNetwork::framework) / [`set_framework(Option<Framework>)`](crate::client::fluent_builders::CreateNetwork::set_framework): <p>The blockchain framework that the network uses.</p>
    ///   - [`framework_version(impl Into<String>)`](crate::client::fluent_builders::CreateNetwork::framework_version) / [`set_framework_version(Option<String>)`](crate::client::fluent_builders::CreateNetwork::set_framework_version): <p>The version of the blockchain framework that the network uses.</p>
    ///   - [`framework_configuration(NetworkFrameworkConfiguration)`](crate::client::fluent_builders::CreateNetwork::framework_configuration) / [`set_framework_configuration(Option<NetworkFrameworkConfiguration>)`](crate::client::fluent_builders::CreateNetwork::set_framework_configuration): <p> Configuration properties of the blockchain framework relevant to the network configuration. </p>
    ///   - [`voting_policy(VotingPolicy)`](crate::client::fluent_builders::CreateNetwork::voting_policy) / [`set_voting_policy(Option<VotingPolicy>)`](crate::client::fluent_builders::CreateNetwork::set_voting_policy): <p> The voting rules used by the network to determine if a proposal is approved. </p>
    ///   - [`member_configuration(MemberConfiguration)`](crate::client::fluent_builders::CreateNetwork::member_configuration) / [`set_member_configuration(Option<MemberConfiguration>)`](crate::client::fluent_builders::CreateNetwork::set_member_configuration): <p>Configuration properties for the first member within the network.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateNetwork::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateNetwork::set_tags): <p>Tags to assign to the network. Each tag consists of a key and optional value.</p>  <p>When specifying tags during creation, you can specify multiple key-value pairs in a single request, with an overall maximum of 50 tags added to each resource.</p>  <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p>
                            /// - On success, responds with [`CreateNetworkOutput`](crate::output::CreateNetworkOutput) with field(s):
    ///   - [`network_id(Option<String>)`](crate::output::CreateNetworkOutput::network_id): <p>The unique identifier for the network.</p>
    ///   - [`member_id(Option<String>)`](crate::output::CreateNetworkOutput::member_id): <p>The unique identifier for the first member within the network.</p>
                            /// - On failure, responds with [`SdkError<CreateNetworkError>`](crate::error::CreateNetworkError)
    pub fn create_network(&self) -> crate::client::fluent_builders::CreateNetwork {
                                crate::client::fluent_builders::CreateNetwork::new(self.handle.clone())
                            }
}

