// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApplication`](crate::client::fluent_builders::CreateApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_application_name): <p>The name of the application. Must be unique within your account.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_description): <p>Your description of the application.</p>
    ///   - [`resource_lifecycle_config(ApplicationResourceLifecycleConfig)`](crate::client::fluent_builders::CreateApplication::resource_lifecycle_config) / [`set_resource_lifecycle_config(Option<ApplicationResourceLifecycleConfig>)`](crate::client::fluent_builders::CreateApplication::set_resource_lifecycle_config): <p>Specifies an application resource lifecycle configuration to prevent your application from accumulating too many versions.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateApplication::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateApplication::set_tags): <p>Specifies the tags applied to the application.</p>  <p>Elastic Beanstalk applies these tags only to the application. Environments that you create in the application don't inherit the tags.</p>
                            /// - On success, responds with [`CreateApplicationOutput`](crate::output::CreateApplicationOutput) with field(s):
    ///   - [`application(Option<ApplicationDescription>)`](crate::output::CreateApplicationOutput::application): <p> The <code>ApplicationDescription</code> of the application. </p>
                            /// - On failure, responds with [`SdkError<CreateApplicationError>`](crate::error::CreateApplicationError)
    pub fn create_application(&self) -> crate::client::fluent_builders::CreateApplication {
                                crate::client::fluent_builders::CreateApplication::new(self.handle.clone())
                            }
}

