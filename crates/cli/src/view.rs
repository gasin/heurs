use heurs_core::ExecutionResult;
use heurs_database::{ExecutionResultModel, SubmissionModel, TestCaseModel};
use std::cmp::Ordering;
use tabled::{Table, Tabled};

#[derive(Clone, Tabled)]
struct TestCaseRow {
    #[tabled(rename = "Case ID")]
    case_id: u32,
    #[tabled(rename = "File Name")]
    file_name: String,
    #[tabled(rename = "Score")]
    score: i64,
    #[tabled(rename = "Time(ms)")]
    time: u32,
}

#[derive(Clone, Tabled)]
struct SubmissionRow {
    #[tabled(rename = "Submission ID")]
    submission_id: i32,
    #[tabled(rename = "Avg Score")]
    avg_score: f64,
    #[tabled(rename = "Avg Time(ms)")]
    avg_time: f64,
}

pub fn render_execution_results(
    execution_results: &Vec<ExecutionResult>,
    test_cases: &Vec<TestCaseModel>,
) {
    let mut rows: Vec<TestCaseRow> = execution_results
        .iter()
        .map(|r| {
            let file_name = test_cases
                .iter()
                .find(|t| t.id == r.test_case_id as i32)
                .map(|t| t.filename.clone())
                .unwrap_or_else(|| "".to_string());

            TestCaseRow {
                case_id: r.test_case_id as u32,
                file_name,
                score: r.score,
                time: r.execution_time_ms as u32,
            }
        })
        .collect();

    rows.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    println!("\n{}", Table::new(rows));
}
pub fn render_submission_summary(
    submission: &SubmissionModel,
    execution_results: &Vec<ExecutionResult>,
) {
    println!("Submission ID: {}", submission.id);
    println!("Timestamp: {}", submission.timestamp);

    println!(
        "Average score: {}",
        execution_results.iter().map(|r| r.score).sum::<i64>() as f64
            / execution_results.len() as f64
    );
    println!(
        "Average time: {}",
        execution_results
            .iter()
            .map(|r| r.execution_time_ms as f64)
            .sum::<f64>()
            / execution_results.len() as f64
    );
}

pub fn render_leaderboard(
    submissions: &Vec<SubmissionModel>,
    execution_results: &Vec<ExecutionResultModel>,
    limit: u32,
) {
    let mut rows: Vec<SubmissionRow> = Vec::new();

    for sub in submissions {
        let results = execution_results
            .iter()
            .filter(|r| r.submission_id == sub.id as i64)
            .collect::<Vec<&ExecutionResultModel>>();

        let (sum, count) = results
            .iter()
            .fold((0i64, 0usize), |(s, c), r| (s + r.score, c + 1));
        let avg_score = if count > 0 {
            sum as f64 / count as f64
        } else {
            0.0
        };

        let avg_time = if count > 0 {
            results
                .iter()
                .map(|r| r.execution_time_ms as f64)
                .sum::<f64>()
                / count as f64
        } else {
            0.0
        };

        rows.push(SubmissionRow {
            submission_id: sub.id,
            avg_score,
            avg_time,
        });
    }

    rows.sort_by(|a, b| {
        b.avg_score
            .partial_cmp(&a.avg_score)
            .unwrap_or(Ordering::Equal)
    });

    let display_rows: Vec<SubmissionRow> = rows.into_iter().take(limit as usize).collect();

    println!("\n{}", Table::new(display_rows));
}
