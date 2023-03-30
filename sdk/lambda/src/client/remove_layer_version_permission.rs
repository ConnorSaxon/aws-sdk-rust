// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveLayerVersionPermission`](crate::client::fluent_builders::RemoveLayerVersionPermission) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`layer_name(impl Into<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::layer_name) / [`set_layer_name(Option<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::set_layer_name): <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    ///   - [`version_number(i64)`](crate::client::fluent_builders::RemoveLayerVersionPermission::version_number) / [`set_version_number(i64)`](crate::client::fluent_builders::RemoveLayerVersionPermission::set_version_number): <p>The version number.</p>
    ///   - [`statement_id(impl Into<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::statement_id) / [`set_statement_id(Option<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::set_statement_id): <p>The identifier that was specified when the statement was added.</p>
    ///   - [`revision_id(impl Into<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::revision_id) / [`set_revision_id(Option<String>)`](crate::client::fluent_builders::RemoveLayerVersionPermission::set_revision_id): <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
                            /// - On success, responds with [`RemoveLayerVersionPermissionOutput`](crate::output::RemoveLayerVersionPermissionOutput)
                            /// - On failure, responds with [`SdkError<RemoveLayerVersionPermissionError>`](crate::error::RemoveLayerVersionPermissionError)
    pub fn remove_layer_version_permission(&self) -> crate::client::fluent_builders::RemoveLayerVersionPermission {
                                crate::client::fluent_builders::RemoveLayerVersionPermission::new(self.handle.clone())
                            }
}

