// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePushTemplate`](crate::client::fluent_builders::UpdatePushTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`create_new_version(bool)`](crate::client::fluent_builders::UpdatePushTemplate::create_new_version) / [`set_create_new_version(bool)`](crate::client::fluent_builders::UpdatePushTemplate::set_create_new_version): <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to (overwrite) the latest existing version of the template.</p>  <p>If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to (overwrites) the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    ///   - [`push_notification_template_request(PushNotificationTemplateRequest)`](crate::client::fluent_builders::UpdatePushTemplate::push_notification_template_request) / [`set_push_notification_template_request(Option<PushNotificationTemplateRequest>)`](crate::client::fluent_builders::UpdatePushTemplate::set_push_notification_template_request): <p>Specifies the content and settings for a message template that can be used in messages that are sent through a push notification channel.</p>
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::UpdatePushTemplate::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::UpdatePushTemplate::set_template_name): <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    ///   - [`version(impl Into<String>)`](crate::client::fluent_builders::UpdatePushTemplate::version) / [`set_version(Option<String>)`](crate::client::fluent_builders::UpdatePushTemplate::set_version): <p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the   <link linkend="templates-template-name-template-type-versions">Template Versions resource.</p>  <p>If specified, this value must match the identifier for an existing template version. If specified for an update operation, this value must match the identifier for the latest existing version of the template. This restriction helps ensure that race conditions don't occur.</p>  <p>If you don't specify a value for this parameter, Amazon Pinpoint does the following:</p>  <ul>  <li><p>For a get operation, retrieves information about the active version of the template.</p></li>   <li><p>For an update operation, saves the updates to (overwrites) the latest existing version of the template, if the create-new-version parameter isn't used or is set to false.</p></li>   <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li> </ul>
                            /// - On success, responds with [`UpdatePushTemplateOutput`](crate::output::UpdatePushTemplateOutput) with field(s):
    ///   - [`message_body(Option<MessageBody>)`](crate::output::UpdatePushTemplateOutput::message_body): <p>Provides information about an API request or response.</p>
                            /// - On failure, responds with [`SdkError<UpdatePushTemplateError>`](crate::error::UpdatePushTemplateError)
    pub fn update_push_template(&self) -> crate::client::fluent_builders::UpdatePushTemplate {
                                crate::client::fluent_builders::UpdatePushTemplate::new(self.handle.clone())
                            }
}

