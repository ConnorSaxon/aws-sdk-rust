// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_put_property_values;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_component_type;

pub(crate) mod shape_create_entity;

pub(crate) mod shape_create_scene;

pub(crate) mod shape_create_sync_job;

pub(crate) mod shape_create_workspace;

pub(crate) mod shape_delete_component_type;

pub(crate) mod shape_delete_entity;

pub(crate) mod shape_delete_scene;

pub(crate) mod shape_delete_sync_job;

pub(crate) mod shape_delete_workspace;

pub(crate) mod shape_execute_query;

pub(crate) mod shape_get_component_type;

pub(crate) mod shape_get_entity;

pub(crate) mod shape_get_pricing_plan;

pub(crate) mod shape_get_property_value;

pub(crate) mod shape_get_property_value_history;

pub(crate) mod shape_get_scene;

pub(crate) mod shape_get_sync_job;

pub(crate) mod shape_get_workspace;

pub(crate) mod shape_list_component_types;

pub(crate) mod shape_list_entities;

pub(crate) mod shape_list_scenes;

pub(crate) mod shape_list_sync_jobs;

pub(crate) mod shape_list_sync_resources;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_workspaces;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_component_type;

pub(crate) mod shape_update_entity;

pub(crate) mod shape_update_pricing_plan;

pub(crate) mod shape_update_scene;

pub(crate) mod shape_update_workspace;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_batch_put_property_values_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_connector_failure_exception;

pub(crate) mod shape_connector_timeout_exception;

pub(crate) mod shape_create_component_type_input;

pub(crate) mod shape_create_entity_input;

pub(crate) mod shape_create_scene_input;

pub(crate) mod shape_create_sync_job_input;

pub(crate) mod shape_create_workspace_input;

pub(crate) mod shape_execute_query_input;

pub(crate) mod shape_get_property_value_history_input;

pub(crate) mod shape_get_property_value_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_component_types_input;

pub(crate) mod shape_list_entities_input;

pub(crate) mod shape_list_scenes_input;

pub(crate) mod shape_list_sync_jobs_input;

pub(crate) mod shape_list_sync_resources_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_workspaces_input;

pub(crate) mod shape_query_timeout_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_update_component_type_input;

pub(crate) mod shape_update_entity_input;

pub(crate) mod shape_update_pricing_plan_input;

pub(crate) mod shape_update_scene_input;

pub(crate) mod shape_update_workspace_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_column_descriptions;

pub(crate) mod shape_component_request;

pub(crate) mod shape_component_type_summaries;

pub(crate) mod shape_component_update_request;

pub(crate) mod shape_components_map;

pub(crate) mod shape_entity_summaries;

pub(crate) mod shape_error_entries;

pub(crate) mod shape_extends_from;

pub(crate) mod shape_function_request;

pub(crate) mod shape_functions_response;

pub(crate) mod shape_interpolation_parameters;

pub(crate) mod shape_list_component_types_filter;

pub(crate) mod shape_list_entities_filter;

pub(crate) mod shape_parent_entity_update_request;

pub(crate) mod shape_pricing_plan;

pub(crate) mod shape_property_definition_request;

pub(crate) mod shape_property_definitions_response;

pub(crate) mod shape_property_filter;

pub(crate) mod shape_property_group_request;

pub(crate) mod shape_property_groups_response;

pub(crate) mod shape_property_latest_value_map;

pub(crate) mod shape_property_value_entry;

pub(crate) mod shape_property_value_list;

pub(crate) mod shape_rows;

pub(crate) mod shape_scene_capabilities;

pub(crate) mod shape_scene_summaries;

pub(crate) mod shape_status;

pub(crate) mod shape_sync_job_status;

pub(crate) mod shape_sync_job_summaries;

pub(crate) mod shape_sync_resource_filter;

pub(crate) mod shape_sync_resource_summaries;

pub(crate) mod shape_tabular_conditions;

pub(crate) mod shape_tabular_property_values;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_workspace_summaries;

pub(crate) mod shape_batch_put_property_error_entry;

pub(crate) mod shape_bundle_information;

pub(crate) mod shape_column_description;

pub(crate) mod shape_component_property_group_request;

pub(crate) mod shape_component_response;

pub(crate) mod shape_component_type_summary;

pub(crate) mod shape_data_connector;

pub(crate) mod shape_data_type;

pub(crate) mod shape_data_value;

pub(crate) mod shape_entity_property_reference;

pub(crate) mod shape_entity_summary;

pub(crate) mod shape_error_details;

pub(crate) mod shape_function_response;

pub(crate) mod shape_order_by;

pub(crate) mod shape_property_definition_response;

pub(crate) mod shape_property_group_response;

pub(crate) mod shape_property_latest_value;

pub(crate) mod shape_property_request;

pub(crate) mod shape_property_value;

pub(crate) mod shape_property_value_history;

pub(crate) mod shape_row;

pub(crate) mod shape_scene_summary;

pub(crate) mod shape_sync_job_summary;

pub(crate) mod shape_sync_resource_summary;

pub(crate) mod shape_tabular_property_value;

pub(crate) mod shape_workspace_summary;

pub(crate) mod shape_component_property_group_responses;

pub(crate) mod shape_configuration;

pub(crate) mod shape_errors;

pub(crate) mod shape_lambda_function;

pub(crate) mod shape_pricing_bundles;

pub(crate) mod shape_property_names;

pub(crate) mod shape_property_responses;

pub(crate) mod shape_property_table_value;

pub(crate) mod shape_relationship;

pub(crate) mod shape_relationship_value;

pub(crate) mod shape_required_properties;

pub(crate) mod shape_row_data;

pub(crate) mod shape_sync_resource_status;

pub(crate) mod shape_values;

pub(crate) mod shape_batch_put_property_error;

pub(crate) mod shape_component_property_group_response;

pub(crate) mod shape_data_value_list;

pub(crate) mod shape_data_value_map;

pub(crate) mod shape_external_id_property;

pub(crate) mod shape_property_response;

pub(crate) mod shape_property_values;

