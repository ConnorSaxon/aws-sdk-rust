// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_access_point;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    crate::rest_xml_wrapped_errors::parse_error_metadata(response.body().as_ref())
}

pub(crate) mod shape_create_access_point_for_object_lambda;

pub(crate) mod shape_create_bucket;

pub fn rest_xml_unset_payload() -> std::vec::Vec<u8> {
                    Vec::new()
                }

pub(crate) mod shape_create_bucket_input;

pub(crate) mod shape_create_job;

pub(crate) mod shape_create_multi_region_access_point;

pub(crate) mod shape_delete_access_point;

pub(crate) mod shape_delete_access_point_for_object_lambda;

pub(crate) mod shape_delete_access_point_policy;

pub(crate) mod shape_delete_access_point_policy_for_object_lambda;

pub(crate) mod shape_delete_bucket;

pub(crate) mod shape_delete_bucket_lifecycle_configuration;

pub(crate) mod shape_delete_bucket_policy;

pub(crate) mod shape_delete_bucket_tagging;

pub(crate) mod shape_delete_job_tagging;

pub(crate) mod shape_delete_multi_region_access_point;

pub(crate) mod shape_delete_public_access_block;

pub(crate) mod shape_delete_storage_lens_configuration;

pub(crate) mod shape_delete_storage_lens_configuration_tagging;

pub(crate) mod shape_describe_job;

pub(crate) mod shape_describe_multi_region_access_point_operation;

pub(crate) mod shape_get_access_point;

pub(crate) mod shape_get_access_point_configuration_for_object_lambda;

pub(crate) mod shape_get_access_point_for_object_lambda;

pub(crate) mod shape_get_access_point_policy;

pub(crate) mod shape_get_access_point_policy_for_object_lambda;

pub(crate) mod shape_get_access_point_policy_status;

pub(crate) mod shape_get_access_point_policy_status_for_object_lambda;

pub(crate) mod shape_get_bucket;

pub(crate) mod shape_get_bucket_lifecycle_configuration;

pub(crate) mod shape_get_bucket_policy;

pub(crate) mod shape_get_bucket_tagging;

pub(crate) mod shape_get_bucket_versioning;

pub(crate) mod shape_get_job_tagging;

pub(crate) mod shape_get_multi_region_access_point;

pub(crate) mod shape_get_multi_region_access_point_policy;

pub(crate) mod shape_get_multi_region_access_point_policy_status;

pub(crate) mod shape_get_multi_region_access_point_routes;

pub(crate) mod shape_get_public_access_block;

pub(crate) mod shape_get_storage_lens_configuration;

pub(crate) mod shape_get_storage_lens_configuration_tagging;

pub(crate) mod shape_list_access_points;

pub(crate) mod shape_list_access_points_for_object_lambda;

pub(crate) mod shape_list_jobs;

pub(crate) mod shape_list_multi_region_access_points;

pub(crate) mod shape_list_regional_buckets;

pub(crate) mod shape_list_storage_lens_configurations;

pub(crate) mod shape_put_access_point_configuration_for_object_lambda;

pub(crate) mod shape_put_access_point_policy;

pub(crate) mod shape_put_access_point_policy_for_object_lambda;

pub(crate) mod shape_put_bucket_lifecycle_configuration;

pub(crate) mod shape_put_bucket_lifecycle_configuration_input;

pub(crate) mod shape_put_bucket_policy;

pub(crate) mod shape_put_bucket_tagging;

pub(crate) mod shape_put_bucket_tagging_input;

pub(crate) mod shape_put_bucket_versioning;

pub(crate) mod shape_put_bucket_versioning_input;

pub(crate) mod shape_put_job_tagging;

pub(crate) mod shape_put_multi_region_access_point_policy;

pub(crate) mod shape_put_public_access_block;

pub(crate) mod shape_put_public_access_block_input;

pub(crate) mod shape_put_storage_lens_configuration;

pub(crate) mod shape_put_storage_lens_configuration_tagging;

