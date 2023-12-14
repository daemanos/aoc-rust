use chrono::{Datelike, FixedOffset, Utc, NaiveDate, NaiveDateTime};
use std::cmp;

const DECEMBER: u32 = 12;
const FIRST_PUZZLE_DAY: Day = 1;
const LAST_PUZZLE_DAY: Day = 25;
const TIMEZONE: FixedOffset = FixedOffset::west_opt(5 * 3600).unwrap();

pub struct AocClient {
    token: String,
}

pub type AocResult<T> = Result<T, AocError>;
pub enum AocError {
    InvalidDate(Day, Year),
    Locked(Day, Year),
}

pub type Year = i32;
pub type Day = u32;
pub type Part = u8;

pub struct PuzzleIdx {
    pub year: Year,
    pub day: Day,
    pub part: Part,

    unlock_datetime: NaiveDateTime,
}

impl PuzzleIdx {
    pub fn new(
        year: Option<Year>,
        day: Option<Day>,
        part: Part,
    ) -> AocResult<Self> {
        let year = year.unwrap_or_else(current_puzzle_year);
        let day = day.unwrap_or_else(|| current_puzzle_day(year));

        let unlock_datetime = NaiveDate::from_ymd_opt(year, DECEMBER, day)
            .ok_or(AocError::InvalidDate(day, year))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        Ok(Self { year, day, part, unlock_datetime })
    }

    pub fn unlocked(&self) -> bool {
        let now = Utc::now().with_timezone(&TIMEZONE).naive_utc();
        now.signed_duration_since(self.unlock_datetime)
            .num_milliseconds() >= 0
    }

    pub fn ensure_unlocked(&self) -> AocResult<()> {
        if self.unlocked() {
            Ok(())
        } else {
            Err(AocError::Locked(self.day, self.year))
        }
    }
}

pub fn current_puzzle_year() -> Year {
    let now = Utc::now().with_timezone(&TIMEZONE).date_naive();

    if now.month() < DECEMBER {
        now.year() - 1
    } else {
        now.year()
    }
}

pub fn current_puzzle_day(year: Year) -> Day {
    let now = Utc::now().with_timezone(&TIMEZONE).date_naive();

    if year == now.year() && now.month() == DECEMBER {
        cmp::min(LAST_PUZZLE_DAY, now.day())
    } else if year < now.year() {
        LAST_PUZZLE_DAY
    } else {
        FIRST_PUZZLE_DAY
    }
}
