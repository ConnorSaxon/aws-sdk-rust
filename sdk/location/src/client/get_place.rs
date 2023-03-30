// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPlace`](crate::client::fluent_builders::GetPlace) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_name(impl Into<String>)`](crate::client::fluent_builders::GetPlace::index_name) / [`set_index_name(Option<String>)`](crate::client::fluent_builders::GetPlace::set_index_name): <p>The name of the place index resource that you want to use for the search.</p>
    ///   - [`place_id(impl Into<String>)`](crate::client::fluent_builders::GetPlace::place_id) / [`set_place_id(Option<String>)`](crate::client::fluent_builders::GetPlace::set_place_id): <p>The identifier of the place to find.</p>
    ///   - [`language(impl Into<String>)`](crate::client::fluent_builders::GetPlace::language) / [`set_language(Option<String>)`](crate::client::fluent_builders::GetPlace::set_language): <p>The preferred language used to return results. The value must be a valid <a href="https://tools.ietf.org/search/bcp47">BCP 47</a> language tag, for example, <code>en</code> for English.</p>  <p>This setting affects the languages used in the results, but not the results themselves. If no language is specified, or not supported for a particular result, the partner automatically chooses a language for the result.</p>  <p>For an example, we'll use the Greek language. You search for a location around Athens, Greece, with the <code>language</code> parameter set to <code>en</code>. The <code>city</code> in the results will most likely be returned as <code>Athens</code>.</p>  <p>If you set the <code>language</code> parameter to <code>el</code>, for Greek, then the <code>city</code> in the results will more likely be returned as <code>Αθήνα</code>.</p>  <p>If the data provider does not have a value for Greek, the result will be in a language that the provider does support.</p>
                            /// - On success, responds with [`GetPlaceOutput`](crate::output::GetPlaceOutput) with field(s):
    ///   - [`place(Option<Place>)`](crate::output::GetPlaceOutput::place): <p>Details about the result, such as its address and position.</p>
                            /// - On failure, responds with [`SdkError<GetPlaceError>`](crate::error::GetPlaceError)
    pub fn get_place(&self) -> crate::client::fluent_builders::GetPlace {
                                crate::client::fluent_builders::GetPlace::new(self.handle.clone())
                            }
}