pub(crate) mod shape_submit_multi_region_access_point_routes;

pub(crate) mod shape_update_job_priority;

pub(crate) mod shape_update_job_status;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_bucket_already_exists;

pub(crate) mod shape_bucket_already_owned_by_you;

pub(crate) mod shape_create_access_point_for_object_lambda_input;

pub(crate) mod shape_create_access_point_input;

pub(crate) mod shape_create_bucket_output;

pub(crate) mod shape_create_job_input;

pub(crate) mod shape_create_multi_region_access_point_input;

pub(crate) mod shape_delete_multi_region_access_point_input;

pub(crate) mod shape_get_public_access_block_output;

pub(crate) mod shape_get_storage_lens_configuration_output;

pub(crate) mod shape_idempotency_exception;

pub(crate) mod shape_internal_service_exception;

pub(crate) mod shape_invalid_next_token_exception;

pub(crate) mod shape_invalid_request_exception;

pub(crate) mod shape_job_status_exception;

pub(crate) mod shape_no_such_public_access_block_configuration;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_access_point_configuration_for_object_lambda_input;

pub(crate) mod shape_put_access_point_policy_for_object_lambda_input;

pub(crate) mod shape_put_access_point_policy_input;

pub(crate) mod shape_put_bucket_policy_input;

pub(crate) mod shape_put_job_tagging_input;

pub(crate) mod shape_put_multi_region_access_point_policy_input;

pub(crate) mod shape_put_storage_lens_configuration_input;

pub(crate) mod shape_put_storage_lens_configuration_tagging_input;

pub(crate) mod shape_submit_multi_region_access_point_routes_input;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_access_point_list;

pub(crate) mod shape_async_operation;

pub(crate) mod shape_create_bucket_configuration;

pub(crate) mod shape_endpoints;

pub(crate) mod shape_job_descriptor;

pub(crate) mod shape_job_list_descriptor_list;

pub(crate) mod shape_job_manifest;

pub(crate) mod shape_job_manifest_generator;

pub(crate) mod shape_job_operation;

pub(crate) mod shape_job_report;

pub(crate) mod shape_lifecycle_configuration;

pub(crate) mod shape_lifecycle_rules;

pub(crate) mod shape_list_storage_lens_configuration_entry;

pub(crate) mod shape_multi_region_access_point_policy_document;

pub(crate) mod shape_multi_region_access_point_report;

pub(crate) mod shape_multi_region_access_point_report_list;

pub(crate) mod shape_multi_region_access_point_route;

pub(crate) mod shape_object_lambda_access_point_list;

pub(crate) mod shape_object_lambda_configuration;

pub(crate) mod shape_policy_status;

pub(crate) mod shape_public_access_block_configuration;

pub(crate) mod shape_regional_bucket_list;

pub(crate) mod shape_route_list;

pub(crate) mod shape_s3_tag;

pub(crate) mod shape_s3_tag_set;

pub(crate) mod shape_storage_lens_configuration;

pub(crate) mod shape_storage_lens_tag;

pub(crate) mod shape_storage_lens_tags;

pub(crate) mod shape_tagging;

pub(crate) mod shape_versioning_configuration;

pub(crate) mod shape_vpc_configuration;

pub(crate) mod shape_access_point;

pub(crate) mod shape_account_level;

pub(crate) mod shape_async_request_parameters;

pub(crate) mod shape_async_response_details;

pub(crate) mod shape_established_multi_region_access_point_policy;

pub(crate) mod shape_exclude;

pub(crate) mod shape_include;

pub(crate) mod shape_job_failure_list;

pub(crate) mod shape_job_list_descriptor;

pub(crate) mod shape_job_manifest_location;

pub(crate) mod shape_job_manifest_spec;

pub(crate) mod shape_job_progress_summary;

pub(crate) mod shape_lambda_invoke_operation;

pub(crate) mod shape_lifecycle_rule;

pub(crate) mod shape_object_lambda_access_point;

