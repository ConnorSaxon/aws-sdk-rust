// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_get_application_component_details;

pub(crate) mod shape_get_application_component_strategies;

pub(crate) mod shape_get_assessment;

pub(crate) mod shape_get_import_file_task;

pub(crate) mod shape_get_latest_assessment_id;

pub(crate) mod shape_get_portfolio_preferences;

pub(crate) mod shape_get_portfolio_summary;

pub(crate) mod shape_get_recommendation_report_details;

pub(crate) mod shape_get_server_details;

pub(crate) mod shape_get_server_strategies;

pub(crate) mod shape_list_application_components;

pub(crate) mod shape_list_collectors;

pub(crate) mod shape_list_import_file_task;

pub(crate) mod shape_list_servers;

pub(crate) mod shape_put_portfolio_preferences;

pub(crate) mod shape_start_assessment;

pub(crate) mod shape_start_import_file_task;

pub(crate) mod shape_start_recommendation_report_generation;

pub(crate) mod shape_stop_assessment;

pub(crate) mod shape_update_application_component_config;

pub(crate) mod shape_update_server_config;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_dependency_exception;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_application_components_input;

pub(crate) mod shape_list_servers_input;

pub(crate) mod shape_put_portfolio_preferences_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_linked_role_lock_client_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_assessment_input;

pub(crate) mod shape_start_import_file_task_input;

pub(crate) mod shape_start_recommendation_report_generation_input;

pub(crate) mod shape_stop_assessment_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_application_component_config_input;

pub(crate) mod shape_update_server_config_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_application_component_detail;

pub(crate) mod shape_application_component_details;

pub(crate) mod shape_application_component_strategies;

pub(crate) mod shape_application_preferences;

pub(crate) mod shape_assessment_summary;

pub(crate) mod shape_assessment_target;

pub(crate) mod shape_assessment_targets;

pub(crate) mod shape_associated_applications;

pub(crate) mod shape_associated_server_i_ds;

pub(crate) mod shape_collectors;

pub(crate) mod shape_data_collection_details;

pub(crate) mod shape_database_preferences;

pub(crate) mod shape_group;

pub(crate) mod shape_list_import_file_task_information;

pub(crate) mod shape_prioritize_business_goals;

pub(crate) mod shape_recommendation_report_details;

pub(crate) mod shape_server_detail;

pub(crate) mod shape_server_details;

pub(crate) mod shape_server_strategies;

pub(crate) mod shape_source_code;

pub(crate) mod shape_strategy_option;

pub(crate) mod shape_app_unit_error;

pub(crate) mod shape_application_component_strategy;

pub(crate) mod shape_associated_application;

pub(crate) mod shape_business_goals;

pub(crate) mod shape_collector;

pub(crate) mod shape_database_config_detail;

pub(crate) mod shape_database_migration_preference;

pub(crate) mod shape_import_file_task_information;

pub(crate) mod shape_list_antipattern_severity_summary;

pub(crate) mod shape_list_application_component_status_summary;

pub(crate) mod shape_list_application_component_summary;

pub(crate) mod shape_list_server_status_summary;

pub(crate) mod shape_list_server_summary;

pub(crate) mod shape_list_strategy_summary;

pub(crate) mod shape_management_preference;

pub(crate) mod shape_recommendation_set;

pub(crate) mod shape_s3_keys;

pub(crate) mod shape_s3_object;

pub(crate) mod shape_server_error;

pub(crate) mod shape_server_strategy;

pub(crate) mod shape_source_code_repositories;

pub(crate) mod shape_system_info;

pub(crate) mod shape_antipattern_severity_summary;

pub(crate) mod shape_application_component_status_summary;

pub(crate) mod shape_application_component_summary;

pub(crate) mod shape_assessment_target_values;

pub(crate) mod shape_aws_managed_resources;

pub(crate) mod shape_configuration_summary;

pub(crate) mod shape_heterogeneous;

pub(crate) mod shape_homogeneous;

pub(crate) mod shape_network_info_list;

pub(crate) mod shape_no_database_migration_preference;

pub(crate) mod shape_no_management_preference;

pub(crate) mod shape_os_info;

pub(crate) mod shape_self_manage_resources;

pub(crate) mod shape_server_status_summary;

pub(crate) mod shape_server_summary;

pub(crate) mod shape_source_code_repository;

pub(crate) mod shape_strategy_summary;

pub(crate) mod shape_transformation_tool;

pub(crate) mod shape_aws_managed_target_destinations;

pub(crate) mod shape_heterogeneous_target_database_engines;

pub(crate) mod shape_homogeneous_target_database_engines;

pub(crate) mod shape_ip_address_based_remote_info_list;

pub(crate) mod shape_network_info;

pub(crate) mod shape_no_preference_target_destinations;

pub(crate) mod shape_pipeline_info_list;

pub(crate) mod shape_remote_source_code_analysis_server_info;

pub(crate) mod shape_self_manage_target_destinations;

pub(crate) mod shape_target_database_engines;

pub(crate) mod shape_vcenter_based_remote_info_list;

pub(crate) mod shape_version_control_info_list;

pub(crate) mod shape_ip_address_based_remote_info;

pub(crate) mod shape_pipeline_info;

pub(crate) mod shape_vcenter_based_remote_info;

pub(crate) mod shape_version_control_info;

