// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_search_devices_output_next_token(input: &crate::output::SearchDevicesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_search_jobs_output_next_token(input: &crate::output::SearchJobsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_search_quantum_tasks_output_next_token(input: &crate::output::SearchQuantumTasksOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_search_devices_output_devices(input: crate::output::SearchDevicesOutput) -> std::option::Option<std::vec::Vec<crate::model::DeviceSummary>> {
                    let input = match input.devices {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_search_jobs_output_jobs(input: crate::output::SearchJobsOutput) -> std::option::Option<std::vec::Vec<crate::model::JobSummary>> {
                    let input = match input.jobs {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_search_quantum_tasks_output_quantum_tasks(input: crate::output::SearchQuantumTasksOutput) -> std::option::Option<std::vec::Vec<crate::model::QuantumTaskSummary>> {
                    let input = match input.quantum_tasks {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

