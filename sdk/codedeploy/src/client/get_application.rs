// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetApplication`](crate::client::fluent_builders::GetApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::GetApplication::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::GetApplication::set_application_name): <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
                            /// - On success, responds with [`GetApplicationOutput`](crate::output::GetApplicationOutput) with field(s):
    ///   - [`application(Option<ApplicationInfo>)`](crate::output::GetApplicationOutput::application): <p>Information about the application.</p>
                            /// - On failure, responds with [`SdkError<GetApplicationError>`](crate::error::GetApplicationError)
    pub fn get_application(&self) -> crate::client::fluent_builders::GetApplication {
                                crate::client::fluent_builders::GetApplication::new(self.handle.clone())
                            }
}

