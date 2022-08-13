use chrono::{DateTime, NaiveDate};
use serde::Deserialize;

// allow dead code on clone for testing
#[allow(dead_code)]
#[derive(Clone, Deserialize, Debug)]
pub struct InputStat {
    company: String,
    position: String,
    applied_dt: f64,
    rejected_dt: Option<f64>,
    first_interview: Option<f64>,
    offer_dt: Option<f64>,
    offer_amt: Option<f64>,
    accepted: Option<String>,
    referral: Option<String>,
}

/// converts a datetime stored as a float to a datetime
fn f64_to_datetime(date: f64) -> NaiveDate {
    // unwrap is okay in this case because the format forces no decimal
    NaiveDate::parse_from_str(&format!("{}", date), "%Y%m%d").unwrap()
}

pub struct JobStats {
    num_applied: u8,
    num_offers: u8,
    num_rejections_all_types: u8,
    num_rejections_no_first_interview: u8,
    num_first_interviews_i_declined: u8,
    num_no_response_following_application: u8,
    num_first_interviews_taken: u8,
    num_rejection_after_first_interview: u8,
    num_referrals: u8,
    mean_days_between_application_first_interview: f64,
    // median_days_between_application_and_first_interview: f64,
    // shortest_days_between_application_and_first_interview: u8,
    // longest_days_between_application_and_frist_interview: u8,
    mean_days_between_application_and_rejection: f64,
    // median_days_between_application_and_rejection: f64,
    // shortest_days_between_application_and_rejection: u8,
    // longest_days_between_application_and_rejection: u8,
    mean_time_between_first_interview_and_offer: f64,
    // median_time_between_first_interview_and_offer: u8,
    // shortest_time_between_first_interview_and_offer: u8,
    // longest_time_between_first_interview_and_offer: u8,
}

impl JobStats {
    pub fn new(raw_input: Vec<InputStat>) -> Self {
        let num_first_interviews_taken = raw_input
            .clone()
            .into_iter()
            .filter(|ri| ri.first_interview.is_some())
            .collect::<Vec<InputStat>>()
            .len() as u8;

        Self {
            num_applied: raw_input.len() as u8,
            num_offers: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.offer_dt.is_some())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_rejections_all_types: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.rejected_dt.is_some())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_rejections_no_first_interview: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.rejected_dt.is_some() && ri.first_interview.is_none())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_first_interviews_i_declined: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.first_interview.unwrap_or(0.0f64) == -1.00)
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_no_response_following_application: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.rejected_dt.is_none() && ri.first_interview.is_none())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_first_interviews_taken,
            num_rejection_after_first_interview: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.first_interview.is_some() && ri.rejected_dt.is_some())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            num_referrals: raw_input
                .clone()
                .into_iter()
                .filter(|ri| ri.referral.is_some())
                .collect::<Vec<InputStat>>()
                .len() as u8,
            mean_days_between_application_first_interview:
                days_between_application_and_first_interview(&raw_input)
                    .into_iter()
                    .sum::<f64>()
                    / (num_first_interviews_taken as f64),
            mean_days_between_application_and_rejection: days_between_application_and_rejection(
                &raw_input,
            )
            .into_iter()
            .sum::<f64>()
                / (num_first_interviews_taken as f64),
            mean_time_between_first_interview_and_offer: days_between_first_interview_and_offer(
                &raw_input,
            )
            .into_iter()
            .sum::<f64>()
                / (num_first_interviews_taken as f64),
        }
    }
}

fn days_between_first_interview_and_offer(input_stats: &Vec<InputStat>) -> Vec<f64> {
    let mut days_between_first_interview_and_offer = vec![];

    // for stat in input_stats {
    // if stat.offer_dt.is_some() && stat.first_interview.is_some() {
    // days_between_first_interview_and_offer
    // .push(f64_to_datetime(stat.offer_dt.unwrap()) - f64_to_datetime(stat.first_interview.unwrap()));
    // }
    // }

    days_between_first_interview_and_offer
}

/// TODO: This must be converted to dates
fn days_between_application_and_rejection(input_stats: &Vec<InputStat>) -> Vec<f64> {
    let mut days_between_application_and_rejection = vec![];

    for stat in input_stats {
        if stat.rejected_dt.is_some() {
            days_between_application_and_rejection
                .push(stat.rejected_dt.unwrap() - stat.applied_dt);
        }
    }

    days_between_application_and_rejection
}

/// TODO: This must be converted to dates
fn days_between_application_and_first_interview(input_stats: &Vec<InputStat>) -> Vec<f64> {
    let mut days_between_application_and_first_interview = vec![];

    for stat in input_stats {
        if stat.first_interview.is_some() {
            days_between_application_and_first_interview
                .push(stat.first_interview.unwrap() - stat.applied_dt);
        }
    }

    days_between_application_and_first_interview
}

/// Divides 2 floats and outputs the result as a string with a precision of 2
fn percent_with_precision_2(part: f32, whole: f32) -> String {
    format!("{:.2}%", (part / whole) * 100.0f32)
}

