use crate::AppConfig;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SECONDS_IN_A_MINUTE:i64 = 60;
pub const SECONDS_IN_AN_HOUR:i64 = 3600;
pub const SECONDS_IN_A_DAY:i64 = 86400;

pub struct DateTime
{
    pub year: u64,
    pub offset: i32,
    pub month: u8,
    pub day: u8,
    pub week: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub full_seconds : i64,
}

impl DateTime
{
    pub fn new( seconds : i64 ) -> Self
    {
        let mut result = DateTime{ year:1970, offset: 0, month: 1, day: 1, week: 3, hour: 0, minute: 0, second: 0, full_seconds: 0 };
        result.p_convert_seconds( seconds );
        result
    }
    pub fn now() -> Self
    {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).expect("시간 계산 오류");
        let seconds = duration.as_secs() as i64; 

        let mut result = DateTime{ year:1970, offset: ( SECONDS_IN_AN_HOUR * 9 ) as i32, month: 1, day: 1, week: 3, hour: 0, minute: 0, second: 0, full_seconds: 0 };
        result.p_convert_seconds( seconds );
        result        
    }
    pub fn get_tick_count() -> u64
    {
        AppConfig::get_tick_count()
    } 
    pub fn add_time( &mut self, seconds: i64 )
    {
        self.p_convert_seconds( self.full_seconds + seconds );
    }  
}

impl DateTime
{

fn p_is_leap_year( &self ) -> bool {
    (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
}

fn p_days_in_month(&self ) -> i64 {
    match self.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if self.p_is_leap_year() { 29 } else { 28 },
        _ => 0,
    }
}

fn p_convert_seconds( &mut self, total_seconds: i64) 
{
    self.year = 1970; // UNIX_EPOCH 기준으로 시작
    self.month = 1;
    self.day = 1;
    let mut seconds = total_seconds + self.offset as i64;
    self.week = ( ( ( seconds / SECONDS_IN_A_DAY ) + 4 ) % 7 ) as u8;

    let mut year_seconds = 0;
    let mut month_seconds = 0;


    self.second = ( seconds % SECONDS_IN_A_MINUTE ) as u8;
    seconds -= self.second as i64;
    self.minute = ( ( seconds % SECONDS_IN_AN_HOUR ) / 60 ) as u8 ;
    seconds -= ( self.minute  ) as i64* 60;
    self.hour = ( ( seconds % SECONDS_IN_A_DAY ) / 60 / 60 ) as u8;
    seconds -= ( self.hour ) as i64 * 60 * 60;
    
    loop
    {
        if self.p_is_leap_year()
        {
            year_seconds = 366 * SECONDS_IN_A_DAY;
        }
        else
        {
            year_seconds = 365 * SECONDS_IN_A_DAY;
        }
        
        if year_seconds < seconds
        {
            self.year += 1;
            seconds -= year_seconds;
        }
        else
        {
            break;
        }
    }

    loop
    {
        month_seconds = self.p_days_in_month() * SECONDS_IN_A_DAY;

        if ( month_seconds - SECONDS_IN_A_DAY ) < seconds
        {
            self.month += 1;
            seconds -= month_seconds;
        }
        else
        {
            break;
        }

    }
    
    self.day += ( seconds / SECONDS_IN_A_DAY ) as u8;


    self.full_seconds = total_seconds;
}

}