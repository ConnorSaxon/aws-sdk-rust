// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_activate_event_source;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_cancel_replay;

pub(crate) mod shape_create_api_destination;

pub(crate) mod shape_create_archive;

pub(crate) mod shape_create_connection;

pub(crate) mod shape_create_event_bus;

pub(crate) mod shape_create_partner_event_source;

pub(crate) mod shape_deactivate_event_source;

pub(crate) mod shape_deauthorize_connection;

pub(crate) mod shape_delete_api_destination;

pub(crate) mod shape_delete_archive;

pub(crate) mod shape_delete_connection;

pub(crate) mod shape_delete_event_bus;

pub(crate) mod shape_delete_partner_event_source;

pub(crate) mod shape_delete_rule;

pub(crate) mod shape_describe_api_destination;

pub(crate) mod shape_describe_archive;

pub(crate) mod shape_describe_connection;

pub(crate) mod shape_describe_event_bus;

pub(crate) mod shape_describe_event_source;

pub(crate) mod shape_describe_partner_event_source;

pub(crate) mod shape_describe_replay;

pub(crate) mod shape_describe_rule;

pub(crate) mod shape_disable_rule;

pub(crate) mod shape_enable_rule;

pub(crate) mod shape_list_api_destinations;

pub(crate) mod shape_list_archives;

pub(crate) mod shape_list_connections;

pub(crate) mod shape_list_event_buses;

pub(crate) mod shape_list_event_sources;

pub(crate) mod shape_list_partner_event_source_accounts;

pub(crate) mod shape_list_partner_event_sources;

pub(crate) mod shape_list_replays;

pub(crate) mod shape_list_rule_names_by_target;

pub(crate) mod shape_list_rules;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_targets_by_rule;

pub(crate) mod shape_put_events;

pub(crate) mod shape_put_partner_events;

pub(crate) mod shape_put_permission;

pub(crate) mod shape_put_rule;

pub(crate) mod shape_put_targets;

pub(crate) mod shape_remove_permission;

pub(crate) mod shape_remove_targets;

pub(crate) mod shape_start_replay;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_test_event_pattern;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_api_destination;

pub(crate) mod shape_update_archive;

pub(crate) mod shape_update_connection;

pub(crate) mod shape_activate_event_source_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_cancel_replay_input;

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_create_api_destination_input;

pub(crate) mod shape_create_archive_input;

pub(crate) mod shape_create_connection_input;

pub(crate) mod shape_create_event_bus_input;

pub(crate) mod shape_create_partner_event_source_input;

pub(crate) mod shape_deactivate_event_source_input;

pub(crate) mod shape_deauthorize_connection_input;

pub(crate) mod shape_delete_api_destination_input;

pub(crate) mod shape_delete_archive_input;

pub(crate) mod shape_delete_connection_input;

pub(crate) mod shape_delete_event_bus_input;

pub(crate) mod shape_delete_partner_event_source_input;

pub(crate) mod shape_delete_rule_input;

pub(crate) mod shape_describe_api_destination_input;

pub(crate) mod shape_describe_archive_input;

pub(crate) mod shape_describe_connection_input;

pub(crate) mod shape_describe_event_bus_input;

pub(crate) mod shape_describe_event_source_input;

pub(crate) mod shape_describe_partner_event_source_input;

pub(crate) mod shape_describe_replay_input;

pub(crate) mod shape_describe_rule_input;

pub(crate) mod shape_disable_rule_input;

pub(crate) mod shape_enable_rule_input;

pub(crate) mod shape_illegal_status_exception;

pub(crate) mod shape_internal_exception;

pub(crate) mod shape_invalid_event_pattern_exception;

pub(crate) mod shape_invalid_state_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_api_destinations_input;

pub(crate) mod shape_list_archives_input;

pub(crate) mod shape_list_connections_input;

pub(crate) mod shape_list_event_buses_input;

pub(crate) mod shape_list_event_sources_input;

pub(crate) mod shape_list_partner_event_source_accounts_input;

pub(crate) mod shape_list_partner_event_sources_input;

pub(crate) mod shape_list_replays_input;

pub(crate) mod shape_list_rule_names_by_target_input;

pub(crate) mod shape_list_rules_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_targets_by_rule_input;

pub(crate) mod shape_managed_rule_exception;

pub(crate) mod shape_operation_disabled_exception;

pub(crate) mod shape_policy_length_exceeded_exception;

pub(crate) mod shape_put_events_input;

pub(crate) mod shape_put_partner_events_input;

pub(crate) mod shape_put_permission_input;

pub(crate) mod shape_put_rule_input;

pub(crate) mod shape_put_targets_input;

pub(crate) mod shape_remove_permission_input;

pub(crate) mod shape_remove_targets_input;

