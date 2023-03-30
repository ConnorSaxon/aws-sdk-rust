// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRunGroup`](crate::client::fluent_builders::UpdateRunGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateRunGroup::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateRunGroup::set_id): <p>The group's ID.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateRunGroup::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateRunGroup::set_name): <p>A name for the group.</p>
    ///   - [`max_cpus(i32)`](crate::client::fluent_builders::UpdateRunGroup::max_cpus) / [`set_max_cpus(Option<i32>)`](crate::client::fluent_builders::UpdateRunGroup::set_max_cpus): <p>The maximum number of CPUs to use.</p>
    ///   - [`max_runs(i32)`](crate::client::fluent_builders::UpdateRunGroup::max_runs) / [`set_max_runs(Option<i32>)`](crate::client::fluent_builders::UpdateRunGroup::set_max_runs): <p>The maximum number of concurrent runs for the group.</p>
    ///   - [`max_duration(i32)`](crate::client::fluent_builders::UpdateRunGroup::max_duration) / [`set_max_duration(Option<i32>)`](crate::client::fluent_builders::UpdateRunGroup::set_max_duration): <p>The maximum amount of time to run.</p>
                            /// - On success, responds with [`UpdateRunGroupOutput`](crate::output::UpdateRunGroupOutput)
                            /// - On failure, responds with [`SdkError<UpdateRunGroupError>`](crate::error::UpdateRunGroupError)
    pub fn update_run_group(&self) -> crate::client::fluent_builders::UpdateRunGroup {
                                crate::client::fluent_builders::UpdateRunGroup::new(self.handle.clone())
                            }
}