pub(crate) mod shape_object_lambda_allowed_features_list;

pub(crate) mod shape_object_lambda_transformation_configuration;

pub(crate) mod shape_object_lambda_transformation_configurations_list;

pub(crate) mod shape_proposed_multi_region_access_point_policy;

pub(crate) mod shape_region;

pub(crate) mod shape_region_report_list;

pub(crate) mod shape_regional_bucket;

pub(crate) mod shape_s3_copy_object_operation;

pub(crate) mod shape_s3_generated_manifest_descriptor;

pub(crate) mod shape_s3_initiate_restore_object_operation;

pub(crate) mod shape_s3_job_manifest_generator;

pub(crate) mod shape_s3_set_object_acl_operation;

pub(crate) mod shape_s3_set_object_legal_hold_operation;

pub(crate) mod shape_s3_set_object_retention_operation;

pub(crate) mod shape_s3_set_object_tagging_operation;

pub(crate) mod shape_storage_lens_aws_org;

pub(crate) mod shape_storage_lens_data_export;

pub(crate) mod shape_abort_incomplete_multipart_upload;

pub(crate) mod shape_activity_metrics;

pub(crate) mod shape_advanced_cost_optimization_metrics;

pub(crate) mod shape_advanced_data_protection_metrics;

pub(crate) mod shape_async_error_details;

pub(crate) mod shape_bucket_level;

pub(crate) mod shape_cloud_watch_metrics;

pub(crate) mod shape_detailed_status_codes_metrics;

pub(crate) mod shape_job_failure;

pub(crate) mod shape_job_manifest_generator_filter;

pub(crate) mod shape_job_timers;

pub(crate) mod shape_lifecycle_expiration;

pub(crate) mod shape_lifecycle_rule_filter;

pub(crate) mod shape_multi_region_access_points_async_response;

pub(crate) mod shape_noncurrent_version_expiration;

pub(crate) mod shape_noncurrent_version_transition;

pub(crate) mod shape_noncurrent_version_transition_list;

pub(crate) mod shape_object_lambda_content_transformation;

pub(crate) mod shape_region_report;

pub(crate) mod shape_s3_access_control_policy;

pub(crate) mod shape_s3_bucket_destination;

pub(crate) mod shape_s3_delete_object_tagging_operation;

pub(crate) mod shape_s3_grant;

pub(crate) mod shape_s3_manifest_output_location;

pub(crate) mod shape_s3_object_lock_legal_hold;

pub(crate) mod shape_s3_object_metadata;

pub(crate) mod shape_s3_replicate_object_operation;

pub(crate) mod shape_s3_retention;

pub(crate) mod shape_transition;

pub(crate) mod shape_transition_list;

pub(crate) mod shape_aws_lambda_transformation;

pub(crate) mod shape_buckets;

pub(crate) mod shape_generated_manifest_encryption;

pub(crate) mod shape_job_manifest_field_list;

pub(crate) mod shape_lifecycle_rule_and_operator;

pub(crate) mod shape_multi_region_access_point_regional_response_list;

pub(crate) mod shape_object_lambda_transformation_configuration_actions_list;

pub(crate) mod shape_prefix_level;

pub(crate) mod shape_region_creation_list;

pub(crate) mod shape_regions;

pub(crate) mod shape_s3_access_control_list;

pub(crate) mod shape_s3_grant_list;

pub(crate) mod shape_s3_grantee;

pub(crate) mod shape_storage_lens_data_export_encryption;

pub(crate) mod shape_multi_region_access_point_regional_response;

pub(crate) mod shape_prefix_level_storage_metrics;

pub(crate) mod shape_replication_status_filter_list;

pub(crate) mod shape_s3_object_owner;

pub(crate) mod shape_s3_user_metadata;

pub(crate) mod shape_ssekms;

pub(crate) mod shape_ssekms_encryption;

pub(crate) mod shape_selection_criteria;

pub(crate) mod shape_sses3;

pub(crate) mod shape_sses3_encryption;

