// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeRobot`](crate::client::fluent_builders::DescribeRobot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`robot(impl Into<String>)`](crate::client::fluent_builders::DescribeRobot::robot) / [`set_robot(Option<String>)`](crate::client::fluent_builders::DescribeRobot::set_robot): <p>The Amazon Resource Name (ARN) of the robot to be described.</p>
                            /// - On success, responds with [`DescribeRobotOutput`](crate::output::DescribeRobotOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DescribeRobotOutput::arn): <p>The Amazon Resource Name (ARN) of the robot.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeRobotOutput::name): <p>The name of the robot.</p>
    ///   - [`fleet_arn(Option<String>)`](crate::output::DescribeRobotOutput::fleet_arn): <p>The Amazon Resource Name (ARN) of the fleet.</p>
    ///   - [`status(Option<RobotStatus>)`](crate::output::DescribeRobotOutput::status): <p>The status of the fleet.</p>
    ///   - [`greengrass_group_id(Option<String>)`](crate::output::DescribeRobotOutput::greengrass_group_id): <p>The Greengrass group id.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::DescribeRobotOutput::created_at): <p>The time, in milliseconds since the epoch, when the robot was created.</p>
    ///   - [`architecture(Option<Architecture>)`](crate::output::DescribeRobotOutput::architecture): <p>The target architecture of the robot application.</p>
    ///   - [`last_deployment_job(Option<String>)`](crate::output::DescribeRobotOutput::last_deployment_job): <p>The Amazon Resource Name (ARN) of the last deployment job.</p>
    ///   - [`last_deployment_time(Option<DateTime>)`](crate::output::DescribeRobotOutput::last_deployment_time): <p>The time of the last deployment job.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribeRobotOutput::tags): <p>The list of all tags added to the specified robot.</p>
                            /// - On failure, responds with [`SdkError<DescribeRobotError>`](crate::error::DescribeRobotError)
    #[deprecated(note = "Support for the AWS RoboMaker application deployment feature has ended. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/fleets.html.")]
    pub fn describe_robot(&self) -> crate::client::fluent_builders::DescribeRobot {
                                crate::client::fluent_builders::DescribeRobot::new(self.handle.clone())
                            }
}

