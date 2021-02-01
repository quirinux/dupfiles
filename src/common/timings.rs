use std::time;
use log::{debug, trace};
use indicatif::{FormattedDuration};

#[derive(Clone, Copy, Debug)]
pub enum Timer {
    OverAll,
    List,
    Compute,
}

#[derive(Clone, Copy, Debug)]
pub struct StopWatch {
    start_overall: time::Instant,
    start_list: time::Instant,
    start_compute: time::Instant,

    stop_overall: time::Instant,
    stop_list: time::Instant,
    stop_compute: time::Instant,
}

pub fn new() -> StopWatch {
    StopWatch{
        start_overall: time::Instant::now(),
        start_list: time::Instant::now(),
        start_compute: time::Instant::now(),

        stop_overall: time::Instant::now(),
        stop_list: time::Instant::now(),
        stop_compute: time::Instant::now(),
    }
}

impl StopWatch {
    pub fn start(&mut self, timer: Timer) {
        let t = time::Instant::now();
        trace!("starting {:?} {:?}", timer, t);
        match timer {
            Timer::OverAll => self.start_overall = t,
            Timer::List => self.start_list = t,
            Timer::Compute => self.start_compute = t,
        };
    }

    pub fn stop(&mut self, timer: Timer) {
        let t = time::Instant::now();
        trace!("stoping {:?} {:?}", timer, t);
        match timer {
            Timer::OverAll => self.stop_overall = t,
            Timer::List => self.stop_list = t,
            Timer::Compute => self.stop_compute = t,
        };
    }

    pub fn duration(&self, timer: Timer) -> Option<time::Duration> {
        let start = match timer {
            Timer::OverAll => &self.start_overall,
            Timer::List => &self.start_list,
            Timer::Compute => &self.start_compute,
        };

        let stop = match timer {
            Timer::OverAll => &self.stop_overall,
            Timer::List => &self.stop_list,
            Timer::Compute => &self.stop_compute,
        };
        debug!("{:?}", self);
        trace!("duration {:?} start:{:?} stop:{:?} diff:{:?}", timer, start, stop, stop.checked_duration_since(*start));
        stop.checked_duration_since(*start)
    }

    pub fn duration_as_string(&self, timer:Timer) -> String {
        match self.duration(timer) {
            Some(t) => format!("{}", FormattedDuration(t)),
            None => "".to_string(),
        }
    }
}
