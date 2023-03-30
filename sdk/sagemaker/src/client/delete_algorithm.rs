// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAlgorithm`](crate::client::fluent_builders::DeleteAlgorithm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`algorithm_name(impl Into<String>)`](crate::client::fluent_builders::DeleteAlgorithm::algorithm_name) / [`set_algorithm_name(Option<String>)`](crate::client::fluent_builders::DeleteAlgorithm::set_algorithm_name): <p>The name of the algorithm to delete.</p>
                            /// - On success, responds with [`DeleteAlgorithmOutput`](crate::output::DeleteAlgorithmOutput)
                            /// - On failure, responds with [`SdkError<DeleteAlgorithmError>`](crate::error::DeleteAlgorithmError)
    pub fn delete_algorithm(&self) -> crate::client::fluent_builders::DeleteAlgorithm {
                                crate::client::fluent_builders::DeleteAlgorithm::new(self.handle.clone())
                            }
}

