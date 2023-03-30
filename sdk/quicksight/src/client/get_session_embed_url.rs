// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSessionEmbedUrl`](crate::client::fluent_builders::GetSessionEmbedUrl) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::set_aws_account_id): <p>The ID for the Amazon Web Services account associated with your Amazon QuickSight subscription.</p>
    ///   - [`entry_point(impl Into<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::entry_point) / [`set_entry_point(Option<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::set_entry_point): <p>The URL you use to access the embedded session. The entry point URL is constrained to the following paths:</p>  <ul>   <li> <p> <code>/start</code> </p> </li>   <li> <p> <code>/start/analyses</code> </p> </li>   <li> <p> <code>/start/dashboards</code> </p> </li>   <li> <p> <code>/start/favorites</code> </p> </li>   <li> <p> <code>/dashboards/<i>DashboardId</i> </code> - where <code>DashboardId</code> is the actual ID key from the Amazon QuickSight console URL of the dashboard</p> </li>   <li> <p> <code>/analyses/<i>AnalysisId</i> </code> - where <code>AnalysisId</code> is the actual ID key from the Amazon QuickSight console URL of the analysis</p> </li>  </ul>
    ///   - [`session_lifetime_in_minutes(i64)`](crate::client::fluent_builders::GetSessionEmbedUrl::session_lifetime_in_minutes) / [`set_session_lifetime_in_minutes(Option<i64>)`](crate::client::fluent_builders::GetSessionEmbedUrl::set_session_lifetime_in_minutes): <p>How many minutes the session is valid. The session lifetime must be 15-600 minutes.</p>
    ///   - [`user_arn(impl Into<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::user_arn) / [`set_user_arn(Option<String>)`](crate::client::fluent_builders::GetSessionEmbedUrl::set_user_arn): <p>The Amazon QuickSight user's Amazon Resource Name (ARN), for use with <code>QUICKSIGHT</code> identity type. You can use this for any type of Amazon QuickSight users in your account (readers, authors, or admins). They need to be authenticated as one of the following:</p>  <ol>   <li> <p>Active Directory (AD) users or group members</p> </li>   <li> <p>Invited nonfederated users</p> </li>   <li> <p>Identity and Access Management (IAM) users and IAM role-based sessions authenticated through Federated Single Sign-On using SAML, OpenID Connect, or IAM federation</p> </li>  </ol>  <p>Omit this parameter for users in the third group, IAM users and IAM role-based sessions.</p>
                            /// - On success, responds with [`GetSessionEmbedUrlOutput`](crate::output::GetSessionEmbedUrlOutput) with field(s):
    ///   - [`embed_url(Option<String>)`](crate::output::GetSessionEmbedUrlOutput::embed_url): <p>A single-use URL that you can put into your server-side web page to embed your Amazon QuickSight session. This URL is valid for 5 minutes. The API operation provides the URL with an <code>auth_code</code> value that enables one (and only one) sign-on to a user session that is valid for 10 hours. </p>
    ///   - [`status(i32)`](crate::output::GetSessionEmbedUrlOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::output::GetSessionEmbedUrlOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<GetSessionEmbedUrlError>`](crate::error::GetSessionEmbedUrlError)
    pub fn get_session_embed_url(&self) -> crate::client::fluent_builders::GetSessionEmbedUrl {
                                crate::client::fluent_builders::GetSessionEmbedUrl::new(self.handle.clone())
                            }
}

