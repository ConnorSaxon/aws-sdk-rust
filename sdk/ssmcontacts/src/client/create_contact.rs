// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateContact`](crate::client::fluent_builders::CreateContact) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alias(impl Into<String>)`](crate::client::fluent_builders::CreateContact::alias) / [`set_alias(Option<String>)`](crate::client::fluent_builders::CreateContact::set_alias): <p>The short name to quickly identify a contact or escalation plan. The contact alias must be unique and identifiable. </p>
    ///   - [`display_name(impl Into<String>)`](crate::client::fluent_builders::CreateContact::display_name) / [`set_display_name(Option<String>)`](crate::client::fluent_builders::CreateContact::set_display_name): <p>The full name of the contact or escalation plan. </p>
    ///   - [`r#type(ContactType)`](crate::client::fluent_builders::CreateContact::type) / [`set_type(Option<ContactType>)`](crate::client::fluent_builders::CreateContact::set_type): <p>To create an escalation plan use <code>ESCALATION</code>. To create a contact use <code>PERSONAL</code>.</p>
    ///   - [`plan(Plan)`](crate::client::fluent_builders::CreateContact::plan) / [`set_plan(Option<Plan>)`](crate::client::fluent_builders::CreateContact::set_plan): <p>A list of stages. A contact has an engagement plan with stages that contact specified contact channels. An escalation plan uses stages that contact specified contacts. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateContact::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateContact::set_tags): <p>Adds a tag to the target. You can only tag resources created in the first Region of your replication set. </p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::CreateContact::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::CreateContact::set_idempotency_token): <p>A token ensuring that the operation is called only once with the specified details.</p>
                            /// - On success, responds with [`CreateContactOutput`](crate::output::CreateContactOutput) with field(s):
    ///   - [`contact_arn(Option<String>)`](crate::output::CreateContactOutput::contact_arn): <p>The Amazon Resource Name (ARN) of the created contact or escalation plan.</p>
                            /// - On failure, responds with [`SdkError<CreateContactError>`](crate::error::CreateContactError)
    pub fn create_contact(&self) -> crate::client::fluent_builders::CreateContact {
                                crate::client::fluent_builders::CreateContact::new(self.handle.clone())
                            }
}

