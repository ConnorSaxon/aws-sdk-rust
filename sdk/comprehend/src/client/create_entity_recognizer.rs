// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateEntityRecognizer`](crate::client::fluent_builders::CreateEntityRecognizer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recognizer_name(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::recognizer_name) / [`set_recognizer_name(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_recognizer_name): <p>The name given to the newly created recognizer. Recognizer names can be a maximum of 256 characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The name must be unique in the account/region.</p>
    ///   - [`version_name(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::version_name) / [`set_version_name(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_version_name): <p>The version name given to the newly created recognizer. Version names can be a maximum of 256 characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The version name must be unique among all models with the same recognizer name in the account/ AWS Region.</p>
    ///   - [`data_access_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::data_access_role_arn) / [`set_data_access_role_arn(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_data_access_role_arn): <p>The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateEntityRecognizer::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_tags): <p>Tags to be associated with the entity recognizer being created. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    ///   - [`input_data_config(EntityRecognizerInputDataConfig)`](crate::client::fluent_builders::CreateEntityRecognizer::input_data_config) / [`set_input_data_config(Option<EntityRecognizerInputDataConfig>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_input_data_config): <p>Specifies the format and location of the input data. The S3 bucket containing the input data must be located in the same region as the entity recognizer being created. </p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_client_request_token): <p> A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    ///   - [`language_code(LanguageCode)`](crate::client::fluent_builders::CreateEntityRecognizer::language_code) / [`set_language_code(Option<LanguageCode>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_language_code): <p> You can specify any of the following languages: English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), German ("de"), or Portuguese ("pt"). If you plan to use this entity recognizer with PDF, Word, or image input files, you must specify English as the language. All training documents must be in the same language.</p>
    ///   - [`volume_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::volume_kms_key_id) / [`set_volume_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_volume_kms_key_id): <p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p>  <ul>   <li> <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li>   <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li>  </ul>
    ///   - [`vpc_config(VpcConfig)`](crate::client::fluent_builders::CreateEntityRecognizer::vpc_config) / [`set_vpc_config(Option<VpcConfig>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_vpc_config): <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your custom entity recognizer. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    ///   - [`model_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::model_kms_key_id) / [`set_model_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_model_kms_key_id): <p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt trained custom models. The ModelKmsKeyId can be either of the following formats</p>  <ul>   <li> <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li>   <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li>  </ul>
    ///   - [`model_policy(impl Into<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::model_policy) / [`set_model_policy(Option<String>)`](crate::client::fluent_builders::CreateEntityRecognizer::set_model_policy): <p>The JSON resource-based policy to attach to your custom entity recognizer model. You can use this policy to allow another AWS account to import your custom model.</p>  <p>Provide your JSON as a UTF-8 encoded string without line breaks. To provide valid JSON for your policy, enclose the attribute names and values in double quotes. If the JSON body is also enclosed in double quotes, then you must escape the double quotes that are inside the policy:</p>  <p> <code>"{\"attribute\": \"value\", \"attribute\": [\"value\"]}"</code> </p>  <p>To avoid escaping quotes, you can use single quotes to enclose the policy and double quotes to enclose the JSON names and values:</p>  <p> <code>'{"attribute": "value", "attribute": ["value"]}'</code> </p>
                            /// - On success, responds with [`CreateEntityRecognizerOutput`](crate::output::CreateEntityRecognizerOutput) with field(s):
    ///   - [`entity_recognizer_arn(Option<String>)`](crate::output::CreateEntityRecognizerOutput::entity_recognizer_arn): <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
                            /// - On failure, responds with [`SdkError<CreateEntityRecognizerError>`](crate::error::CreateEntityRecognizerError)
    pub fn create_entity_recognizer(&self) -> crate::client::fluent_builders::CreateEntityRecognizer {
                                crate::client::fluent_builders::CreateEntityRecognizer::new(self.handle.clone())
                            }
}

