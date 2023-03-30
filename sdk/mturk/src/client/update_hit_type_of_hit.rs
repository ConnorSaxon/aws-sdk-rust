// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateHITTypeOfHIT`](crate::client::fluent_builders::UpdateHITTypeOfHIT) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hit_id(impl Into<String>)`](crate::client::fluent_builders::UpdateHITTypeOfHIT::hit_id) / [`set_hit_id(Option<String>)`](crate::client::fluent_builders::UpdateHITTypeOfHIT::set_hit_id): <p>The HIT to update.</p>
    ///   - [`hit_type_id(impl Into<String>)`](crate::client::fluent_builders::UpdateHITTypeOfHIT::hit_type_id) / [`set_hit_type_id(Option<String>)`](crate::client::fluent_builders::UpdateHITTypeOfHIT::set_hit_type_id): <p>The ID of the new HIT type.</p>
                            /// - On success, responds with [`UpdateHitTypeOfHitOutput`](crate::output::UpdateHitTypeOfHitOutput)
                            /// - On failure, responds with [`SdkError<UpdateHITTypeOfHITError>`](crate::error::UpdateHITTypeOfHITError)
    pub fn update_hit_type_of_hit(&self) -> crate::client::fluent_builders::UpdateHITTypeOfHIT {
                                crate::client::fluent_builders::UpdateHITTypeOfHIT::new(self.handle.clone())
                            }
}

