// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_discoverers_output_next_token(input: &crate::output::ListDiscoverersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_registries_output_next_token(input: &crate::output::ListRegistriesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_schemas_output_next_token(input: &crate::output::ListSchemasOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_schema_versions_output_next_token(input: &crate::output::ListSchemaVersionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_search_schemas_output_next_token(input: &crate::output::SearchSchemasOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_discoverers_output_discoverers(input: crate::output::ListDiscoverersOutput) -> std::option::Option<std::vec::Vec<crate::model::DiscovererSummary>> {
                    let input = match input.discoverers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_registries_output_registries(input: crate::output::ListRegistriesOutput) -> std::option::Option<std::vec::Vec<crate::model::RegistrySummary>> {
                    let input = match input.registries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_schemas_output_schemas(input: crate::output::ListSchemasOutput) -> std::option::Option<std::vec::Vec<crate::model::SchemaSummary>> {
                    let input = match input.schemas {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_schema_versions_output_schema_versions(input: crate::output::ListSchemaVersionsOutput) -> std::option::Option<std::vec::Vec<crate::model::SchemaVersionSummary>> {
                    let input = match input.schema_versions {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_search_schemas_output_schemas(input: crate::output::SearchSchemasOutput) -> std::option::Option<std::vec::Vec<crate::model::SearchSchemaSummary>> {
                    let input = match input.schemas {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

