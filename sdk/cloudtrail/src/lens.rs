// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_get_query_results_output_next_token(input: &crate::output::GetQueryResultsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_channels_output_next_token(input: &crate::output::ListChannelsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_event_data_stores_output_next_token(input: &crate::output::ListEventDataStoresOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_import_failures_output_next_token(input: &crate::output::ListImportFailuresOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_imports_output_next_token(input: &crate::output::ListImportsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_public_keys_output_next_token(input: &crate::output::ListPublicKeysOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_queries_output_next_token(input: &crate::output::ListQueriesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_tags_output_next_token(input: &crate::output::ListTagsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_trails_output_next_token(input: &crate::output::ListTrailsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_lookup_events_output_next_token(input: &crate::output::LookupEventsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_import_failures_output_failures(input: crate::output::ListImportFailuresOutput) -> std::option::Option<std::vec::Vec<crate::model::ImportFailureListItem>> {
                    let input = match input.failures {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_imports_output_imports(input: crate::output::ListImportsOutput) -> std::option::Option<std::vec::Vec<crate::model::ImportsListItem>> {
                    let input = match input.imports {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_public_keys_output_public_key_list(input: crate::output::ListPublicKeysOutput) -> std::option::Option<std::vec::Vec<crate::model::PublicKey>> {
                    let input = match input.public_key_list {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_tags_output_resource_tag_list(input: crate::output::ListTagsOutput) -> std::option::Option<std::vec::Vec<crate::model::ResourceTag>> {
                    let input = match input.resource_tag_list {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_trails_output_trails(input: crate::output::ListTrailsOutput) -> std::option::Option<std::vec::Vec<crate::model::TrailInfo>> {
                    let input = match input.trails {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_lookup_events_output_events(input: crate::output::LookupEventsOutput) -> std::option::Option<std::vec::Vec<crate::model::Event>> {
                    let input = match input.events {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

