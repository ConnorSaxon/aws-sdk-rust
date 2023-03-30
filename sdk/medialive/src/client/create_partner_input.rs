// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreatePartnerInput`](crate::client::fluent_builders::CreatePartnerInput) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`input_id(impl Into<String>)`](crate::client::fluent_builders::CreatePartnerInput::input_id) / [`set_input_id(Option<String>)`](crate::client::fluent_builders::CreatePartnerInput::set_input_id): Unique ID of the input.
    ///   - [`request_id(impl Into<String>)`](crate::client::fluent_builders::CreatePartnerInput::request_id) / [`set_request_id(Option<String>)`](crate::client::fluent_builders::CreatePartnerInput::set_request_id): Unique identifier of the request to ensure the request is handled exactly once in case of retries.
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreatePartnerInput::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreatePartnerInput::set_tags): A collection of key-value pairs.
                            /// - On success, responds with [`CreatePartnerInputOutput`](crate::output::CreatePartnerInputOutput) with field(s):
    ///   - [`input(Option<Input>)`](crate::output::CreatePartnerInputOutput::input): Placeholder documentation for Input
                            /// - On failure, responds with [`SdkError<CreatePartnerInputError>`](crate::error::CreatePartnerInputError)
    pub fn create_partner_input(&self) -> crate::client::fluent_builders::CreatePartnerInput {
                                crate::client::fluent_builders::CreatePartnerInput::new(self.handle.clone())
                            }
}

