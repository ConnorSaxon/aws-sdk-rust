// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFindingsFeedback`](crate::client::fluent_builders::UpdateFindingsFeedback) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::UpdateFindingsFeedback::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::UpdateFindingsFeedback::set_detector_id): <p>The ID of the detector associated with the findings to update feedback for.</p>
    ///   - [`finding_ids(Vec<String>)`](crate::client::fluent_builders::UpdateFindingsFeedback::finding_ids) / [`set_finding_ids(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateFindingsFeedback::set_finding_ids): <p>The IDs of the findings that you want to mark as useful or not useful.</p>
    ///   - [`feedback(Feedback)`](crate::client::fluent_builders::UpdateFindingsFeedback::feedback) / [`set_feedback(Option<Feedback>)`](crate::client::fluent_builders::UpdateFindingsFeedback::set_feedback): <p>The feedback for the finding.</p>
    ///   - [`comments(impl Into<String>)`](crate::client::fluent_builders::UpdateFindingsFeedback::comments) / [`set_comments(Option<String>)`](crate::client::fluent_builders::UpdateFindingsFeedback::set_comments): <p>Additional feedback about the GuardDuty findings.</p>
                            /// - On success, responds with [`UpdateFindingsFeedbackOutput`](crate::output::UpdateFindingsFeedbackOutput)
                            /// - On failure, responds with [`SdkError<UpdateFindingsFeedbackError>`](crate::error::UpdateFindingsFeedbackError)
    pub fn update_findings_feedback(&self) -> crate::client::fluent_builders::UpdateFindingsFeedback {
                                crate::client::fluent_builders::UpdateFindingsFeedback::new(self.handle.clone())
                            }
}

