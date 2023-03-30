// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutRepositoryTriggers`](crate::client::fluent_builders::PutRepositoryTriggers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::PutRepositoryTriggers::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::PutRepositoryTriggers::set_repository_name): <p>The name of the repository where you want to create or update the trigger.</p>
    ///   - [`triggers(Vec<RepositoryTrigger>)`](crate::client::fluent_builders::PutRepositoryTriggers::triggers) / [`set_triggers(Option<Vec<RepositoryTrigger>>)`](crate::client::fluent_builders::PutRepositoryTriggers::set_triggers): <p>The JSON block of configuration information for each trigger.</p>
                            /// - On success, responds with [`PutRepositoryTriggersOutput`](crate::output::PutRepositoryTriggersOutput) with field(s):
    ///   - [`configuration_id(Option<String>)`](crate::output::PutRepositoryTriggersOutput::configuration_id): <p>The system-generated unique ID for the create or update operation.</p>
                            /// - On failure, responds with [`SdkError<PutRepositoryTriggersError>`](crate::error::PutRepositoryTriggersError)
    pub fn put_repository_triggers(&self) -> crate::client::fluent_builders::PutRepositoryTriggers {
                                crate::client::fluent_builders::PutRepositoryTriggers::new(self.handle.clone())
                            }
}

