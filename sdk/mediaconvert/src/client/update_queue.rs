// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateQueue`](crate::client::fluent_builders::UpdateQueue) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateQueue::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateQueue::set_description): The new description for the queue, if you are changing it.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateQueue::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateQueue::set_name): The name of the queue that you are modifying.
    ///   - [`reservation_plan_settings(ReservationPlanSettings)`](crate::client::fluent_builders::UpdateQueue::reservation_plan_settings) / [`set_reservation_plan_settings(Option<ReservationPlanSettings>)`](crate::client::fluent_builders::UpdateQueue::set_reservation_plan_settings): The new details of your pricing plan for your reserved queue. When you set up a new pricing plan to replace an expired one, you enter into another 12-month commitment. When you add capacity to your queue by increasing the number of RTS, you extend the term of your commitment to 12 months from when you add capacity. After you make these commitments, you can't cancel them.
    ///   - [`status(QueueStatus)`](crate::client::fluent_builders::UpdateQueue::status) / [`set_status(Option<QueueStatus>)`](crate::client::fluent_builders::UpdateQueue::set_status): Pause or activate a queue by changing its status between ACTIVE and PAUSED. If you pause a queue, jobs in that queue won't begin. Jobs that are running when you pause the queue continue to run until they finish or result in an error.
                            /// - On success, responds with [`UpdateQueueOutput`](crate::output::UpdateQueueOutput) with field(s):
    ///   - [`queue(Option<Queue>)`](crate::output::UpdateQueueOutput::queue): You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don't specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.
                            /// - On failure, responds with [`SdkError<UpdateQueueError>`](crate::error::UpdateQueueError)
    pub fn update_queue(&self) -> crate::client::fluent_builders::UpdateQueue {
                                crate::client::fluent_builders::UpdateQueue::new(self.handle.clone())
                            }
}

