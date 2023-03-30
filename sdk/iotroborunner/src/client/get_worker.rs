// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWorker`](crate::client::fluent_builders::GetWorker) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetWorker::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetWorker::set_id): Full ARN of the worker.
                            /// - On success, responds with [`GetWorkerOutput`](crate::output::GetWorkerOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::GetWorkerOutput::arn): Full ARN of the worker.
    ///   - [`id(Option<String>)`](crate::output::GetWorkerOutput::id): Filters access by the workers identifier
    ///   - [`fleet(Option<String>)`](crate::output::GetWorkerOutput::fleet): Full ARN of the worker fleet.
    ///   - [`site(Option<String>)`](crate::output::GetWorkerOutput::site): Site ARN.
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetWorkerOutput::created_at): Timestamp at which the resource was created.
    ///   - [`updated_at(Option<DateTime>)`](crate::output::GetWorkerOutput::updated_at): Timestamp at which the resource was last updated.
    ///   - [`name(Option<String>)`](crate::output::GetWorkerOutput::name): Human friendly name of the resource.
    ///   - [`additional_transient_properties(Option<String>)`](crate::output::GetWorkerOutput::additional_transient_properties): JSON blob containing unstructured worker properties that are transient and may change during regular operation.
    ///   - [`additional_fixed_properties(Option<String>)`](crate::output::GetWorkerOutput::additional_fixed_properties): JSON blob containing unstructured worker properties that are fixed and won't change during regular operation.
    ///   - [`vendor_properties(Option<VendorProperties>)`](crate::output::GetWorkerOutput::vendor_properties): Properties of the worker that are provided by the vendor FMS.
    ///   - [`position(Option<PositionCoordinates>)`](crate::output::GetWorkerOutput::position): Supported coordinates for worker position.
    ///   - [`orientation(Option<Orientation>)`](crate::output::GetWorkerOutput::orientation): Worker orientation measured in units clockwise from north.
                            /// - On failure, responds with [`SdkError<GetWorkerError>`](crate::error::GetWorkerError)
    pub fn get_worker(&self) -> crate::client::fluent_builders::GetWorker {
                                crate::client::fluent_builders::GetWorker::new(self.handle.clone())
                            }
}

