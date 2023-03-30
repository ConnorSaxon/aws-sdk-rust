// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePackagingGroup`](crate::client::fluent_builders::DeletePackagingGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeletePackagingGroup::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeletePackagingGroup::set_id): The ID of the MediaPackage VOD PackagingGroup resource to delete.
                            /// - On success, responds with [`DeletePackagingGroupOutput`](crate::output::DeletePackagingGroupOutput)
                            /// - On failure, responds with [`SdkError<DeletePackagingGroupError>`](crate::error::DeletePackagingGroupError)
    pub fn delete_packaging_group(&self) -> crate::client::fluent_builders::DeletePackagingGroup {
                                crate::client::fluent_builders::DeletePackagingGroup::new(self.handle.clone())
                            }
}

