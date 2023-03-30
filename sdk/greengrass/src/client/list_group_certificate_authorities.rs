// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListGroupCertificateAuthorities`](crate::client::fluent_builders::ListGroupCertificateAuthorities) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group_id(impl Into<String>)`](crate::client::fluent_builders::ListGroupCertificateAuthorities::group_id) / [`set_group_id(Option<String>)`](crate::client::fluent_builders::ListGroupCertificateAuthorities::set_group_id): The ID of the Greengrass group.
                            /// - On success, responds with [`ListGroupCertificateAuthoritiesOutput`](crate::output::ListGroupCertificateAuthoritiesOutput) with field(s):
    ///   - [`group_certificate_authorities(Option<Vec<GroupCertificateAuthorityProperties>>)`](crate::output::ListGroupCertificateAuthoritiesOutput::group_certificate_authorities): A list of certificate authorities associated with the group.
                            /// - On failure, responds with [`SdkError<ListGroupCertificateAuthoritiesError>`](crate::error::ListGroupCertificateAuthoritiesError)
    pub fn list_group_certificate_authorities(&self) -> crate::client::fluent_builders::ListGroupCertificateAuthorities {
                                crate::client::fluent_builders::ListGroupCertificateAuthorities::new(self.handle.clone())
                            }
}

