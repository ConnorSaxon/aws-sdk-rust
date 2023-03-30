// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAppInstanceUser`](crate::client::fluent_builders::CreateAppInstanceUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_instance_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::app_instance_arn) / [`set_app_instance_arn(Option<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_app_instance_arn): <p>The ARN of the <code>AppInstance</code> request.</p>
    ///   - [`app_instance_user_id(impl Into<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::app_instance_user_id) / [`set_app_instance_user_id(Option<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_app_instance_user_id): <p>The user ID of the <code>AppInstance</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_name): <p>The user's name.</p>
    ///   - [`metadata(impl Into<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::metadata) / [`set_metadata(Option<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_metadata): <p>The request's metadata. Limited to a 1KB string in UTF-8.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_client_request_token): <p>The token assigned to the user requesting an <code>AppInstance</code>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAppInstanceUser::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAppInstanceUser::set_tags): <p>Tags assigned to the <code>AppInstanceUser</code>.</p>
                            /// - On success, responds with [`CreateAppInstanceUserOutput`](crate::output::CreateAppInstanceUserOutput) with field(s):
    ///   - [`app_instance_user_arn(Option<String>)`](crate::output::CreateAppInstanceUserOutput::app_instance_user_arn): <p>The user's ARN.</p>
                            /// - On failure, responds with [`SdkError<CreateAppInstanceUserError>`](crate::error::CreateAppInstanceUserError)
    pub fn create_app_instance_user(&self) -> crate::client::fluent_builders::CreateAppInstanceUser {
                                crate::client::fluent_builders::CreateAppInstanceUser::new(self.handle.clone())
                            }
}

