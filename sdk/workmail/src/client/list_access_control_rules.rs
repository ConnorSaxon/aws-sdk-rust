// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAccessControlRules`](crate::client::fluent_builders::ListAccessControlRules) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::ListAccessControlRules::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::ListAccessControlRules::set_organization_id): <p>The identifier for the organization.</p>
                            /// - On success, responds with [`ListAccessControlRulesOutput`](crate::output::ListAccessControlRulesOutput) with field(s):
    ///   - [`rules(Option<Vec<AccessControlRule>>)`](crate::output::ListAccessControlRulesOutput::rules): <p>The access control rules.</p>
                            /// - On failure, responds with [`SdkError<ListAccessControlRulesError>`](crate::error::ListAccessControlRulesError)
    pub fn list_access_control_rules(&self) -> crate::client::fluent_builders::ListAccessControlRules {
                                crate::client::fluent_builders::ListAccessControlRules::new(self.handle.clone())
                            }
}

