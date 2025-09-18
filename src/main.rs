use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct PomoTimer {
    work_duration: Duration,
    short_break: Duration,
    long_break_duration: Duration,
    cycles_until_long_duration: u32,
    current_cycle: u32
}

impl PomoTimer {
    fn new() -> Self {
        Self { 
            work_duration: Duration::from_secs(25 * 60),
            short_break: Duration::from_secs(5 * 60),
            long_break_duration: Duration::from_secs(15 * 60),
            cycles_until_long_duration: 4,
            current_cycle:0,
        }
    }

    fn start(&mut self) {
        println!("🍅 Pomodoro Timer Started!");
        println!("Press ctrl + C to exit at any time. \n");

        loop {
            self.current_cycle += 1;

            println!("work session {} - Focus time!", self.current_cycle);
            self.run_timer(self.work_duration, "Work");

            if self.current_cycle % self.current_cycle == 0 {
                println!("Long break time! You've completed {} work session", self.current_cycle);
                self.run_timer(self.long_break_duration, "Long break");
            } else {
                println!("Short break timer!");
                self.run_timer(self.short_break, "Short break");
            }
        }
    }

    fn run_timer(&self, duration: Duration, session_type: &str) {
        let start_time = Instant::now();
        let total_seconds = duration.as_secs();

        while start_time.elapsed() < duration {
            let elapsed = start_time.elapsed().as_secs();
            let  remaining = total_seconds -  elapsed;

            let minutes = remaining / 60;
            let seconds = remaining % 60;

            print!("\r{}: {:02}:{:02} remaining", session_type, minutes, seconds);
            io::stdout().flush().unwrap();

            thread::sleep(Duration::from_secs(1));
        }

        // timer finished
        println!("\r{} complete", session_type);
        self.play_notification_sound();

        // wait for user input before contiuning
        println!("Press enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn play_notification_sound(&self) {
        print!("\x07");
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let mut timer = PomoTimer::new();
    
    println!("pomo timer xd");
    println!("Config:");
    println!("- Work session 25 min");
    println!("- Short breaks 5 min");
    println!("- Long braks 15 min");
    println!("- Long break after every 4 work sessions\n");

    println!("Press Enter to start your first Pomodoro session...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    timer.start();
}
