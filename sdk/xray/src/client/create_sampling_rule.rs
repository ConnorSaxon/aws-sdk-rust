// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSamplingRule`](crate::client::fluent_builders::CreateSamplingRule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`sampling_rule(SamplingRule)`](crate::client::fluent_builders::CreateSamplingRule::sampling_rule) / [`set_sampling_rule(Option<SamplingRule>)`](crate::client::fluent_builders::CreateSamplingRule::set_sampling_rule): <p>The rule definition.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateSamplingRule::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateSamplingRule::set_tags): <p>A map that contains one or more tag keys and tag values to attach to an X-Ray sampling rule. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference</i>.</p>  <p>The following restrictions apply to tags:</p>  <ul>   <li> <p>Maximum number of user-applied tags per resource: 50</p> </li>   <li> <p>Maximum tag key length: 128 Unicode characters</p> </li>   <li> <p>Maximum tag value length: 256 Unicode characters</p> </li>   <li> <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . : / = + - and @</p> </li>   <li> <p>Tag keys and values are case sensitive.</p> </li>   <li> <p>Don't use <code>aws:</code> as a prefix for keys; it's reserved for Amazon Web Services use.</p> </li>  </ul>
                            /// - On success, responds with [`CreateSamplingRuleOutput`](crate::output::CreateSamplingRuleOutput) with field(s):
    ///   - [`sampling_rule_record(Option<SamplingRuleRecord>)`](crate::output::CreateSamplingRuleOutput::sampling_rule_record): <p>The saved rule definition and metadata.</p>
                            /// - On failure, responds with [`SdkError<CreateSamplingRuleError>`](crate::error::CreateSamplingRuleError)
    pub fn create_sampling_rule(&self) -> crate::client::fluent_builders::CreateSamplingRule {
                                crate::client::fluent_builders::CreateSamplingRule::new(self.handle.clone())
                            }
}

