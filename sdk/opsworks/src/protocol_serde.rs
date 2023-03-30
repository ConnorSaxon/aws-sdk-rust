// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_assign_instance;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_assign_volume;

pub(crate) mod shape_associate_elastic_ip;

pub(crate) mod shape_attach_elastic_load_balancer;

pub(crate) mod shape_clone_stack;

pub(crate) mod shape_create_app;

pub(crate) mod shape_create_deployment;

pub(crate) mod shape_create_instance;

pub(crate) mod shape_create_layer;

pub(crate) mod shape_create_stack;

pub(crate) mod shape_create_user_profile;

pub(crate) mod shape_delete_app;

pub(crate) mod shape_delete_instance;

pub(crate) mod shape_delete_layer;

pub(crate) mod shape_delete_stack;

pub(crate) mod shape_delete_user_profile;

pub(crate) mod shape_deregister_ecs_cluster;

pub(crate) mod shape_deregister_elastic_ip;

pub(crate) mod shape_deregister_instance;

pub(crate) mod shape_deregister_rds_db_instance;

pub(crate) mod shape_deregister_volume;

pub(crate) mod shape_describe_agent_versions;

pub(crate) mod shape_describe_apps;

pub(crate) mod shape_describe_commands;

pub(crate) mod shape_describe_deployments;

pub(crate) mod shape_describe_ecs_clusters;

pub(crate) mod shape_describe_elastic_ips;

pub(crate) mod shape_describe_elastic_load_balancers;

pub(crate) mod shape_describe_instances;

pub(crate) mod shape_describe_layers;

pub(crate) mod shape_describe_load_based_auto_scaling;

pub(crate) mod shape_describe_my_user_profile;

pub(crate) mod shape_describe_operating_systems;

pub(crate) mod shape_describe_permissions;

pub(crate) mod shape_describe_raid_arrays;

pub(crate) mod shape_describe_rds_db_instances;

pub(crate) mod shape_describe_service_errors;

pub(crate) mod shape_describe_stack_provisioning_parameters;

pub(crate) mod shape_describe_stack_summary;

pub(crate) mod shape_describe_stacks;

pub(crate) mod shape_describe_time_based_auto_scaling;

pub(crate) mod shape_describe_user_profiles;

pub(crate) mod shape_describe_volumes;

pub(crate) mod shape_detach_elastic_load_balancer;

pub(crate) mod shape_disassociate_elastic_ip;

pub(crate) mod shape_get_hostname_suggestion;

pub(crate) mod shape_grant_access;

pub(crate) mod shape_list_tags;

pub(crate) mod shape_reboot_instance;

pub(crate) mod shape_register_ecs_cluster;

pub(crate) mod shape_register_elastic_ip;

pub(crate) mod shape_register_instance;

pub(crate) mod shape_register_rds_db_instance;

pub(crate) mod shape_register_volume;

pub(crate) mod shape_set_load_based_auto_scaling;

pub(crate) mod shape_set_permission;

pub(crate) mod shape_set_time_based_auto_scaling;

pub(crate) mod shape_start_instance;

pub(crate) mod shape_start_stack;

pub(crate) mod shape_stop_instance;

pub(crate) mod shape_stop_stack;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_unassign_instance;

pub(crate) mod shape_unassign_volume;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_app;

pub(crate) mod shape_update_elastic_ip;

pub(crate) mod shape_update_instance;

pub(crate) mod shape_update_layer;

pub(crate) mod shape_update_my_user_profile;

pub(crate) mod shape_update_rds_db_instance;

pub(crate) mod shape_update_stack;

pub(crate) mod shape_update_user_profile;

pub(crate) mod shape_update_volume;

pub(crate) mod shape_assign_instance_input;

pub(crate) mod shape_assign_volume_input;

pub(crate) mod shape_associate_elastic_ip_input;

pub(crate) mod shape_attach_elastic_load_balancer_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_clone_stack_input;

pub(crate) mod shape_create_app_input;

pub(crate) mod shape_create_deployment_input;

pub(crate) mod shape_create_instance_input;

pub(crate) mod shape_create_layer_input;

pub(crate) mod shape_create_stack_input;

pub(crate) mod shape_create_user_profile_input;

pub(crate) mod shape_delete_app_input;

pub(crate) mod shape_delete_instance_input;

pub(crate) mod shape_delete_layer_input;

pub(crate) mod shape_delete_stack_input;

pub(crate) mod shape_delete_user_profile_input;

pub(crate) mod shape_deregister_ecs_cluster_input;

pub(crate) mod shape_deregister_elastic_ip_input;

pub(crate) mod shape_deregister_instance_input;

pub(crate) mod shape_deregister_rds_db_instance_input;

pub(crate) mod shape_deregister_volume_input;

pub(crate) mod shape_describe_agent_versions_input;

pub(crate) mod shape_describe_apps_input;

pub(crate) mod shape_describe_commands_input;

pub(crate) mod shape_describe_deployments_input;

pub(crate) mod shape_describe_ecs_clusters_input;

pub(crate) mod shape_describe_elastic_ips_input;

pub(crate) mod shape_describe_elastic_load_balancers_input;

pub(crate) mod shape_describe_instances_input;

pub(crate) mod shape_describe_layers_input;

pub(crate) mod shape_describe_load_based_auto_scaling_input;

pub(crate) mod shape_describe_permissions_input;

pub(crate) mod shape_describe_raid_arrays_input;

pub(crate) mod shape_describe_rds_db_instances_input;

