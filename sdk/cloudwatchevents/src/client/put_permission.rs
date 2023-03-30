// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutPermission`](crate::client::fluent_builders::PutPermission) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_bus_name(impl Into<String>)`](crate::client::fluent_builders::PutPermission::event_bus_name) / [`set_event_bus_name(Option<String>)`](crate::client::fluent_builders::PutPermission::set_event_bus_name): <p>The name of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    ///   - [`action(impl Into<String>)`](crate::client::fluent_builders::PutPermission::action) / [`set_action(Option<String>)`](crate::client::fluent_builders::PutPermission::set_action): <p>The action that you are enabling the other account to perform.</p>
    ///   - [`principal(impl Into<String>)`](crate::client::fluent_builders::PutPermission::principal) / [`set_principal(Option<String>)`](crate::client::fluent_builders::PutPermission::set_principal): <p>The 12-digit Amazon Web Services account ID that you are permitting to put events to your default event bus. Specify "*" to permit any account to put events to your default event bus.</p>  <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that may match undesirable events. To create more secure rules, make sure that the event pattern for each rule contains an <code>account</code> field with a specific account ID from which to receive events. Rules with an account field do not match any events sent from other accounts.</p>
    ///   - [`statement_id(impl Into<String>)`](crate::client::fluent_builders::PutPermission::statement_id) / [`set_statement_id(Option<String>)`](crate::client::fluent_builders::PutPermission::set_statement_id): <p>An identifier string for the external account that you are granting permissions to. If you later want to revoke the permission for this external account, specify this <code>StatementId</code> when you run <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_RemovePermission.html">RemovePermission</a>.</p>
    ///   - [`condition(Condition)`](crate::client::fluent_builders::PutPermission::condition) / [`set_condition(Option<Condition>)`](crate::client::fluent_builders::PutPermission::set_condition): <p>This parameter enables you to limit the permission to accounts that fulfill a certain condition, such as being a member of a certain Amazon Web Services organization. For more information about Amazon Web Services Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is Amazon Web Services Organizations</a> in the <i>Amazon Web Services Organizations User Guide</i>.</p>  <p>If you specify <code>Condition</code> with an Amazon Web Services organization ID, and specify "*" as the value for <code>Principal</code>, you grant permission to all the accounts in the named organization.</p>  <p>The <code>Condition</code> is a JSON string which must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields.</p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::PutPermission::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::PutPermission::set_policy): <p>A JSON string that describes the permission policy statement. You can include a <code>Policy</code> parameter in the request instead of using the <code>StatementId</code>, <code>Action</code>, <code>Principal</code>, or <code>Condition</code> parameters.</p>
                            /// - On success, responds with [`PutPermissionOutput`](crate::output::PutPermissionOutput)
                            /// - On failure, responds with [`SdkError<PutPermissionError>`](crate::error::PutPermissionError)
    pub fn put_permission(&self) -> crate::client::fluent_builders::PutPermission {
                                crate::client::fluent_builders::PutPermission::new(self.handle.clone())
                            }
}

