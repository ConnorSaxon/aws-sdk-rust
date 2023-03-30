// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableMacie`](crate::client::fluent_builders::DisableMacie) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DisableMacie::send) it.
                            /// - On success, responds with [`DisableMacieOutput`](crate::output::DisableMacieOutput)
                            /// - On failure, responds with [`SdkError<DisableMacieError>`](crate::error::DisableMacieError)
    pub fn disable_macie(&self) -> crate::client::fluent_builders::DisableMacie {
                                crate::client::fluent_builders::DisableMacie::new(self.handle.clone())
                            }
}

