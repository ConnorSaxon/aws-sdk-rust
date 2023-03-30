// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSite`](crate::client::fluent_builders::DeleteSite) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::DeleteSite::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::DeleteSite::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`site_id(impl Into<String>)`](crate::client::fluent_builders::DeleteSite::site_id) / [`set_site_id(Option<String>)`](crate::client::fluent_builders::DeleteSite::set_site_id): <p>The ID of the site.</p>
                            /// - On success, responds with [`DeleteSiteOutput`](crate::output::DeleteSiteOutput) with field(s):
    ///   - [`site(Option<Site>)`](crate::output::DeleteSiteOutput::site): <p>Information about the site.</p>
                            /// - On failure, responds with [`SdkError<DeleteSiteError>`](crate::error::DeleteSiteError)
    pub fn delete_site(&self) -> crate::client::fluent_builders::DeleteSite {
                                crate::client::fluent_builders::DeleteSite::new(self.handle.clone())
                            }
}

