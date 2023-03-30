// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCompatibleImages`](crate::client::fluent_builders::ListCompatibleImages) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCompatibleImages::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCompatibleImages::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCompatibleImages::set_max_results): <p>The maximum number of results for the list of compatible images. Currently, a Snowball Edge device can store 10 AMIs.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCompatibleImages::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCompatibleImages::set_next_token): <p>HTTP requests are stateless. To identify what object comes "next" in the list of compatible images, you can specify a value for <code>NextToken</code> as the starting point for your list of returned images.</p>
                            /// - On success, responds with [`ListCompatibleImagesOutput`](crate::output::ListCompatibleImagesOutput) with field(s):
    ///   - [`compatible_images(Option<Vec<CompatibleImage>>)`](crate::output::ListCompatibleImagesOutput::compatible_images): <p>A JSON-formatted object that describes a compatible AMI, including the ID and name for a Snow device AMI.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListCompatibleImagesOutput::next_token): <p>Because HTTP requests are stateless, this is the starting point for your next list of returned images.</p>
                            /// - On failure, responds with [`SdkError<ListCompatibleImagesError>`](crate::error::ListCompatibleImagesError)
    pub fn list_compatible_images(&self) -> crate::client::fluent_builders::ListCompatibleImages {
                                crate::client::fluent_builders::ListCompatibleImages::new(self.handle.clone())
                            }
}

