// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSimulationApplication`](crate::client::fluent_builders::UpdateSimulationApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application(impl Into<String>)`](crate::client::fluent_builders::UpdateSimulationApplication::application) / [`set_application(Option<String>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_application): <p>The application information for the simulation application.</p>
    ///   - [`sources(Vec<SourceConfig>)`](crate::client::fluent_builders::UpdateSimulationApplication::sources) / [`set_sources(Option<Vec<SourceConfig>>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_sources): <p>The sources of the simulation application.</p>
    ///   - [`simulation_software_suite(SimulationSoftwareSuite)`](crate::client::fluent_builders::UpdateSimulationApplication::simulation_software_suite) / [`set_simulation_software_suite(Option<SimulationSoftwareSuite>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_simulation_software_suite): <p>The simulation software suite used by the simulation application.</p>
    ///   - [`robot_software_suite(RobotSoftwareSuite)`](crate::client::fluent_builders::UpdateSimulationApplication::robot_software_suite) / [`set_robot_software_suite(Option<RobotSoftwareSuite>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_robot_software_suite): <p>Information about the robot software suite (ROS distribution).</p>
    ///   - [`rendering_engine(RenderingEngine)`](crate::client::fluent_builders::UpdateSimulationApplication::rendering_engine) / [`set_rendering_engine(Option<RenderingEngine>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_rendering_engine): <p>The rendering engine for the simulation application.</p>
    ///   - [`current_revision_id(impl Into<String>)`](crate::client::fluent_builders::UpdateSimulationApplication::current_revision_id) / [`set_current_revision_id(Option<String>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_current_revision_id): <p>The revision id for the robot application.</p>
    ///   - [`environment(Environment)`](crate::client::fluent_builders::UpdateSimulationApplication::environment) / [`set_environment(Option<Environment>)`](crate::client::fluent_builders::UpdateSimulationApplication::set_environment): <p>The object that contains the Docker image URI for your simulation application.</p>
                            /// - On success, responds with [`UpdateSimulationApplicationOutput`](crate::output::UpdateSimulationApplicationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::UpdateSimulationApplicationOutput::arn): <p>The Amazon Resource Name (ARN) of the updated simulation application.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateSimulationApplicationOutput::name): <p>The name of the simulation application.</p>
    ///   - [`version(Option<String>)`](crate::output::UpdateSimulationApplicationOutput::version): <p>The version of the robot application.</p>
    ///   - [`sources(Option<Vec<Source>>)`](crate::output::UpdateSimulationApplicationOutput::sources): <p>The sources of the simulation application.</p>
    ///   - [`simulation_software_suite(Option<SimulationSoftwareSuite>)`](crate::output::UpdateSimulationApplicationOutput::simulation_software_suite): <p>The simulation software suite used by the simulation application.</p>
    ///   - [`robot_software_suite(Option<RobotSoftwareSuite>)`](crate::output::UpdateSimulationApplicationOutput::robot_software_suite): <p>Information about the robot software suite (ROS distribution).</p>
    ///   - [`rendering_engine(Option<RenderingEngine>)`](crate::output::UpdateSimulationApplicationOutput::rendering_engine): <p>The rendering engine for the simulation application.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::output::UpdateSimulationApplicationOutput::last_updated_at): <p>The time, in milliseconds since the epoch, when the simulation application was last updated.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::UpdateSimulationApplicationOutput::revision_id): <p>The revision id of the simulation application.</p>
    ///   - [`environment(Option<Environment>)`](crate::output::UpdateSimulationApplicationOutput::environment): <p>The object that contains the Docker image URI used for your simulation application.</p>
                            /// - On failure, responds with [`SdkError<UpdateSimulationApplicationError>`](crate::error::UpdateSimulationApplicationError)
    pub fn update_simulation_application(&self) -> crate::client::fluent_builders::UpdateSimulationApplication {
                                crate::client::fluent_builders::UpdateSimulationApplication::new(self.handle.clone())
                            }
}

