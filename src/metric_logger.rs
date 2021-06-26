use std::cell::RefCell;
use std::time::SystemTime;

/**
 * Defines a metric to be displayed
 */
#[derive(Debug)]
pub enum Metric {
    /// no data
    Empty,
    /// use formatting like 5.4M, 4.7K, etc.
    LargeNumber(f64),
    /// number of seconds (milliseconds precision)
    Time(f32),
    /// standard text
    Text(String),
    /// int pretty print: example: 1000000 -> "1.000.000"
    Int(i64),
}

/**
    produces a String to represent a given metric
*/
pub fn metric_to_string(m:&Metric) -> String {
    match m {
        Metric::Empty => { String::new() },
        Metric::LargeNumber(n) => {
            human_format::Formatter::new()
                .with_decimals(1)
                .format(*n)
        },
        Metric::Time(n) => { format!("  {:.3}", n) },
        Metric::Text(s) => { s.clone() },
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
            res
        },
    }
}


/**
 * Implements a logger.
 * It allows components to register headers and update values to display
 * **workflow:**
 *  1. a component registers to the MetricLogger. It provides the data names and the MetricLogger returns their IDs.
 *  2. a component can update a metric by providing its ID and new value
 *  3. a component can request a display of all metrics
 */
#[derive(Debug)]
pub struct MetricLogger {
    headers: RefCell<Vec<String>>,  // maintains header order
    metrics: RefCell<Vec<Metric>>, // maintains every up-to-date entry
    t_start: SystemTime,
}

impl Default for MetricLogger {
    fn default() -> Self {
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
}

impl MetricLogger {

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
        println!();
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
        res
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
        println!();
        println!("{}",("-".repeat(size)));
    }
}