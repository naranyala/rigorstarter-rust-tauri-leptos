use chrono::Datelike;
use leptos::prelude::*;

const MONTH_NAMES: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const DAY_NAMES: &[&str] = &["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DateInfo {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone, Copy)]
pub struct CalendarViewModel {
    pub state: RwSignal<CalendarState>,
}

#[derive(PartialEq, Clone)]
pub struct CalendarState {
    pub current_year: i32,
    pub current_month: u32,
    pub selected_date: Option<DateInfo>,
    pub today: DateInfo,
}

impl CalendarViewModel {
    #[allow(dead_code)]
    pub fn new(today: DateInfo) -> Self {
        let state = RwSignal::new(CalendarState {
            current_year: today.year,
            current_month: today.month,
            selected_date: None,
            today,
        });
        Self { state }
    }

    pub fn prev_month(&self) {
        self.state.update(|s| {
            if s.current_month == 1 {
                s.current_month = 12;
                s.current_year -= 1;
            } else {
                s.current_month -= 1;
            }
        });
    }

    pub fn next_month(&self) {
        self.state.update(|s| {
            if s.current_month == 12 {
                s.current_month = 1;
                s.current_year += 1;
            } else {
                s.current_month += 1;
            }
        });
    }

    pub fn select_date(&self, day: u32) {
        let (year, month) = {
            let s = self.state.get();
            (s.current_year, s.current_month)
        };
        self.state.update(|s| {
            s.selected_date = Some(DateInfo { year, month, day });
        });
    }

    pub fn is_selected(&self, day: u32) -> bool {
        let s = self.state.get();
        s.selected_date
            .is_some_and(|d| d.year == s.current_year && d.month == s.current_month && d.day == day)
    }

    pub fn is_today(&self, day: u32) -> bool {
        let s = self.state.get();
        day == s.today.day && s.current_month == s.today.month && s.current_year == s.today.year
    }
}

// Utility functions (same as your Dioxus version)
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn first_day_of_month(year: i32, month: u32) -> u32 {
    let m: i32 = if month < 3 {
        month as i32 + 12
    } else {
        month as i32
    };
    let y: i32 = if month < 3 { year - 1 } else { year };
    let k: i32 = 1;
    let w: i32 = (k + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    ((w + 6) % 7) as u32
}

pub fn get_today() -> DateInfo {
    use chrono::Local;
    let now = Local::now().date_naive();
    DateInfo {
        year: now.year(),
        month: now.month(),
        day: now.day(),
    }
}

pub fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut total = day;
    for m in 1..month {
        total += days_in_month(year, m);
    }
    total
}

#[component]
pub fn Calendar() -> impl IntoView {
    let today = get_today();
    let vm = CalendarViewModel::new(today);

    let month = Memo::new(move |_| vm.state.get().current_month);
    let year = Memo::new(move |_| vm.state.get().current_year);
    let selected = Memo::new(move |_| vm.state.get().selected_date);

    let days = Memo::new(move |_| days_in_month(year.get(), month.get()));
    let first_day = Memo::new(move |_| first_day_of_month(year.get(), month.get()));

    view! {
        <div style="display: flex; flex-direction: column; align-items: center; padding: 20px; font-family: sans-serif; max-width: 500px; margin: 0 auto;">
            <div style="display: flex; align-items: center; justify-content: space-between; width: 100%; margin-bottom: 20px;">
                <button on:click=move |_| {
                    let vm_clone = vm;
                    vm_clone.prev_month()
                }
                    style="background: none; border: 1px solid #ccc; border-radius: 4px; padding: 8px 12px; cursor: pointer; font-size: 16px;">
                    "◀"
                </button>

                <h2 style="margin: 0; font-size: 20px;">
                    {move || format!("{} {}", MONTH_NAMES[(month.get() - 1) as usize], year.get())}
                </h2>

                <button on:click=move |_| {
                    let vm_clone = vm;
                    vm_clone.next_month()
                }
                    style="background: none; border: 1px solid #ccc; border-radius: 4px; padding: 8px 12px; cursor: pointer; font-size: 16px;">
                    "▶"
                </button>
            </div>

            <div style="display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; width: 100%;">
                {DAY_NAMES.iter().map(|day_name| view! {
                    <div style="text-align: center; font-weight: bold; padding: 8px; color: #666;">
                        {day_name.to_string()}
                    </div>
                }).collect::<Vec<_>>()}

                {move || (0..first_day.get()).map(|_| view! {
                    <div style="padding: 8px;"></div>
                }).collect::<Vec<_>>()}

                {move || (1..=days.get()).map(|day| {
                    let is_selected = vm.is_selected(day);
                    let is_today = vm.is_today(day);
                    let vm_clone = vm;
                    view! {
                        <DayCell
                            day=day
                            is_selected=is_selected
                            is_today=is_today
                            on_click=Callback::new(move |_| vm_clone.select_date(day))
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>

            {move || selected.get().map(|date| view! {
                <div style="margin-top: 24px; padding: 16px; background-color: #f3f4f6; border-radius: 8px; width: 100%; text-align: center;">
                    <h3 style="margin: 0 0 8px 0;">"Selected Date"</h3>
                    <p style="margin: 0; font-size: 18px;">
                        {format!("{} {}, {}", MONTH_NAMES[(date.month - 1) as usize], date.day, date.year)}
                    </p>
                    <p style="margin: 4px 0 0 0; color: #666;">
                        {format!("Day of year: {}", day_of_year(date.year, date.month, date.day))}
                    </p>
                </div>
            }).into_any()}
        </div>
    }
}

#[component]
fn DayCell(day: u32, is_selected: bool, is_today: bool, on_click: Callback<()>) -> impl IntoView {
    let bg_color = if is_selected {
        "#3b82f6"
    } else if is_today {
        "#e0e7ff"
    } else {
        "transparent"
    };
    let text_color = if is_selected { "white" } else { "#333" };

    view! {
        <div
            on:click=move |_| on_click.run(())
            style=format!("text-align: center; padding: 10px 8px; border-radius: 4px; cursor: pointer; transition: background 0.2s; background-color: {}; color: {};", bg_color, text_color)
        >
            {day}
        </div>
    }
}
