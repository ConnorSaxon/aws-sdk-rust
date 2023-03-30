// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAppInstanceUserEndpoint`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_instance_user_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::app_instance_user_arn) / [`set_app_instance_user_arn(Option<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::set_app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`endpoint_id(impl Into<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::endpoint_id) / [`set_endpoint_id(Option<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::set_endpoint_id): <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::set_name): <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    ///   - [`allow_messages(AllowMessages)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::allow_messages) / [`set_allow_messages(Option<AllowMessages>)`](crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::set_allow_messages): <p>Boolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
                            /// - On success, responds with [`UpdateAppInstanceUserEndpointOutput`](crate::output::UpdateAppInstanceUserEndpointOutput) with field(s):
    ///   - [`app_instance_user_arn(Option<String>)`](crate::output::UpdateAppInstanceUserEndpointOutput::app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`endpoint_id(Option<String>)`](crate::output::UpdateAppInstanceUserEndpointOutput::endpoint_id): <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
                            /// - On failure, responds with [`SdkError<UpdateAppInstanceUserEndpointError>`](crate::error::UpdateAppInstanceUserEndpointError)
    pub fn update_app_instance_user_endpoint(&self) -> crate::client::fluent_builders::UpdateAppInstanceUserEndpoint {
                                crate::client::fluent_builders::UpdateAppInstanceUserEndpoint::new(self.handle.clone())
                            }
}