pub(crate) mod shape_describe_service_errors_input;

pub(crate) mod shape_describe_stack_provisioning_parameters_input;

pub(crate) mod shape_describe_stack_summary_input;

pub(crate) mod shape_describe_stacks_input;

pub(crate) mod shape_describe_time_based_auto_scaling_input;

pub(crate) mod shape_describe_user_profiles_input;

pub(crate) mod shape_describe_volumes_input;

pub(crate) mod shape_detach_elastic_load_balancer_input;

pub(crate) mod shape_disassociate_elastic_ip_input;

pub(crate) mod shape_get_hostname_suggestion_input;

pub(crate) mod shape_grant_access_input;

pub(crate) mod shape_list_tags_input;

pub(crate) mod shape_reboot_instance_input;

pub(crate) mod shape_register_ecs_cluster_input;

pub(crate) mod shape_register_elastic_ip_input;

pub(crate) mod shape_register_instance_input;

pub(crate) mod shape_register_rds_db_instance_input;

pub(crate) mod shape_register_volume_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_set_load_based_auto_scaling_input;

pub(crate) mod shape_set_permission_input;

pub(crate) mod shape_set_time_based_auto_scaling_input;

pub(crate) mod shape_start_instance_input;

pub(crate) mod shape_start_stack_input;

pub(crate) mod shape_stop_instance_input;

pub(crate) mod shape_stop_stack_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_unassign_instance_input;

pub(crate) mod shape_unassign_volume_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_app_input;

pub(crate) mod shape_update_elastic_ip_input;

pub(crate) mod shape_update_instance_input;

pub(crate) mod shape_update_layer_input;

pub(crate) mod shape_update_my_user_profile_input;

pub(crate) mod shape_update_rds_db_instance_input;

pub(crate) mod shape_update_stack_input;

pub(crate) mod shape_update_user_profile_input;

pub(crate) mod shape_update_volume_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_agent_versions;

pub(crate) mod shape_apps;

pub(crate) mod shape_auto_scaling_thresholds;

pub(crate) mod shape_block_device_mapping;

pub(crate) mod shape_chef_configuration;

pub(crate) mod shape_cloud_watch_logs_configuration;

pub(crate) mod shape_commands;

pub(crate) mod shape_data_source;

pub(crate) mod shape_deployment_command;

pub(crate) mod shape_deployments;

pub(crate) mod shape_ecs_clusters;

pub(crate) mod shape_elastic_ips;

pub(crate) mod shape_elastic_load_balancers;

pub(crate) mod shape_environment_variable;

pub(crate) mod shape_instance_identity;

pub(crate) mod shape_instances;

pub(crate) mod shape_layers;

pub(crate) mod shape_lifecycle_event_configuration;

pub(crate) mod shape_load_based_auto_scaling_configurations;

pub(crate) mod shape_operating_systems;

pub(crate) mod shape_parameters;

pub(crate) mod shape_permissions;

pub(crate) mod shape_raid_arrays;

pub(crate) mod shape_rds_db_instances;

pub(crate) mod shape_recipes;

pub(crate) mod shape_self_user_profile;

pub(crate) mod shape_service_errors;

pub(crate) mod shape_source;

pub(crate) mod shape_ssl_configuration;

pub(crate) mod shape_stack_configuration_manager;

pub(crate) mod shape_stack_summary;

pub(crate) mod shape_stacks;

pub(crate) mod shape_tags;

pub(crate) mod shape_temporary_credential;

pub(crate) mod shape_time_based_auto_scaling_configurations;

pub(crate) mod shape_user_profiles;

pub(crate) mod shape_volume_configuration;

pub(crate) mod shape_volumes;

pub(crate) mod shape_weekly_auto_scaling_schedule;

pub(crate) mod shape_agent_version;

pub(crate) mod shape_app;

pub(crate) mod shape_cloud_watch_logs_log_stream;

pub(crate) mod shape_command;

pub(crate) mod shape_deployment;

pub(crate) mod shape_ebs_block_device;

pub(crate) mod shape_ecs_cluster;

pub(crate) mod shape_elastic_ip;

pub(crate) mod shape_elastic_load_balancer;

pub(crate) mod shape_instance;

pub(crate) mod shape_instances_count;

pub(crate) mod shape_layer;

pub(crate) mod shape_load_based_auto_scaling_configuration;

pub(crate) mod shape_operating_system;

pub(crate) mod shape_permission;

pub(crate) mod shape_raid_array;

pub(crate) mod shape_rds_db_instance;

pub(crate) mod shape_service_error;

pub(crate) mod shape_shutdown_event_configuration;

pub(crate) mod shape_stack;

pub(crate) mod shape_time_based_auto_scaling_configuration;

pub(crate) mod shape_user_profile;

pub(crate) mod shape_volume;

pub(crate) mod shape_app_attributes;

pub(crate) mod shape_block_device_mappings;

pub(crate) mod shape_data_sources;

pub(crate) mod shape_environment_variables;

pub(crate) mod shape_layer_attributes;

pub(crate) mod shape_operating_system_configuration_managers;

pub(crate) mod shape_reported_os;

pub(crate) mod shape_stack_attributes;

pub(crate) mod shape_strings;

pub(crate) mod shape_volume_configurations;

pub(crate) mod shape_cloud_watch_logs_log_streams;

pub(crate) mod shape_daily_auto_scaling_schedule;

pub(crate) mod shape_deployment_command_args;

pub(crate) mod shape_operating_system_configuration_manager;

