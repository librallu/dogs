use std::cell::RefCell;
use std::time::{SystemTime};

extern crate human_format;



/**
 * Defines a metric to be displayed
 */
pub enum Metric {
    Empty,
    LargeNumber(f64),
    Time(f32),
    Text(String),
    Int(i64),  // 1000000 -> "1.000.000"
}

pub fn metric_to_string(m:&Metric) -> String {
    match m {
        Metric::Empty => {
            return String::new();
        },
        Metric::LargeNumber(n) => {
            return format!("{}", human_format::Formatter::new()
                .with_decimals(1)
                .format(*n));
        },
        Metric::Time(n) => {
            return format!("  {:.3}", n);
        },
        Metric::Text(s) => {
            return s.clone();
        },
        Metric::Int(n) => {
            let mut res:String = String::new();
            let mut tmp:i64 = *n;
            if tmp < 0 {
                res += "-";
                tmp *= -1;
            }
            let mut l:Vec<String> = Vec::new();
            while tmp >= 1000 {
                l.push(format!("{:0>3}",(tmp % 1000)));
                tmp /= 1000;
            }
            res += format!("{}", tmp).as_str();
            for e in l.iter().rev() {
                res += format!(".{}", e).as_str();
            }
            return res;
        },
    }
}


/**
 * Implements a logger.
 * It allows components to register headers and update values to display
 */
pub struct MetricLogger {
    headers: RefCell<Vec<String>>,  // maintains header order
    metrics: RefCell<Vec<Metric>>, // maintains every up-to-date entry
    t_start: SystemTime,
}

impl MetricLogger {

    pub fn new() -> Self {
        let mut headers = Vec::new();
        let mut metrics = Vec::new();
        headers.push(format!("{:<15}"," time (s)"));
        metrics.push(Metric::Empty);
        Self {
            headers: RefCell::new(headers),
            metrics: RefCell::new(metrics),
            t_start: SystemTime::now(),
        }
    }

    /**
     * requests metric information from all the registered components and display the metrics
     */
    pub fn request_logging(&self) {
        // display other metrics
        let headers = self.headers.borrow();
        let mut metrics = self.metrics.borrow_mut();
        // add time to metrics
        metrics[0] = Metric::Time(self.t_start.elapsed().unwrap().as_secs_f32());
        // for each entry in the metrics array, display it
        for (i,e) in metrics.iter().enumerate() {
            let s = metric_to_string(e);
            print!("{}{}", s, " ".repeat(headers[i].len()-s.len()));
        }
        println!("");
    }

    /**
     * adds some headers to reference metrics to be later logged.
     * Returns indices of the metric headers.
     */
    pub fn register_headers(&self, headers:Vec<String>) -> Vec<usize> {
        let mut res = Vec::new();
        let mut headers_ref = self.headers.borrow_mut();
        let mut metrics_ref = self.metrics.borrow_mut();
        for h in headers {
            res.push(headers_ref.len());
            headers_ref.push(h);
            metrics_ref.push(Metric::Empty);
        }
        return res;
    }

    /**
     * updates a metric for further logging
     */
    pub fn update_metric(&self, id: usize, metric: Metric) {
        let mut metrics = self.metrics.borrow_mut();
        metrics[id] = metric;
    }


    /**
     * Displays metric headers. Should be called at the beginning of the search
     */
    pub fn display_headers(&self) {
        let mut size:usize = 0;
        for h in self.headers.borrow().iter() {
            print!("{}", h);
            size += h.len();
        }
        println!("");
        println!("{}",("-".repeat(size)));
    }
}