use std::env;

use cronjob::CronJob;

use crate::FILE_CONFIG;

use super::requests::update_domains;

pub fn start_cron() {
    let mut cron = CronJob::new("bora", on_cron);
    get_and_set_cron_pattern(&mut cron);

    cron.start_job();
}

fn on_cron(_name: &str) {
    let config = FILE_CONFIG.lock().unwrap();
    let config = config.get(0).unwrap();
    // map.
    update_domains(config);
}

fn get_and_set_cron_pattern(cron: &mut CronJob) {
    let v = env::var("CRON_PATTERN").expect("$CRON_PATTERN is not set");
    let splitted_cron = v.split(" ");

    for (pos, pattern) in splitted_cron.enumerate() {
        match pos {
            0 => cron.seconds(pattern),
            1 => cron.minutes(pattern),
            2 => cron.hours(pattern),
            3 => cron.day_of_month(pattern),
            4 => cron.month(pattern),
            5 => cron.day_of_week(pattern),
            6 => cron.year(pattern),
            _ => unreachable!("Should never happen."),
        };
    }
}
