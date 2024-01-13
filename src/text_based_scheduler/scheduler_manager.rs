use std::io;

pub struct ScheduleManager {
    pub schedules: Vec<String>,
}

impl ScheduleManager {
    pub fn new() -> ScheduleManager {
        ScheduleManager { schedules: Vec::new() }
    }

    fn add_new_schedule(&mut self) {
        println!("Enter your schedule:");
        let mut schedule = String::new();
        io::stdin().read_line(&mut schedule).expect("Failed to read line");
        self.schedules.push(schedule.trim().to_string());
    }

    fn list_schedules(&self) {
        for schedule in &self.schedules {
            println!("{}", schedule);
        }
    }

    pub fn handle_schedule_manager(&mut self) {
        loop {
            println!("Choose your option");
            println!("1. Add new schedule");
            println!("2. List schedules");
            println!("3. Quit");

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            match user_input.trim().parse() {
                Ok(1) => self.add_new_schedule(),
                Ok(2) => self.list_schedules(),
                Ok(3) => {
                    println!("Program finished");
                    break;
                }
                _ => println!("Invalid input!")
            };
        }
    }
}
