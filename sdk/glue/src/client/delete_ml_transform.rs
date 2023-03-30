// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMLTransform`](crate::client::fluent_builders::DeleteMLTransform) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transform_id(impl Into<String>)`](crate::client::fluent_builders::DeleteMLTransform::transform_id) / [`set_transform_id(Option<String>)`](crate::client::fluent_builders::DeleteMLTransform::set_transform_id): <p>The unique identifier of the transform to delete.</p>
                            /// - On success, responds with [`DeleteMlTransformOutput`](crate::output::DeleteMlTransformOutput) with field(s):
    ///   - [`transform_id(Option<String>)`](crate::output::DeleteMlTransformOutput::transform_id): <p>The unique identifier of the transform that was deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteMLTransformError>`](crate::error::DeleteMLTransformError)
    pub fn delete_ml_transform(&self) -> crate::client::fluent_builders::DeleteMLTransform {
                                crate::client::fluent_builders::DeleteMLTransform::new(self.handle.clone())
                            }
}