pub(crate) mod shape_resource_already_exists_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_replay_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_test_event_pattern_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_api_destination_input;

pub(crate) mod shape_update_archive_input;

pub(crate) mod shape_update_connection_input;

pub(crate) mod shape_api_destination_response_list;

pub(crate) mod shape_archive_response_list;

pub(crate) mod shape_condition;

pub(crate) mod shape_connection_auth_response_parameters;

pub(crate) mod shape_connection_response_list;

pub(crate) mod shape_create_connection_auth_request_parameters;

pub(crate) mod shape_event_bus_list;

pub(crate) mod shape_event_source_list;

pub(crate) mod shape_partner_event_source_account_list;

pub(crate) mod shape_partner_event_source_list;

pub(crate) mod shape_put_events_request_entry;

pub(crate) mod shape_put_events_result_entry_list;

pub(crate) mod shape_put_partner_events_request_entry;

pub(crate) mod shape_put_partner_events_result_entry_list;

pub(crate) mod shape_put_targets_result_entry_list;

pub(crate) mod shape_remove_targets_result_entry_list;

pub(crate) mod shape_replay_destination;

pub(crate) mod shape_replay_list;

pub(crate) mod shape_rule_name_list;

pub(crate) mod shape_rule_response_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_target;

pub(crate) mod shape_target_list;

pub(crate) mod shape_update_connection_auth_request_parameters;

pub(crate) mod shape_api_destination;

pub(crate) mod shape_archive;

pub(crate) mod shape_batch_parameters;

pub(crate) mod shape_connection;

pub(crate) mod shape_connection_api_key_auth_response_parameters;

pub(crate) mod shape_connection_basic_auth_response_parameters;

pub(crate) mod shape_connection_http_parameters;

pub(crate) mod shape_connection_o_auth_response_parameters;

pub(crate) mod shape_create_connection_api_key_auth_request_parameters;

pub(crate) mod shape_create_connection_basic_auth_request_parameters;

pub(crate) mod shape_create_connection_o_auth_request_parameters;

pub(crate) mod shape_dead_letter_config;

pub(crate) mod shape_ecs_parameters;

pub(crate) mod shape_event_bus;

pub(crate) mod shape_event_source;

pub(crate) mod shape_http_parameters;

pub(crate) mod shape_input_transformer;

pub(crate) mod shape_kinesis_parameters;

pub(crate) mod shape_partner_event_source;

pub(crate) mod shape_partner_event_source_account;

pub(crate) mod shape_put_events_result_entry;

pub(crate) mod shape_put_partner_events_result_entry;

pub(crate) mod shape_put_targets_result_entry;

pub(crate) mod shape_redshift_data_parameters;

pub(crate) mod shape_remove_targets_result_entry;

pub(crate) mod shape_replay;

pub(crate) mod shape_replay_destination_filters;

pub(crate) mod shape_retry_policy;

pub(crate) mod shape_rule;

pub(crate) mod shape_run_command_parameters;

pub(crate) mod shape_sage_maker_pipeline_parameters;

pub(crate) mod shape_sqs_parameters;

pub(crate) mod shape_update_connection_api_key_auth_request_parameters;

pub(crate) mod shape_update_connection_basic_auth_request_parameters;

pub(crate) mod shape_update_connection_o_auth_request_parameters;

pub(crate) mod shape_batch_array_properties;

pub(crate) mod shape_batch_retry_strategy;

pub(crate) mod shape_capacity_provider_strategy_item;

pub(crate) mod shape_connection_body_parameter;

pub(crate) mod shape_connection_body_parameters_list;

pub(crate) mod shape_connection_header_parameter;

pub(crate) mod shape_connection_header_parameters_list;

pub(crate) mod shape_connection_o_auth_client_response_parameters;

pub(crate) mod shape_connection_query_string_parameter;

pub(crate) mod shape_connection_query_string_parameters_list;

pub(crate) mod shape_create_connection_o_auth_client_request_parameters;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_placement_constraint;

pub(crate) mod shape_placement_strategy;

pub(crate) mod shape_run_command_target;

pub(crate) mod shape_sage_maker_pipeline_parameter;

pub(crate) mod shape_update_connection_o_auth_client_request_parameters;

pub(crate) mod shape_aws_vpc_configuration;

pub(crate) mod shape_capacity_provider_strategy;

pub(crate) mod shape_header_parameters_map;

pub(crate) mod shape_path_parameter_list;

pub(crate) mod shape_placement_constraints;

pub(crate) mod shape_placement_strategies;

pub(crate) mod shape_query_string_parameters_map;

pub(crate) mod shape_run_command_targets;

pub(crate) mod shape_sage_maker_pipeline_parameter_list;

pub(crate) mod shape_transformer_paths;

pub(crate) mod shape_run_command_target_values;

pub(crate) mod shape_string_list;

