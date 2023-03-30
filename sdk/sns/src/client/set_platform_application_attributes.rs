// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SetPlatformApplicationAttributes`](crate::client::fluent_builders::SetPlatformApplicationAttributes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`platform_application_arn(impl Into<String>)`](crate::client::fluent_builders::SetPlatformApplicationAttributes::platform_application_arn) / [`set_platform_application_arn(Option<String>)`](crate::client::fluent_builders::SetPlatformApplicationAttributes::set_platform_application_arn): <p>PlatformApplicationArn for SetPlatformApplicationAttributes action.</p>
    ///   - [`attributes(HashMap<String, String>)`](crate::client::fluent_builders::SetPlatformApplicationAttributes::attributes) / [`set_attributes(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SetPlatformApplicationAttributes::set_attributes): <p>A map of the platform application attributes. Attributes in this map include the following:</p>  <ul>   <li> <p> <code>PlatformCredential</code> – The credential received from the notification service.</p>    <ul>     <li> <p>For ADM, <code>PlatformCredential</code>is client secret.</p> </li>     <li> <p>For Apple Services using certificate credentials, <code>PlatformCredential</code> is private key.</p> </li>     <li> <p>For Apple Services using token credentials, <code>PlatformCredential</code> is signing key.</p> </li>     <li> <p>For GCM (Firebase Cloud Messaging), <code>PlatformCredential</code> is API key. </p> </li>    </ul> </li>  </ul>  <ul>   <li> <p> <code>PlatformPrincipal</code> – The principal received from the notification service.</p>    <ul>     <li> <p>For ADM, <code>PlatformPrincipal</code>is client id.</p> </li>     <li> <p>For Apple Services using certificate credentials, <code>PlatformPrincipal</code> is SSL certificate.</p> </li>     <li> <p>For Apple Services using token credentials, <code>PlatformPrincipal</code> is signing key ID.</p> </li>     <li> <p>For GCM (Firebase Cloud Messaging), there is no <code>PlatformPrincipal</code>. </p> </li>    </ul> </li>  </ul>  <ul>   <li> <p> <code>EventEndpointCreated</code> – Topic ARN to which <code>EndpointCreated</code> event notifications are sent.</p> </li>   <li> <p> <code>EventEndpointDeleted</code> – Topic ARN to which <code>EndpointDeleted</code> event notifications are sent.</p> </li>   <li> <p> <code>EventEndpointUpdated</code> – Topic ARN to which <code>EndpointUpdate</code> event notifications are sent.</p> </li>   <li> <p> <code>EventDeliveryFailure</code> – Topic ARN to which <code>DeliveryFailure</code> event notifications are sent upon Direct Publish delivery failure (permanent) to one of the application's endpoints.</p> </li>   <li> <p> <code>SuccessFeedbackRoleArn</code> – IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li>   <li> <p> <code>FailureFeedbackRoleArn</code> – IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li>   <li> <p> <code>SuccessFeedbackSampleRate</code> – Sample rate percentage (0-100) of successfully delivered messages.</p> </li>  </ul>  <p>The following attributes only apply to <code>APNs</code> token-based authentication:</p>  <ul>   <li> <p> <code>ApplePlatformTeamID</code> – The identifier that's assigned to your Apple developer account team.</p> </li>   <li> <p> <code>ApplePlatformBundleID</code> – The bundle identifier that's assigned to your iOS app.</p> </li>  </ul>
                            /// - On success, responds with [`SetPlatformApplicationAttributesOutput`](crate::output::SetPlatformApplicationAttributesOutput)
                            /// - On failure, responds with [`SdkError<SetPlatformApplicationAttributesError>`](crate::error::SetPlatformApplicationAttributesError)
    pub fn set_platform_application_attributes(&self) -> crate::client::fluent_builders::SetPlatformApplicationAttributes {
                                crate::client::fluent_builders::SetPlatformApplicationAttributes::new(self.handle.clone())
                            }
}

