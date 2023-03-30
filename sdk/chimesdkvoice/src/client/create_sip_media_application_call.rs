// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSipMediaApplicationCall`](crate::client::fluent_builders::CreateSipMediaApplicationCall) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`from_phone_number(impl Into<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::from_phone_number) / [`set_from_phone_number(Option<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::set_from_phone_number): (undocumented)
    ///   - [`to_phone_number(impl Into<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::to_phone_number) / [`set_to_phone_number(Option<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::set_to_phone_number): (undocumented)
    ///   - [`sip_media_application_id(impl Into<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::sip_media_application_id) / [`set_sip_media_application_id(Option<String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::set_sip_media_application_id): (undocumented)
    ///   - [`sip_headers(HashMap<String, String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::sip_headers) / [`set_sip_headers(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::set_sip_headers): (undocumented)
    ///   - [`arguments_map(HashMap<String, String>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::arguments_map) / [`set_arguments_map(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateSipMediaApplicationCall::set_arguments_map): (undocumented)
                            /// - On success, responds with [`CreateSipMediaApplicationCallOutput`](crate::output::CreateSipMediaApplicationCallOutput) with field(s):
    ///   - [`sip_media_application_call(Option<SipMediaApplicationCall>)`](crate::output::CreateSipMediaApplicationCallOutput::sip_media_application_call): (undocumented)
                            /// - On failure, responds with [`SdkError<CreateSipMediaApplicationCallError>`](crate::error::CreateSipMediaApplicationCallError)
    pub fn create_sip_media_application_call(&self) -> crate::client::fluent_builders::CreateSipMediaApplicationCall {
                                crate::client::fluent_builders::CreateSipMediaApplicationCall::new(self.handle.clone())
                            }
}

