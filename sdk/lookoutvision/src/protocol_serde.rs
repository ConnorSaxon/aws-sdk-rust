// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_dataset;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_model;

pub(crate) mod shape_create_project;

pub(crate) mod shape_delete_dataset;

pub(crate) mod shape_delete_model;

pub(crate) mod shape_delete_project;

pub(crate) mod shape_describe_dataset;

pub(crate) mod shape_describe_model;

pub(crate) mod shape_describe_model_packaging_job;

pub(crate) mod shape_describe_project;

pub(crate) mod shape_detect_anomalies;

pub(crate) mod shape_detect_anomalies_input;

pub(crate) mod shape_list_dataset_entries;

pub(crate) mod shape_list_model_packaging_jobs;

pub(crate) mod shape_list_models;

pub(crate) mod shape_list_projects;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_model;

pub(crate) mod shape_start_model_packaging_job;

pub(crate) mod shape_stop_model;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_dataset_entries;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_dataset_input;

pub(crate) mod shape_create_model_input;

pub(crate) mod shape_create_project_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_model_input;

pub(crate) mod shape_start_model_packaging_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_dataset_entries_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_dataset_description;

pub(crate) mod shape_dataset_entry_list;

pub(crate) mod shape_dataset_metadata;

pub(crate) mod shape_dataset_source;

pub(crate) mod shape_detect_anomaly_result;

pub(crate) mod shape_model_description;

pub(crate) mod shape_model_metadata;

pub(crate) mod shape_model_metadata_list;

pub(crate) mod shape_model_packaging_configuration;

pub(crate) mod shape_model_packaging_description;

pub(crate) mod shape_model_packaging_jobs_list;

pub(crate) mod shape_output_config;

pub(crate) mod shape_project_description;

pub(crate) mod shape_project_metadata;

pub(crate) mod shape_project_metadata_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_anomaly_list;

pub(crate) mod shape_dataset_ground_truth_manifest;

pub(crate) mod shape_dataset_image_stats;

pub(crate) mod shape_dataset_metadata_list;

pub(crate) mod shape_greengrass_configuration;

pub(crate) mod shape_image_source;

pub(crate) mod shape_model_packaging_job_metadata;

pub(crate) mod shape_model_packaging_output_details;

pub(crate) mod shape_model_performance;

pub(crate) mod shape_output_s3_object;

pub(crate) mod shape_s3_location;

pub(crate) mod shape_anomaly;

pub(crate) mod shape_greengrass_output_details;

pub(crate) mod shape_input_s3_object;

pub(crate) mod shape_target_platform;

pub(crate) mod shape_pixel_anomaly;

