mod text_based_scheduler;

use text_based_scheduler::scheduler_manager as sm;

fn main() {
    let mut manager = sm::ScheduleManager::new();
    manager.handle_schedule_manager();
}
