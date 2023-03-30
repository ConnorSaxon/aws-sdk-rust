// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAssessmentTemplate`](crate::client::fluent_builders::CreateAssessmentTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`assessment_target_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAssessmentTemplate::assessment_target_arn) / [`set_assessment_target_arn(Option<String>)`](crate::client::fluent_builders::CreateAssessmentTemplate::set_assessment_target_arn): <p>The ARN that specifies the assessment target for which you want to create the assessment template.</p>
    ///   - [`assessment_template_name(impl Into<String>)`](crate::client::fluent_builders::CreateAssessmentTemplate::assessment_template_name) / [`set_assessment_template_name(Option<String>)`](crate::client::fluent_builders::CreateAssessmentTemplate::set_assessment_template_name): <p>The user-defined name that identifies the assessment template that you want to create. You can create several assessment templates for an assessment target. The names of the assessment templates that correspond to a particular assessment target must be unique.</p>
    ///   - [`duration_in_seconds(i32)`](crate::client::fluent_builders::CreateAssessmentTemplate::duration_in_seconds) / [`set_duration_in_seconds(i32)`](crate::client::fluent_builders::CreateAssessmentTemplate::set_duration_in_seconds): <p>The duration of the assessment run in seconds.</p>
    ///   - [`rules_package_arns(Vec<String>)`](crate::client::fluent_builders::CreateAssessmentTemplate::rules_package_arns) / [`set_rules_package_arns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateAssessmentTemplate::set_rules_package_arns): <p>The ARNs that specify the rules packages that you want to attach to the assessment template.</p>
    ///   - [`user_attributes_for_findings(Vec<Attribute>)`](crate::client::fluent_builders::CreateAssessmentTemplate::user_attributes_for_findings) / [`set_user_attributes_for_findings(Option<Vec<Attribute>>)`](crate::client::fluent_builders::CreateAssessmentTemplate::set_user_attributes_for_findings): <p>The user-defined attributes that are assigned to every finding that is generated by the assessment run that uses this assessment template. An attribute is a key and value pair (an <code>Attribute</code> object). Within an assessment template, each key must be unique.</p>
                            /// - On success, responds with [`CreateAssessmentTemplateOutput`](crate::output::CreateAssessmentTemplateOutput) with field(s):
    ///   - [`assessment_template_arn(Option<String>)`](crate::output::CreateAssessmentTemplateOutput::assessment_template_arn): <p>The ARN that specifies the assessment template that is created.</p>
                            /// - On failure, responds with [`SdkError<CreateAssessmentTemplateError>`](crate::error::CreateAssessmentTemplateError)
    pub fn create_assessment_template(&self) -> crate::client::fluent_builders::CreateAssessmentTemplate {
                                crate::client::fluent_builders::CreateAssessmentTemplate::new(self.handle.clone())
                            }
}

