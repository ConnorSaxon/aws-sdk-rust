// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRobotApplication`](crate::client::fluent_builders::CreateRobotApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateRobotApplication::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateRobotApplication::set_name): <p>The name of the robot application.</p>
    ///   - [`sources(Vec<SourceConfig>)`](crate::client::fluent_builders::CreateRobotApplication::sources) / [`set_sources(Option<Vec<SourceConfig>>)`](crate::client::fluent_builders::CreateRobotApplication::set_sources): <p>The sources of the robot application.</p>
    ///   - [`robot_software_suite(RobotSoftwareSuite)`](crate::client::fluent_builders::CreateRobotApplication::robot_software_suite) / [`set_robot_software_suite(Option<RobotSoftwareSuite>)`](crate::client::fluent_builders::CreateRobotApplication::set_robot_software_suite): <p>The robot software suite (ROS distribuition) used by the robot application.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateRobotApplication::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateRobotApplication::set_tags): <p>A map that contains tag keys and tag values that are attached to the robot application.</p>
    ///   - [`environment(Environment)`](crate::client::fluent_builders::CreateRobotApplication::environment) / [`set_environment(Option<Environment>)`](crate::client::fluent_builders::CreateRobotApplication::set_environment): <p>The object that contains that URI of the Docker image that you use for your robot application.</p>
                            /// - On success, responds with [`CreateRobotApplicationOutput`](crate::output::CreateRobotApplicationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateRobotApplicationOutput::arn): <p>The Amazon Resource Name (ARN) of the robot application.</p>
    ///   - [`name(Option<String>)`](crate::output::CreateRobotApplicationOutput::name): <p>The name of the robot application.</p>
    ///   - [`version(Option<String>)`](crate::output::CreateRobotApplicationOutput::version): <p>The version of the robot application.</p>
    ///   - [`sources(Option<Vec<Source>>)`](crate::output::CreateRobotApplicationOutput::sources): <p>The sources of the robot application.</p>
    ///   - [`robot_software_suite(Option<RobotSoftwareSuite>)`](crate::output::CreateRobotApplicationOutput::robot_software_suite): <p>The robot software suite (ROS distribution) used by the robot application.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::output::CreateRobotApplicationOutput::last_updated_at): <p>The time, in milliseconds since the epoch, when the robot application was last updated.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::CreateRobotApplicationOutput::revision_id): <p>The revision id of the robot application.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateRobotApplicationOutput::tags): <p>The list of all tags added to the robot application.</p>
    ///   - [`environment(Option<Environment>)`](crate::output::CreateRobotApplicationOutput::environment): <p>An object that contains the Docker image URI used to a create your robot application.</p>
                            /// - On failure, responds with [`SdkError<CreateRobotApplicationError>`](crate::error::CreateRobotApplicationError)
    pub fn create_robot_application(&self) -> crate::client::fluent_builders::CreateRobotApplication {
                                crate::client::fluent_builders::CreateRobotApplication::new(self.handle.clone())
                            }
}

