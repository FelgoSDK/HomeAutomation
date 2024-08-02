use chrono::{Datelike, Duration, Local};
use slint::{Timer, TimerMode};

use crate::ui::*;

fn get_ordinal_suffix(day: u32) -> &'static str {
    match day {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn format_date(date: chrono::DateTime<Local>) -> String {
    let day = date.day();
    let month = date.format("%b").to_string();
    let suffix = get_ordinal_suffix(day);

    format!("{} {}{}", month, day, suffix).to_string()
}

fn format_time(date: chrono::DateTime<Local>) -> String {
    date.format("%H:%M").to_string()
}

fn day_name(day_shift: i64) -> String {
    let shifted_date = Local::now() + Duration::days(day_shift);
    shifted_date.format("%a").to_string()
}

pub fn run() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new().expect("Cannot create main window!");

    window.on_close_request({
        let window_weak = window.as_weak();
        move || {
            window_weak.unwrap().window().hide().unwrap();
        }
    });

    // handle temperature change
    {
        let timer = Timer::default();
        {
            let window_weak = window.as_weak();
            timer.start(
                TimerMode::Repeated,
                std::time::Duration::from_secs(2),
                move || {
                    let window = window_weak.unwrap();
                    let timer_controller = window.global::<TemperatureController>();

                    let now_temperature = timer_controller.get_now_temperature();
                    let target_temperature = timer_controller.get_temperature();

                    match now_temperature.cmp(&target_temperature) {
                        std::cmp::Ordering::Less => {
                            timer_controller.set_now_temperature(now_temperature + 1);
                        }
                        std::cmp::Ordering::Greater => {
                            timer_controller.set_now_temperature(now_temperature - 1);
                        }
                        _ => {}
                    }
                },
            );
        }

        window.on_temperature_changed(move || {
            timer.restart();
        });
    }

    // provide date time
    let datetime_controller = window.global::<DateTimeController>();
    datetime_controller
        .on_provide_date(|_request_number| slint::SharedString::from(format_date(Local::now())));
    datetime_controller
        .on_provide_time(|_request_number| slint::SharedString::from(format_time(Local::now())));

    // provide weather days
    let weather_controller = window.global::<WeatherController>();
    weather_controller
        .on_day_name(|day_shift| slint::SharedString::from(day_name(day_shift as i64)));

    window.run()
}
