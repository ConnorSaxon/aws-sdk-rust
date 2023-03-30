// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateDiscoveredResource`](crate::client::fluent_builders::AssociateDiscoveredResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`progress_update_stream(impl Into<String>)`](crate::client::fluent_builders::AssociateDiscoveredResource::progress_update_stream) / [`set_progress_update_stream(Option<String>)`](crate::client::fluent_builders::AssociateDiscoveredResource::set_progress_update_stream): <p>The name of the ProgressUpdateStream.</p>
    ///   - [`migration_task_name(impl Into<String>)`](crate::client::fluent_builders::AssociateDiscoveredResource::migration_task_name) / [`set_migration_task_name(Option<String>)`](crate::client::fluent_builders::AssociateDiscoveredResource::set_migration_task_name): <p>The identifier given to the MigrationTask. <i>Do not store personal data in this field.</i> </p>
    ///   - [`discovered_resource(DiscoveredResource)`](crate::client::fluent_builders::AssociateDiscoveredResource::discovered_resource) / [`set_discovered_resource(Option<DiscoveredResource>)`](crate::client::fluent_builders::AssociateDiscoveredResource::set_discovered_resource): <p>Object representing a Resource.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AssociateDiscoveredResource::dry_run) / [`set_dry_run(bool)`](crate::client::fluent_builders::AssociateDiscoveredResource::set_dry_run): <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
                            /// - On success, responds with [`AssociateDiscoveredResourceOutput`](crate::output::AssociateDiscoveredResourceOutput)
                            /// - On failure, responds with [`SdkError<AssociateDiscoveredResourceError>`](crate::error::AssociateDiscoveredResourceError)
    pub fn associate_discovered_resource(&self) -> crate::client::fluent_builders::AssociateDiscoveredResource {
                                crate::client::fluent_builders::AssociateDiscoveredResource::new(self.handle.clone())
                            }
}

