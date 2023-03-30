// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartOutboundVoiceContact`](crate::client::fluent_builders::StartOutboundVoiceContact) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_phone_number(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::destination_phone_number) / [`set_destination_phone_number(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_destination_phone_number): <p>The phone number of the customer, in E.164 format.</p>
    ///   - [`contact_flow_id(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::contact_flow_id) / [`set_contact_flow_id(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_contact_flow_id): <p>The identifier of the flow for the outbound call. To see the ContactFlowId in the Amazon Connect console user interface, on the navigation menu go to <b>Routing</b>, <b>Contact Flows</b>. Choose the flow. On the flow page, under the name of the flow, choose <b>Show additional flow information</b>. The ContactFlowId is the last part of the ARN, shown here in bold: </p>  <p>arn:aws:connect:us-west-2:xxxxxxxxxxxx:instance/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/contact-flow/<b>846ec553-a005-41c0-8341-xxxxxxxxxxxx</b> </p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>. The token is valid for 7 days after creation. If a contact is already started, the contact ID is returned. </p>
    ///   - [`source_phone_number(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::source_phone_number) / [`set_source_phone_number(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_source_phone_number): <p>The phone number associated with the Amazon Connect instance, in E.164 format. If you do not specify a source phone number, you must specify a queue.</p>
    ///   - [`queue_id(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::queue_id) / [`set_queue_id(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_queue_id): <p>The queue for the call. If you specify a queue, the phone displayed for caller ID is the phone number specified in the queue. If you do not specify a queue, the queue defined in the flow is used. If you do not specify a queue, you must specify a source phone number.</p>
    ///   - [`attributes(HashMap<String, String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::attributes) / [`set_attributes(Option<HashMap<String, String>>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_attributes): <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in flows just like any other contact attributes.</p>  <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    ///   - [`answer_machine_detection_config(AnswerMachineDetectionConfig)`](crate::client::fluent_builders::StartOutboundVoiceContact::answer_machine_detection_config) / [`set_answer_machine_detection_config(Option<AnswerMachineDetectionConfig>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_answer_machine_detection_config): <p>Configuration of the answering machine detection for this outbound call. </p>
    ///   - [`campaign_id(impl Into<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::campaign_id) / [`set_campaign_id(Option<String>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_campaign_id): <p>The campaign identifier of the outbound communication.</p>
    ///   - [`traffic_type(TrafficType)`](crate::client::fluent_builders::StartOutboundVoiceContact::traffic_type) / [`set_traffic_type(Option<TrafficType>)`](crate::client::fluent_builders::StartOutboundVoiceContact::set_traffic_type): <p>Denotes the class of traffic. Calls with different traffic types are handled differently by Amazon Connect. The default value is <code>GENERAL</code>. Use <code>CAMPAIGN</code> if <code>EnableAnswerMachineDetection</code> is set to <code>true</code>. For all other cases, use <code>GENERAL</code>. </p>
                            /// - On success, responds with [`StartOutboundVoiceContactOutput`](crate::output::StartOutboundVoiceContactOutput) with field(s):
    ///   - [`contact_id(Option<String>)`](crate::output::StartOutboundVoiceContactOutput::contact_id): <p>The identifier of this contact within the Amazon Connect instance.</p>
                            /// - On failure, responds with [`SdkError<StartOutboundVoiceContactError>`](crate::error::StartOutboundVoiceContactError)
    pub fn start_outbound_voice_contact(&self) -> crate::client::fluent_builders::StartOutboundVoiceContact {
                                crate::client::fluent_builders::StartOutboundVoiceContact::new(self.handle.clone())
                            }
}

