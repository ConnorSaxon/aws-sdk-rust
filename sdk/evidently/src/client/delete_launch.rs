// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteLaunch`](crate::client::fluent_builders::DeleteLaunch) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project(impl Into<String>)`](crate::client::fluent_builders::DeleteLaunch::project) / [`set_project(Option<String>)`](crate::client::fluent_builders::DeleteLaunch::set_project): <p>The name or ARN of the project that contains the launch to delete.</p>
    ///   - [`launch(impl Into<String>)`](crate::client::fluent_builders::DeleteLaunch::launch) / [`set_launch(Option<String>)`](crate::client::fluent_builders::DeleteLaunch::set_launch): <p>The name of the launch to delete.</p>
                            /// - On success, responds with [`DeleteLaunchOutput`](crate::output::DeleteLaunchOutput)
                            /// - On failure, responds with [`SdkError<DeleteLaunchError>`](crate::error::DeleteLaunchError)
    pub fn delete_launch(&self) -> crate::client::fluent_builders::DeleteLaunch {
                                crate::client::fluent_builders::DeleteLaunch::new(self.handle.clone())
                            }
}

