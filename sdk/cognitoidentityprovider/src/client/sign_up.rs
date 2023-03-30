// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SignUp`](crate::client::fluent_builders::SignUp) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_id(impl Into<String>)`](crate::client::fluent_builders::SignUp::client_id) / [`set_client_id(Option<String>)`](crate::client::fluent_builders::SignUp::set_client_id): <p>The ID of the client associated with the user pool.</p>
    ///   - [`secret_hash(impl Into<String>)`](crate::client::fluent_builders::SignUp::secret_hash) / [`set_secret_hash(Option<String>)`](crate::client::fluent_builders::SignUp::set_secret_hash): <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::SignUp::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::SignUp::set_username): <p>The user name of the user you want to register.</p>
    ///   - [`password(impl Into<String>)`](crate::client::fluent_builders::SignUp::password) / [`set_password(Option<String>)`](crate::client::fluent_builders::SignUp::set_password): <p>The password of the user you want to register.</p>
    ///   - [`user_attributes(Vec<AttributeType>)`](crate::client::fluent_builders::SignUp::user_attributes) / [`set_user_attributes(Option<Vec<AttributeType>>)`](crate::client::fluent_builders::SignUp::set_user_attributes): <p>An array of name-value pairs representing user attributes.</p>  <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    ///   - [`validation_data(Vec<AttributeType>)`](crate::client::fluent_builders::SignUp::validation_data) / [`set_validation_data(Option<Vec<AttributeType>>)`](crate::client::fluent_builders::SignUp::set_validation_data): <p>The validation data in the request to register a user.</p>
    ///   - [`analytics_metadata(AnalyticsMetadataType)`](crate::client::fluent_builders::SignUp::analytics_metadata) / [`set_analytics_metadata(Option<AnalyticsMetadataType>)`](crate::client::fluent_builders::SignUp::set_analytics_metadata): <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>SignUp</code> calls.</p>
    ///   - [`user_context_data(UserContextDataType)`](crate::client::fluent_builders::SignUp::user_context_data) / [`set_user_context_data(Option<UserContextDataType>)`](crate::client::fluent_builders::SignUp::set_user_context_data): <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    ///   - [`client_metadata(HashMap<String, String>)`](crate::client::fluent_builders::SignUp::client_metadata) / [`set_client_metadata(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SignUp::set_client_metadata): <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>  <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the SignUp API action, Amazon Cognito invokes any functions that are assigned to the following triggers: <i>pre sign-up</i>, <i>custom message</i>, and <i>post confirmation</i>. When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your SignUp request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>   <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>   <ul>    <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>    <li> <p>Validate the ClientMetadata value.</p> </li>    <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>   </ul>  </note>
                            /// - On success, responds with [`SignUpOutput`](crate::output::SignUpOutput) with field(s):
    ///   - [`user_confirmed(bool)`](crate::output::SignUpOutput::user_confirmed): <p>A response from the server indicating that a user registration has been confirmed.</p>
    ///   - [`code_delivery_details(Option<CodeDeliveryDetailsType>)`](crate::output::SignUpOutput::code_delivery_details): <p>The code delivery details returned by the server response to the user registration request.</p>
    ///   - [`user_sub(Option<String>)`](crate::output::SignUpOutput::user_sub): <p>The UUID of the authenticated user. This isn't the same as <code>username</code>.</p>
                            /// - On failure, responds with [`SdkError<SignUpError>`](crate::error::SignUpError)
    pub fn sign_up(&self) -> crate::client::fluent_builders::SignUp {
                                crate::client::fluent_builders::SignUp::new(self.handle.clone())
                            }
}

