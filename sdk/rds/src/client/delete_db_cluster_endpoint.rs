// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDBClusterEndpoint`](crate::client::fluent_builders::DeleteDBClusterEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_cluster_endpoint_identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteDBClusterEndpoint::db_cluster_endpoint_identifier) / [`set_db_cluster_endpoint_identifier(Option<String>)`](crate::client::fluent_builders::DeleteDBClusterEndpoint::set_db_cluster_endpoint_identifier): <p>The identifier associated with the custom endpoint. This parameter is stored as a lowercase string.</p>
                            /// - On success, responds with [`DeleteDbClusterEndpointOutput`](crate::output::DeleteDbClusterEndpointOutput) with field(s):
    ///   - [`db_cluster_endpoint_identifier(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::db_cluster_endpoint_identifier): <p>The identifier associated with the endpoint. This parameter is stored as a lowercase string.</p>
    ///   - [`db_cluster_identifier(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::db_cluster_identifier): <p>The DB cluster identifier of the DB cluster associated with the endpoint. This parameter is stored as a lowercase string.</p>
    ///   - [`db_cluster_endpoint_resource_identifier(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::db_cluster_endpoint_resource_identifier): <p>A unique system-generated identifier for an endpoint. It remains the same for the whole life of the endpoint.</p>
    ///   - [`endpoint(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::endpoint): <p>The DNS address of the endpoint.</p>
    ///   - [`status(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::status): <p>The current status of the endpoint. One of: <code>creating</code>, <code>available</code>, <code>deleting</code>, <code>inactive</code>, <code>modifying</code>. The <code>inactive</code> state applies to an endpoint that can't be used for a certain kind of cluster, such as a <code>writer</code> endpoint for a read-only secondary cluster in a global database.</p>
    ///   - [`endpoint_type(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::endpoint_type): <p>The type of the endpoint. One of: <code>READER</code>, <code>WRITER</code>, <code>CUSTOM</code>.</p>
    ///   - [`custom_endpoint_type(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::custom_endpoint_type): <p>The type associated with a custom endpoint. One of: <code>READER</code>, <code>WRITER</code>, <code>ANY</code>.</p>
    ///   - [`static_members(Option<Vec<String>>)`](crate::output::DeleteDbClusterEndpointOutput::static_members): <p>List of DB instance identifiers that are part of the custom endpoint group.</p>
    ///   - [`excluded_members(Option<Vec<String>>)`](crate::output::DeleteDbClusterEndpointOutput::excluded_members): <p>List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.</p>
    ///   - [`db_cluster_endpoint_arn(Option<String>)`](crate::output::DeleteDbClusterEndpointOutput::db_cluster_endpoint_arn): <p>The Amazon Resource Name (ARN) for the endpoint.</p>
                            /// - On failure, responds with [`SdkError<DeleteDBClusterEndpointError>`](crate::error::DeleteDBClusterEndpointError)
    pub fn delete_db_cluster_endpoint(&self) -> crate::client::fluent_builders::DeleteDBClusterEndpoint {
                                crate::client::fluent_builders::DeleteDBClusterEndpoint::new(self.handle.clone())
                            }
}

