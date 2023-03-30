// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBuiltinSlotTypes`](crate::client::fluent_builders::GetBuiltinSlotTypes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetBuiltinSlotTypes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`locale(Locale)`](crate::client::fluent_builders::GetBuiltinSlotTypes::locale) / [`set_locale(Option<Locale>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::set_locale): <p>A list of locales that the slot type supports.</p>
    ///   - [`signature_contains(impl Into<String>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::signature_contains) / [`set_signature_contains(Option<String>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::set_signature_contains): <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::set_next_token): <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetBuiltinSlotTypes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetBuiltinSlotTypes::set_max_results): <p>The maximum number of slot types to return in the response. The default is 10.</p>
                            /// - On success, responds with [`GetBuiltinSlotTypesOutput`](crate::output::GetBuiltinSlotTypesOutput) with field(s):
    ///   - [`slot_types(Option<Vec<BuiltinSlotTypeMetadata>>)`](crate::output::GetBuiltinSlotTypesOutput::slot_types): <p>An array of <code>BuiltInSlotTypeMetadata</code> objects, one entry for each slot type returned.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetBuiltinSlotTypesOutput::next_token): <p>If the response is truncated, the response includes a pagination token that you can use in your next request to fetch the next page of slot types.</p>
                            /// - On failure, responds with [`SdkError<GetBuiltinSlotTypesError>`](crate::error::GetBuiltinSlotTypesError)
    pub fn get_builtin_slot_types(&self) -> crate::client::fluent_builders::GetBuiltinSlotTypes {
                                crate::client::fluent_builders::GetBuiltinSlotTypes::new(self.handle.clone())
                            }
}

