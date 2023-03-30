// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateWorkGroup`](crate::client::fluent_builders::CreateWorkGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateWorkGroup::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateWorkGroup::set_name): <p>The workgroup name.</p>
    ///   - [`configuration(WorkGroupConfiguration)`](crate::client::fluent_builders::CreateWorkGroup::configuration) / [`set_configuration(Option<WorkGroupConfiguration>)`](crate::client::fluent_builders::CreateWorkGroup::set_configuration): <p>Contains configuration information for creating an Athena SQL workgroup, which includes the location in Amazon S3 where query results are stored, the encryption configuration, if any, used for encrypting query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup, the limit for the amount of bytes scanned (cutoff) per query, if it is specified, and whether workgroup's settings (specified with <code>EnforceWorkGroupConfiguration</code>) in the <code>WorkGroupConfiguration</code> override client-side settings. See <code>WorkGroupConfiguration$EnforceWorkGroupConfiguration</code>.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateWorkGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateWorkGroup::set_description): <p>The workgroup description.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateWorkGroup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateWorkGroup::set_tags): <p>A list of comma separated tags to add to the workgroup that is created.</p>
                            /// - On success, responds with [`CreateWorkGroupOutput`](crate::output::CreateWorkGroupOutput)
                            /// - On failure, responds with [`SdkError<CreateWorkGroupError>`](crate::error::CreateWorkGroupError)
    pub fn create_work_group(&self) -> crate::client::fluent_builders::CreateWorkGroup {
                                crate::client::fluent_builders::CreateWorkGroup::new(self.handle.clone())
                            }
}

