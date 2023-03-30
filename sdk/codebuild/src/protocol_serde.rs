// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_delete_builds;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_batch_get_build_batches;

pub(crate) mod shape_batch_get_builds;

pub(crate) mod shape_batch_get_projects;

pub(crate) mod shape_batch_get_report_groups;

pub(crate) mod shape_batch_get_reports;

pub(crate) mod shape_create_project;

pub(crate) mod shape_create_report_group;

pub(crate) mod shape_create_webhook;

pub(crate) mod shape_delete_build_batch;

pub(crate) mod shape_delete_project;

pub(crate) mod shape_delete_report;

pub(crate) mod shape_delete_report_group;

pub(crate) mod shape_delete_resource_policy;

pub(crate) mod shape_delete_source_credentials;

pub(crate) mod shape_delete_webhook;

pub(crate) mod shape_describe_code_coverages;

pub(crate) mod shape_describe_test_cases;

pub(crate) mod shape_get_report_group_trend;

pub(crate) mod shape_get_resource_policy;

pub(crate) mod shape_import_source_credentials;

pub(crate) mod shape_invalidate_project_cache;

pub(crate) mod shape_list_build_batches;

pub(crate) mod shape_list_build_batches_for_project;

pub(crate) mod shape_list_builds;

pub(crate) mod shape_list_builds_for_project;

pub(crate) mod shape_list_curated_environment_images;

pub(crate) mod shape_list_projects;

pub(crate) mod shape_list_report_groups;

pub(crate) mod shape_list_reports;

pub(crate) mod shape_list_reports_for_report_group;

pub(crate) mod shape_list_shared_projects;

pub(crate) mod shape_list_shared_report_groups;

pub(crate) mod shape_list_source_credentials;

pub(crate) mod shape_put_resource_policy;

pub(crate) mod shape_retry_build;

pub(crate) mod shape_retry_build_batch;

pub(crate) mod shape_start_build;

pub(crate) mod shape_start_build_batch;

pub(crate) mod shape_stop_build;

pub(crate) mod shape_stop_build_batch;

pub(crate) mod shape_update_project;

pub(crate) mod shape_update_project_visibility;

pub(crate) mod shape_update_report_group;

pub(crate) mod shape_update_webhook;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_account_limit_exceeded_exception;

pub(crate) mod shape_batch_delete_builds_input;

pub(crate) mod shape_batch_get_build_batches_input;

pub(crate) mod shape_batch_get_builds_input;

pub(crate) mod shape_batch_get_projects_input;

pub(crate) mod shape_batch_get_report_groups_input;

pub(crate) mod shape_batch_get_reports_input;

pub(crate) mod shape_create_project_input;

pub(crate) mod shape_create_report_group_input;

pub(crate) mod shape_create_webhook_input;

pub(crate) mod shape_delete_build_batch_input;

pub(crate) mod shape_delete_project_input;

pub(crate) mod shape_delete_report_group_input;

pub(crate) mod shape_delete_report_input;

pub(crate) mod shape_delete_resource_policy_input;

pub(crate) mod shape_delete_source_credentials_input;

pub(crate) mod shape_delete_webhook_input;

pub(crate) mod shape_describe_code_coverages_input;

pub(crate) mod shape_describe_test_cases_input;

pub(crate) mod shape_get_report_group_trend_input;

pub(crate) mod shape_get_resource_policy_input;

pub(crate) mod shape_import_source_credentials_input;

pub(crate) mod shape_invalid_input_exception;

pub(crate) mod shape_invalidate_project_cache_input;

pub(crate) mod shape_list_build_batches_for_project_input;

pub(crate) mod shape_list_build_batches_input;

pub(crate) mod shape_list_builds_for_project_input;

pub(crate) mod shape_list_builds_input;

pub(crate) mod shape_list_projects_input;

pub(crate) mod shape_list_report_groups_input;

pub(crate) mod shape_list_reports_for_report_group_input;

pub(crate) mod shape_list_reports_input;

pub(crate) mod shape_list_shared_projects_input;

pub(crate) mod shape_list_shared_report_groups_input;

pub(crate) mod shape_o_auth_provider_exception;

pub(crate) mod shape_put_resource_policy_input;

pub(crate) mod shape_resource_already_exists_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_retry_build_batch_input;

pub(crate) mod shape_retry_build_input;

pub(crate) mod shape_start_build_batch_input;

pub(crate) mod shape_start_build_input;

pub(crate) mod shape_stop_build_batch_input;

pub(crate) mod shape_stop_build_input;

