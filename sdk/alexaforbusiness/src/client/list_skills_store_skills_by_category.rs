// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSkillsStoreSkillsByCategory`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`category_id(i64)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::category_id) / [`set_category_id(Option<i64>)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::set_category_id): <p>The category ID for which the skills are being retrieved from the skill store.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::set_next_token): <p>The tokens used for pagination.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::set_max_results): <p>The maximum number of skills returned per paginated calls.</p>
                            /// - On success, responds with [`ListSkillsStoreSkillsByCategoryOutput`](crate::output::ListSkillsStoreSkillsByCategoryOutput) with field(s):
    ///   - [`skills_store_skills(Option<Vec<SkillsStoreSkill>>)`](crate::output::ListSkillsStoreSkillsByCategoryOutput::skills_store_skills): <p>The skill store skills.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSkillsStoreSkillsByCategoryOutput::next_token): <p>The tokens used for pagination.</p>
                            /// - On failure, responds with [`SdkError<ListSkillsStoreSkillsByCategoryError>`](crate::error::ListSkillsStoreSkillsByCategoryError)
    pub fn list_skills_store_skills_by_category(&self) -> crate::client::fluent_builders::ListSkillsStoreSkillsByCategory {
                                crate::client::fluent_builders::ListSkillsStoreSkillsByCategory::new(self.handle.clone())
                            }
}

