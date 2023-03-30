// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApp`](crate::client::fluent_builders::CreateApp) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`create_application_request(CreateApplicationRequest)`](crate::client::fluent_builders::CreateApp::create_application_request) / [`set_create_application_request(Option<CreateApplicationRequest>)`](crate::client::fluent_builders::CreateApp::set_create_application_request): <p>Specifies the display name of an application and the tags to associate with the application.</p>
                            /// - On success, responds with [`CreateAppOutput`](crate::output::CreateAppOutput) with field(s):
    ///   - [`application_response(Option<ApplicationResponse>)`](crate::output::CreateAppOutput::application_response): <p>Provides information about an application.</p>
                            /// - On failure, responds with [`SdkError<CreateAppError>`](crate::error::CreateAppError)
    pub fn create_app(&self) -> crate::client::fluent_builders::CreateApp {
                                crate::client::fluent_builders::CreateApp::new(self.handle.clone())
                            }
}