pub(crate) mod shape_update_project_input;

pub(crate) mod shape_update_project_visibility_input;

pub(crate) mod shape_update_report_group_input;

pub(crate) mod shape_update_webhook_input;

pub(crate) mod shape_build;

pub(crate) mod shape_build_batch;

pub(crate) mod shape_build_batch_filter;

pub(crate) mod shape_build_batch_ids;

pub(crate) mod shape_build_batches;

pub(crate) mod shape_build_ids;

pub(crate) mod shape_build_status_config;

pub(crate) mod shape_builds;

pub(crate) mod shape_builds_not_deleted;

pub(crate) mod shape_code_coverages;

pub(crate) mod shape_environment_platforms;

pub(crate) mod shape_environment_variable;

pub(crate) mod shape_git_submodules_config;

pub(crate) mod shape_logs_config;

pub(crate) mod shape_project;

pub(crate) mod shape_project_arns;

pub(crate) mod shape_project_artifacts;

pub(crate) mod shape_project_build_batch_config;

pub(crate) mod shape_project_cache;

pub(crate) mod shape_project_environment;

pub(crate) mod shape_project_file_system_location;

pub(crate) mod shape_project_names;

pub(crate) mod shape_project_source;

pub(crate) mod shape_project_source_version;

pub(crate) mod shape_projects;

pub(crate) mod shape_registry_credential;

pub(crate) mod shape_report_arns;

pub(crate) mod shape_report_export_config;

pub(crate) mod shape_report_filter;

pub(crate) mod shape_report_group;

pub(crate) mod shape_report_group_arns;

pub(crate) mod shape_report_group_trend_raw_data_list;

pub(crate) mod shape_report_group_trend_stats;

pub(crate) mod shape_report_groups;

pub(crate) mod shape_reports;

pub(crate) mod shape_source_auth;

pub(crate) mod shape_source_credentials_infos;

pub(crate) mod shape_tag;

pub(crate) mod shape_test_case_filter;

pub(crate) mod shape_test_cases;

pub(crate) mod shape_vpc_config;

pub(crate) mod shape_webhook;

pub(crate) mod shape_webhook_filter;

pub(crate) mod shape_batch_restrictions;

pub(crate) mod shape_build_artifacts;

pub(crate) mod shape_build_artifacts_list;

pub(crate) mod shape_build_batch_phases;

pub(crate) mod shape_build_groups;

pub(crate) mod shape_build_not_deleted;

pub(crate) mod shape_build_phases;

pub(crate) mod shape_build_report_arns;

pub(crate) mod shape_cloud_watch_logs_config;

pub(crate) mod shape_code_coverage;

pub(crate) mod shape_debug_session;

pub(crate) mod shape_environment_platform;

pub(crate) mod shape_exported_environment_variables;

pub(crate) mod shape_filter_groups;

pub(crate) mod shape_logs_location;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_project_artifacts_list;

pub(crate) mod shape_project_badge;

pub(crate) mod shape_project_file_system_locations;

pub(crate) mod shape_project_secondary_source_versions;

pub(crate) mod shape_project_sources;

pub(crate) mod shape_report;

pub(crate) mod shape_report_with_raw_data;

pub(crate) mod shape_s3_logs_config;

pub(crate) mod shape_s3_report_export_config;

pub(crate) mod shape_source_credentials_info;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_test_case;

pub(crate) mod shape_build_batch_phase;

pub(crate) mod shape_build_group;

pub(crate) mod shape_build_phase;

pub(crate) mod shape_code_coverage_report_summary;

pub(crate) mod shape_environment_languages;

pub(crate) mod shape_environment_variables;

pub(crate) mod shape_exported_environment_variable;

pub(crate) mod shape_filter_group;

pub(crate) mod shape_project_cache_modes;

pub(crate) mod shape_security_group_ids;

pub(crate) mod shape_subnets;

pub(crate) mod shape_test_report_summary;

pub(crate) mod shape_build_summaries;

pub(crate) mod shape_build_summary;

pub(crate) mod shape_compute_types_allowed;

pub(crate) mod shape_environment_language;

pub(crate) mod shape_identifiers;

pub(crate) mod shape_phase_contexts;

pub(crate) mod shape_report_status_counts;

pub(crate) mod shape_environment_images;

pub(crate) mod shape_phase_context;

pub(crate) mod shape_resolved_artifact;

pub(crate) mod shape_resolved_secondary_artifacts;

pub(crate) mod shape_environment_image;

pub(crate) mod shape_image_versions;

