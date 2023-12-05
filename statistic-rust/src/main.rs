use serde::Deserialize;
use reqwest::blocking::get;
use std::io::Read;
use csv::ReadBuilder;
use std::error::Error;

#[derive(Debug,Deserialize)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salaryusd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: i32,
}

fn fetch_datase( url: &str ) -> <<ResultBoxstd