impl std::fmt::Display for JobStats {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("|Metric|Value|");
        println!("|--------|-------|");
        println!("|Jobs applied to|{}|", self.num_applied);
        println!(
            "|Count of referrals | {} ({}) |",
            self.num_referrals,
            percent_with_precision_2(self.num_referrals.into(), self.num_applied.into())
        );
        println!(
            "|First interviews taken | {} ({})|",
            self.num_first_interviews_taken,
            percent_with_precision_2(
                self.num_first_interviews_taken.into(),
                self.num_applied.into()
            )
        );
        println!(
            "|First interviews I declined | {} ({})|",
            self.num_first_interviews_i_declined,
            percent_with_precision_2(
                self.num_first_interviews_i_declined.into(),
                self.num_applied.into()
            )
        );
        println!(
            "|Offers |{} ({})|",
            self.num_offers,
            percent_with_precision_2(self.num_offers.into(), self.num_applied.into())
        );
        println!(
            "|Rejections (all types)|{} ({})|",
            self.num_rejections_all_types,
            percent_with_precision_2(
                self.num_rejections_all_types.into(),
                self.num_applied.into()
            )
        );
        println!(
            "|Rejections without first interview|{} ({} of rejections)|",
            self.num_rejections_no_first_interview,
            percent_with_precision_2(
                self.num_rejections_no_first_interview.into(),
                self.num_rejections_all_types.into()
            )
        );
        println!(
            "|Rejection after first interview | {} ({}) |",
            self.num_rejection_after_first_interview,
            percent_with_precision_2(
                self.num_rejection_after_first_interview.into(),
                self.num_rejections_all_types.into()
            )
        );
        println!(
            "|No Response following application | {} ({} of rejections (all types))|",
            self.num_no_response_following_application,
            percent_with_precision_2(
                self.num_no_response_following_application.into(),
                self.num_rejections_all_types.into()
            )
        );
        println!("|----|---|");
        println!(
            "|Mean time (days) between application date and first interview|{}|",
            self.mean_days_between_application_first_interview
        );
        println!("|Median time (days) between application date and first interview |");
        println!("|Shortest time (days) between application date and first interview |");
        println!("|Longest time (days) between application date and first interview |");
        println!("|---|---|");
        println!("|Mean time (days) between application and rejection |");
        println!("|Median time (days) between application and rejection |");
        println!("|Shortest time (days) between application and rejection |");
        println!("|Longest time (days) between application and rejection |");
        println!("|---|---|");
        println!("|Mean time between first interview and rejection |");
        println!("|Median time between first interview and rejection |");
        println!("|Shortest time between first interview and rejection |");
        println!("|Longest time between first interview and rejection |");
        println!("|---|---|");
        println!("|Mean time between first interview and offer|");
        println!("|Median time between first interview and offer|");
        println!("|Shortest time between first interview and offer|");
        println!("|Longest time between first interview and offer|");

        Ok(())
    }
}

/// Finds the mean of a vector of numbers
fn mean(v: &[f32]) -> Option<f32> {
    if v.len() > 0 {
        return Some(v.into_iter().sum::<f32>() / v.len() as f32);
    }
    None
}

/// Given a vector finds the median value
/// if the array is empty return None
fn median(v: &mut [f32]) -> Option<f32> {
    // if its even find the mean of the 2 middle elements
    let s = v.len();
    if s == 0 {
        return None;
    } else if s == 1 {
        return Some(v[0]);
    }

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if s % 2 == 0 {
        let l_mid = (s / 2) as usize - 1;
        let r_mid = l_mid + 2;
        return mean(&v[l_mid..r_mid]);
    }

    Some(v[(s / 2) as usize])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(None, median(&mut []));
        assert_eq!(1f32, median(&mut [1f32]).unwrap());
        assert_eq!(4f32, median(&mut [1f32, 4f32, 5f32]).unwrap());
        assert_eq!(4f32, median(&mut [1f32, 5f32, 4f32]).unwrap());
        assert_eq!(4.5f32, median(&mut [1f32, 4f32, 5f32, 9f32]).unwrap());
        assert_eq!(4.5f32, median(&mut [5f32, 9f32, 1f32, 4f32]).unwrap());
    }

    #[test]
    fn test_mean() {
        assert_eq!(1.5f32, mean(&[1f32, 2f32]).unwrap());
        assert_eq!(1f32, mean(&[1f32]).unwrap());
        assert_eq!(None, mean(&[]));
    }

    #[test]
    fn test_new() {
        let input_stat = InputStat {
            company: "company".to_string(),
            position: "software".to_string(),
            applied_dt: 20220406.0,
            rejected_dt: Some(20220408.0),
            first_interview: Some(20220407.0),
            offer_dt: Some(20220408.0),
            offer_amt: Some(349.99),
            accepted: Some("T".to_string()),
            referral: Some("Ol Nessy".to_string()),
        };
        let input_stats = vec![input_stat];

        let job_stats = JobStats::new(input_stats);

        assert_eq!(job_stats.num_applied, 1);
        assert_eq!(job_stats.num_offers, 1);
        assert_eq!(job_stats.num_rejections_all_types, 1);
        assert_eq!(job_stats.num_rejections_no_first_interview, 0);
        assert_eq!(job_stats.num_first_interviews_i_declined, 0);
        assert_eq!(job_stats.num_rejections_no_first_interview, 0);
        assert_eq!(job_stats.num_first_interviews_taken, 1);
        assert_eq!(job_stats.num_rejection_after_first_interview, 1);
        assert_eq!(job_stats.num_referrals, 1);

        println!("{}", job_stats);

        assert_eq!(1, 0)
    }

    #[test]
    fn test_precision_2() {
        assert_eq!(percent_with_precision_2(3.0f32, 8.0f32), "37.50%")
    }

    #[test]
    fn test_f32_to_datetime() {
        let expected_dt = NaiveDate::from_ymd(2022, 04, 08);
        assert_eq!(expected_dt, f64_to_datetime(20220408.0f64));
    }

    #[test]
    fn test_datetime_delta() {
        let local_date_pre = NaiveDate::from_ymd(2022, 04, 08);
        let local_date_post = NaiveDate::from_ymd(2022, 04, 12);

        let delta = local_date_post - local_date_pre;
        assert_eq!(4, delta.num_days());
    }
}
