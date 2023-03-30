// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_extended_source_server;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_replication_configuration_template;

pub(crate) mod shape_delete_job;

pub(crate) mod shape_delete_recovery_instance;

pub(crate) mod shape_delete_replication_configuration_template;

pub(crate) mod shape_delete_source_server;

pub(crate) mod shape_describe_job_log_items;

pub(crate) mod shape_describe_jobs;

pub(crate) mod shape_describe_recovery_instances;

pub(crate) mod shape_describe_recovery_snapshots;

pub(crate) mod shape_describe_replication_configuration_templates;

pub(crate) mod shape_describe_source_servers;

pub(crate) mod shape_disconnect_recovery_instance;

pub(crate) mod shape_disconnect_source_server;

pub(crate) mod shape_get_failback_replication_configuration;

pub(crate) mod shape_get_launch_configuration;

pub(crate) mod shape_get_replication_configuration;

pub(crate) mod shape_initialize_service;

pub(crate) mod shape_list_extensible_source_servers;

pub(crate) mod shape_list_staging_accounts;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_retry_data_replication;

pub(crate) mod shape_reverse_replication;

pub(crate) mod shape_start_failback_launch;

pub(crate) mod shape_start_recovery;

pub(crate) mod shape_start_replication;

pub(crate) mod shape_stop_failback;

pub(crate) mod shape_stop_replication;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_terminate_recovery_instances;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_failback_replication_configuration;

pub(crate) mod shape_update_launch_configuration;

pub(crate) mod shape_update_replication_configuration;

pub(crate) mod shape_update_replication_configuration_template;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_extended_source_server_input;

pub(crate) mod shape_create_replication_configuration_template_input;

pub(crate) mod shape_delete_job_input;

pub(crate) mod shape_delete_recovery_instance_input;

pub(crate) mod shape_delete_replication_configuration_template_input;

pub(crate) mod shape_delete_source_server_input;

pub(crate) mod shape_describe_job_log_items_input;

pub(crate) mod shape_describe_jobs_input;

pub(crate) mod shape_describe_recovery_instances_input;

pub(crate) mod shape_describe_recovery_snapshots_input;

pub(crate) mod shape_describe_replication_configuration_templates_input;

pub(crate) mod shape_describe_source_servers_input;

pub(crate) mod shape_disconnect_recovery_instance_input;

pub(crate) mod shape_disconnect_source_server_input;

pub(crate) mod shape_get_failback_replication_configuration_input;

pub(crate) mod shape_get_launch_configuration_input;

pub(crate) mod shape_get_replication_configuration_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_extensible_source_servers_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_retry_data_replication_input;

pub(crate) mod shape_reverse_replication_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_failback_launch_input;

pub(crate) mod shape_start_recovery_input;

pub(crate) mod shape_start_replication_input;

pub(crate) mod shape_stop_failback_input;

pub(crate) mod shape_stop_replication_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_terminate_recovery_instances_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_uninitialized_account_exception;

pub(crate) mod shape_update_failback_replication_configuration_input;

pub(crate) mod shape_update_launch_configuration_input;

pub(crate) mod shape_update_replication_configuration_input;

pub(crate) mod shape_update_replication_configuration_template_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_accounts;

pub(crate) mod shape_data_replication_info;

pub(crate) mod shape_describe_jobs_request_filters;

pub(crate) mod shape_describe_recovery_instances_items;

pub(crate) mod shape_describe_recovery_instances_request_filters;

pub(crate) mod shape_describe_recovery_snapshots_request_filters;

pub(crate) mod shape_describe_source_servers_request_filters;

pub(crate) mod shape_job;

pub(crate) mod shape_job_logs;

pub(crate) mod shape_jobs_list;

pub(crate) mod shape_licensing;

pub(crate) mod shape_life_cycle;

pub(crate) mod shape_pit_policy;

pub(crate) mod shape_pit_policy_rule;

pub(crate) mod shape_recovery_snapshots_list;

pub(crate) mod shape_replication_configuration_replicated_disk;

pub(crate) mod shape_replication_configuration_replicated_disks;

pub(crate) mod shape_replication_configuration_templates;

pub(crate) mod shape_replication_servers_security_groups_i_ds;

pub(crate) mod shape_source_cloud_properties;

pub(crate) mod shape_source_properties;

pub(crate) mod shape_source_server;

pub(crate) mod shape_source_servers_list;

pub(crate) mod shape_staging_area;

pub(crate) mod shape_staging_source_servers_list;

pub(crate) mod shape_start_recovery_request_source_server;

pub(crate) mod shape_tags_map;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_account;

pub(crate) mod shape_cpus;

pub(crate) mod shape_data_replication_error;

pub(crate) mod shape_data_replication_info_replicated_disks;

pub(crate) mod shape_data_replication_initiation;

pub(crate) mod shape_disks;

pub(crate) mod shape_identification_hints;

pub(crate) mod shape_job_log;

pub(crate) mod shape_life_cycle_last_launch;

pub(crate) mod shape_network_interfaces;

pub(crate) mod shape_os;

pub(crate) mod shape_participating_servers;

pub(crate) mod shape_recovery_instance;

pub(crate) mod shape_recovery_snapshot;

pub(crate) mod shape_replication_configuration_template;

pub(crate) mod shape_staging_source_server;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_cpu;

pub(crate) mod shape_data_replication_info_replicated_disk;

pub(crate) mod shape_data_replication_initiation_steps;

pub(crate) mod shape_disk;

pub(crate) mod shape_ebs_snapshots_list;

pub(crate) mod shape_job_log_event_data;

pub(crate) mod shape_life_cycle_last_launch_initiated;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_participating_server;

pub(crate) mod shape_recovery_instance_data_replication_info;

pub(crate) mod shape_recovery_instance_failback;

pub(crate) mod shape_recovery_instance_properties;

pub(crate) mod shape_conversion_properties;

pub(crate) mod shape_data_replication_initiation_step;

pub(crate) mod shape_i_ps_list;

pub(crate) mod shape_recovery_instance_data_replication_error;

pub(crate) mod shape_recovery_instance_data_replication_info_replicated_disks;

pub(crate) mod shape_recovery_instance_data_replication_initiation;

pub(crate) mod shape_recovery_instance_disks;

pub(crate) mod shape_recovery_instance_data_replication_info_replicated_disk;

pub(crate) mod shape_recovery_instance_data_replication_initiation_steps;

pub(crate) mod shape_recovery_instance_disk;

pub(crate) mod shape_volume_to_conversion_map;

pub(crate) mod shape_volume_to_size_map;

pub(crate) mod shape_conversion_map;

pub(crate) mod shape_recovery_instance_data_replication_initiation_step;

