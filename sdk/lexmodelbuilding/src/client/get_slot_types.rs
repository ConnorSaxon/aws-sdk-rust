// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSlotTypes`](crate::client::fluent_builders::GetSlotTypes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetSlotTypes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetSlotTypes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetSlotTypes::set_next_token): <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch next page of slot types, specify the pagination token in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetSlotTypes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetSlotTypes::set_max_results): <p>The maximum number of slot types to return in the response. The default is 10.</p>
    ///   - [`name_contains(impl Into<String>)`](crate::client::fluent_builders::GetSlotTypes::name_contains) / [`set_name_contains(Option<String>)`](crate::client::fluent_builders::GetSlotTypes::set_name_contains): <p>Substring to match in slot type names. A slot type will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
                            /// - On success, responds with [`GetSlotTypesOutput`](crate::output::GetSlotTypesOutput) with field(s):
    ///   - [`slot_types(Option<Vec<SlotTypeMetadata>>)`](crate::output::GetSlotTypesOutput::slot_types): <p>An array of objects, one for each slot type, that provides information such as the name of the slot type, the version, and a description.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetSlotTypesOutput::next_token): <p>If the response is truncated, it includes a pagination token that you can specify in your next request to fetch the next page of slot types.</p>
                            /// - On failure, responds with [`SdkError<GetSlotTypesError>`](crate::error::GetSlotTypesError)
    pub fn get_slot_types(&self) -> crate::client::fluent_builders::GetSlotTypes {
                                crate::client::fluent_builders::GetSlotTypes::new(self.handle.clone())
                            }
}

