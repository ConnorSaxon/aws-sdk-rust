// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateInputSecurityGroup`](crate::client::fluent_builders::CreateInputSecurityGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateInputSecurityGroup::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateInputSecurityGroup::set_tags): A collection of key-value pairs.
    ///   - [`whitelist_rules(Vec<InputWhitelistRuleCidr>)`](crate::client::fluent_builders::CreateInputSecurityGroup::whitelist_rules) / [`set_whitelist_rules(Option<Vec<InputWhitelistRuleCidr>>)`](crate::client::fluent_builders::CreateInputSecurityGroup::set_whitelist_rules): List of IPv4 CIDR addresses to whitelist
                            /// - On success, responds with [`CreateInputSecurityGroupOutput`](crate::output::CreateInputSecurityGroupOutput) with field(s):
    ///   - [`security_group(Option<InputSecurityGroup>)`](crate::output::CreateInputSecurityGroupOutput::security_group): An Input Security Group
                            /// - On failure, responds with [`SdkError<CreateInputSecurityGroupError>`](crate::error::CreateInputSecurityGroupError)
    pub fn create_input_security_group(&self) -> crate::client::fluent_builders::CreateInputSecurityGroup {
                                crate::client::fluent_builders::CreateInputSecurityGroup::new(self.handle.clone())
                            }
}

