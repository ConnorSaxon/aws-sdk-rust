// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLDAPSSettings`](crate::client::fluent_builders::DescribeLDAPSSettings) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeLDAPSSettings::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::client::fluent_builders::DescribeLDAPSSettings::directory_id) / [`set_directory_id(Option<String>)`](crate::client::fluent_builders::DescribeLDAPSSettings::set_directory_id): <p>The identifier of the directory.</p>
    ///   - [`r#type(LdapsType)`](crate::client::fluent_builders::DescribeLDAPSSettings::type) / [`set_type(Option<LdapsType>)`](crate::client::fluent_builders::DescribeLDAPSSettings::set_type): <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeLDAPSSettings::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeLDAPSSettings::set_next_token): <p>The type of next token used for pagination.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeLDAPSSettings::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeLDAPSSettings::set_limit): <p>Specifies the number of items that should be displayed on one page.</p>
                            /// - On success, responds with [`DescribeLdapsSettingsOutput`](crate::output::DescribeLdapsSettingsOutput) with field(s):
    ///   - [`ldaps_settings_info(Option<Vec<LdapsSettingInfo>>)`](crate::output::DescribeLdapsSettingsOutput::ldaps_settings_info): <p>Information about LDAP security for the specified directory, including status of enablement, state last updated date time, and the reason for the state.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeLdapsSettingsOutput::next_token): <p>The next token used to retrieve the LDAPS settings if the number of setting types exceeds page limit and there is another page.</p>
                            /// - On failure, responds with [`SdkError<DescribeLDAPSSettingsError>`](crate::error::DescribeLDAPSSettingsError)
    pub fn describe_ldaps_settings(&self) -> crate::client::fluent_builders::DescribeLDAPSSettings {
                                crate::client::fluent_builders::DescribeLDAPSSettings::new(self.handle.clone())
                            }
}

