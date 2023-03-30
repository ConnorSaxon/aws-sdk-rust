// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateQualificationWithWorker`](crate::client::fluent_builders::AssociateQualificationWithWorker) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`qualification_type_id(impl Into<String>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::qualification_type_id) / [`set_qualification_type_id(Option<String>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::set_qualification_type_id): <p>The ID of the Qualification type to use for the assigned Qualification.</p>
    ///   - [`worker_id(impl Into<String>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::worker_id) / [`set_worker_id(Option<String>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::set_worker_id): <p> The ID of the Worker to whom the Qualification is being assigned. Worker IDs are included with submitted HIT assignments and Qualification requests. </p>
    ///   - [`integer_value(i32)`](crate::client::fluent_builders::AssociateQualificationWithWorker::integer_value) / [`set_integer_value(Option<i32>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::set_integer_value): <p>The value of the Qualification to assign.</p>
    ///   - [`send_notification(bool)`](crate::client::fluent_builders::AssociateQualificationWithWorker::send_notification) / [`set_send_notification(Option<bool>)`](crate::client::fluent_builders::AssociateQualificationWithWorker::set_send_notification): <p> Specifies whether to send a notification email message to the Worker saying that the qualification was assigned to the Worker. Note: this is true by default. </p>
                            /// - On success, responds with [`AssociateQualificationWithWorkerOutput`](crate::output::AssociateQualificationWithWorkerOutput)
                            /// - On failure, responds with [`SdkError<AssociateQualificationWithWorkerError>`](crate::error::AssociateQualificationWithWorkerError)
    pub fn associate_qualification_with_worker(&self) -> crate::client::fluent_builders::AssociateQualificationWithWorker {
                                crate::client::fluent_builders::AssociateQualificationWithWorker::new(self.handle.clone())
                            }
}

