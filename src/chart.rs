use chrono::Duration;

use crate::github::Star;

pub struct StarChartData {
    pub data: Vec<(f64, f64)>,
    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],
    pub x_labels: Vec<String>,
    pub y_labels: Vec<String>,
}

impl StarChartData {
    pub fn new(stars: &[Star]) -> Self {
        let n = stars.len();
        let min_date = stars[0].starred_at;
        let max_date = stars[n - 1].starred_at;

        let mut data = Vec::new();

        let mut i = 0;
        let mut count = 0;
        let mut date = min_date;

        while date <= max_date {
            while i < n && stars[i].starred_at.date_naive() == date.date_naive() {
                i += 1;
                count += 1;
            }
            data.push((date.timestamp() as f64, count as f64));
            date += Duration::days(1);
        }

        let y_max = round_up(stars.len() as f64);
        let x_bounds = [data[0].0, data[data.len() - 1].0];
        let y_bounds = [0 as f64, y_max];

        let x_labels = vec![
            min_date.format("%Y-%m-%d").to_string(),
            max_date.format("%Y-%m-%d").to_string(),
        ];
        let y_labels = vec!["0".to_string(), y_max.to_string()];

        StarChartData {
            data,
            x_bounds,
            y_bounds,
            x_labels,
            y_labels,
        }
    }
}

fn round_up(n: f64) -> f64 {
    let mut n = n;
    let mut d = 1.0;

    while n >= 10.0 {
        n /= 10.0;
        d *= 10.0;
    }

    (n.ceil() * d).max(10.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up() {
        assert_eq!(round_up(12.0), 20.0);
        assert_eq!(round_up(123.0), 200.0);
        assert_eq!(round_up(1234.0), 2000.0);
        assert_eq!(round_up(12345.0), 20000.0);
        assert_eq!(round_up(0.0), 10.0);
        assert_eq!(round_up(1.0), 10.0);
        assert_eq!(round_up(9.0), 10.0);
        assert_eq!(round_up(10.0), 10.0);
        assert_eq!(round_up(11.0), 20.0);
        assert_eq!(round_up(99.0), 100.0);
        assert_eq!(round_up(999.0), 1000.0);
        assert_eq!(round_up(9999.0), 10000.0);
        assert_eq!(round_up(10000.0), 10000.0);
        assert_eq!(round_up(10001.0), 20000.0);
    }
}
