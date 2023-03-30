// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateBrowserSettings`](crate::client::fluent_builders::UpdateBrowserSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`browser_settings_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::browser_settings_arn) / [`set_browser_settings_arn(Option<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::set_browser_settings_arn): <p>The ARN of the browser settings.</p>
    ///   - [`browser_policy(impl Into<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::browser_policy) / [`set_browser_policy(Option<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::set_browser_policy): <p>A JSON string containing Chrome Enterprise policies that will be applied to all streaming sessions. </p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateBrowserSettings::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, subsequent retries with the same client token return the result from the original successful request. </p>  <p>If you do not specify a client token, one is automatically generated by the AWS SDK.</p>
                            /// - On success, responds with [`UpdateBrowserSettingsOutput`](crate::output::UpdateBrowserSettingsOutput) with field(s):
    ///   - [`browser_settings(Option<BrowserSettings>)`](crate::output::UpdateBrowserSettingsOutput::browser_settings): <p>The browser settings.</p>
                            /// - On failure, responds with [`SdkError<UpdateBrowserSettingsError>`](crate::error::UpdateBrowserSettingsError)
    pub fn update_browser_settings(&self) -> crate::client::fluent_builders::UpdateBrowserSettings {
                                crate::client::fluent_builders::UpdateBrowserSettings::new(self.handle.clone())
                            }
}

