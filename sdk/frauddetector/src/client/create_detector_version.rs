// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDetectorVersion`](crate::client::fluent_builders::CreateDetectorVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::CreateDetectorVersion::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::CreateDetectorVersion::set_detector_id): <p>The ID of the detector under which you want to create a new version.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateDetectorVersion::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateDetectorVersion::set_description): <p>The description of the detector version.</p>
    ///   - [`external_model_endpoints(Vec<String>)`](crate::client::fluent_builders::CreateDetectorVersion::external_model_endpoints) / [`set_external_model_endpoints(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDetectorVersion::set_external_model_endpoints): <p>The Amazon Sagemaker model endpoints to include in the detector version.</p>
    ///   - [`rules(Vec<Rule>)`](crate::client::fluent_builders::CreateDetectorVersion::rules) / [`set_rules(Option<Vec<Rule>>)`](crate::client::fluent_builders::CreateDetectorVersion::set_rules): <p>The rules to include in the detector version.</p>
    ///   - [`model_versions(Vec<ModelVersion>)`](crate::client::fluent_builders::CreateDetectorVersion::model_versions) / [`set_model_versions(Option<Vec<ModelVersion>>)`](crate::client::fluent_builders::CreateDetectorVersion::set_model_versions): <p>The model versions to include in the detector version.</p>
    ///   - [`rule_execution_mode(RuleExecutionMode)`](crate::client::fluent_builders::CreateDetectorVersion::rule_execution_mode) / [`set_rule_execution_mode(Option<RuleExecutionMode>)`](crate::client::fluent_builders::CreateDetectorVersion::set_rule_execution_mode): <p>The rule execution mode for the rules included in the detector version.</p>  <p>You can define and edit the rule mode at the detector version level, when it is in draft status.</p>  <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>  <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. </p>  <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateDetectorVersion::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateDetectorVersion::set_tags): <p>A collection of key and value pairs.</p>
                            /// - On success, responds with [`CreateDetectorVersionOutput`](crate::output::CreateDetectorVersionOutput) with field(s):
    ///   - [`detector_id(Option<String>)`](crate::output::CreateDetectorVersionOutput::detector_id): <p>The ID for the created version's parent detector.</p>
    ///   - [`detector_version_id(Option<String>)`](crate::output::CreateDetectorVersionOutput::detector_version_id): <p>The ID for the created detector. </p>
    ///   - [`status(Option<DetectorVersionStatus>)`](crate::output::CreateDetectorVersionOutput::status): <p>The status of the detector version.</p>
                            /// - On failure, responds with [`SdkError<CreateDetectorVersionError>`](crate::error::CreateDetectorVersionError)
    pub fn create_detector_version(&self) -> crate::client::fluent_builders::CreateDetectorVersion {
                                crate::client::fluent_builders::CreateDetectorVersion::new(self.handle.clone())
                            }
}

