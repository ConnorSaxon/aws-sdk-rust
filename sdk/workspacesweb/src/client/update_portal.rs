// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePortal`](crate::client::fluent_builders::UpdatePortal) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_arn(impl Into<String>)`](crate::client::fluent_builders::UpdatePortal::portal_arn) / [`set_portal_arn(Option<String>)`](crate::client::fluent_builders::UpdatePortal::set_portal_arn): <p>The ARN of the web portal.</p>
    ///   - [`display_name(impl Into<String>)`](crate::client::fluent_builders::UpdatePortal::display_name) / [`set_display_name(Option<String>)`](crate::client::fluent_builders::UpdatePortal::set_display_name): <p>The name of the web portal. This is not visible to users who log into the web portal.</p>
    ///   - [`authentication_type(AuthenticationType)`](crate::client::fluent_builders::UpdatePortal::authentication_type) / [`set_authentication_type(Option<AuthenticationType>)`](crate::client::fluent_builders::UpdatePortal::set_authentication_type): <p>The type of authentication integration points used when signing into the web portal. Defaults to <code>Standard</code>.</p>  <p> <code>Standard</code> web portals are authenticated directly through your identity provider. You need to call <code>CreateIdentityProvider</code> to integrate your identity provider with your web portal. User and group access to your web portal is controlled through your identity provider.</p>  <p> <code>IAM_Identity_Center</code> web portals are authenticated through AWS IAM Identity Center (successor to AWS Single Sign-On). They provide additional features, such as IdP-initiated authentication. Identity sources (including external identity provider integration), plus user and group access to your web portal, can be configured in the IAM Identity Center.</p>
                            /// - On success, responds with [`UpdatePortalOutput`](crate::output::UpdatePortalOutput) with field(s):
    ///   - [`portal(Option<Portal>)`](crate::output::UpdatePortalOutput::portal): <p>The web portal.</p>
                            /// - On failure, responds with [`SdkError<UpdatePortalError>`](crate::error::UpdatePortalError)
    pub fn update_portal(&self) -> crate::client::fluent_builders::UpdatePortal {
                                crate::client::fluent_builders::UpdatePortal::new(self.handle.clone())
                            }
}

