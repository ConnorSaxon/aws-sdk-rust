// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApplication`](crate::client::fluent_builders::CreateApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_name): <p>The name of the application. The name must be unique in the region in which you are creating the application.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_description): <p>The description of the application.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateApplication::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateApplication::set_tags): <p>Key-value pairs you can use to associate with the application.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_client_token): <p>A unique identifier that you provide to ensure idempotency. If you retry a request that completed successfully using the same client token and the same parameters, the retry succeeds without performing any further actions. If you retry a successful request using the same client token, but one or more of the parameters are different, the retry fails.</p>
                            /// - On success, responds with [`CreateApplicationOutput`](crate::output::CreateApplicationOutput) with field(s):
    ///   - [`application(Option<Application>)`](crate::output::CreateApplicationOutput::application): <p>Information about the application.</p>
                            /// - On failure, responds with [`SdkError<CreateApplicationError>`](crate::error::CreateApplicationError)
    pub fn create_application(&self) -> crate::client::fluent_builders::CreateApplication {
                                crate::client::fluent_builders::CreateApplication::new(self.handle.clone())
                            }
}

