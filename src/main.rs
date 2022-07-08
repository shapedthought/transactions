use colored::*;
use dialoguer::Input;

fn main() {
    let capacity: String = Input::new()
        .with_prompt("Capacity in TB")
        .interact_text()
        .expect("Input wrong");
    let capacity = capacity.parse::<f32>().unwrap();

    let scope: String = Input::new()
        .with_prompt("Scope in years")
        .interact_text()
        .expect("Input wrong");
    let scope = scope.parse::<i16>().unwrap();

    let change: String = Input::new()
        .with_prompt("Daily change")
        .interact_text()
        .expect("Input wrong");
    let change = change.parse::<f32>().unwrap();

    let growth: String = Input::new()
        .with_prompt("Yearly growth")
        .interact_text()
        .expect("Input wrong");
    let growth = growth.parse::<f32>().unwrap();

    let move_after: String = Input::new()
        .with_prompt("Move after x days")
        .interact_text()
        .expect("Input wrong");
    let move_after = move_after.parse::<usize>().unwrap();

    let cost: String = Input::new()
        .with_prompt("Cost per 1000 transactions")
        .interact_text()
        .expect("Input wrong");
    let cost = cost.parse::<f32>().unwrap();

    let dev = 1024_f32.powf(2.00);
    let cap_as_mb = capacity * dev;
    let change_cap = cap_as_mb * change;

    let days = scope * 365;
    let prorate_growth = growth / 365.00;
    let full_trans = cap_as_mb * 2.00;
    dbg!(full_trans);

    let mut inc_cap = Vec::new();

    // for this to work we need to reduce the amount of days if move is used
    let days_red = days - i16::try_from(move_after).expect("could't parse usize");
    dbg!(days_red);

    for i in 1..days_red {
        let inc_growth = f32::from(i) * prorate_growth;
        let inc = change_cap * (1.00 + inc_growth);
        inc_cap.push(inc);
    }

    let week_delay = move_after / 7;
    dbg!(week_delay);

    let month_delay = move_after / 28;
    dbg!(month_delay);

    let yeary_delay = move_after / 365;
    dbg!(yeary_delay);

    let inc_trans: Vec<f32> = inc_cap
        .iter()
        .skip(move_after)
        .map(|x| (x * 2.00) / 1000.00)
        .collect();

    let weekly_trans: Vec<f32> = inc_cap
        .iter()
        .step_by(7)
        .skip(week_delay)
        .map(|x| ((x * 3.00) * 2.00) / 1000.00)
        .collect();

    let month_trans: Vec<f32> = inc_cap
        .iter()
        .step_by(30)
        .skip(month_delay)
        .map(|x| ((x * 5.00) * 2.00) / 1000.00)
        .collect();

    let yearly_trans: Vec<f32> = inc_cap
        .iter()
        .step_by(365)
        .skip(yeary_delay)
        .map(|x| ((x * 15.00) * 2.00) / 1000.00)
        .collect();

    println!("");
    println!("{}", "Results".to_string().purple());
    let inc_total: f32 = inc_trans.iter().sum();
    let weekly_total: f32 = weekly_trans.iter().sum();
    let monthly_total: f32 = month_trans.iter().sum();
    let yearly_total: f32 = yearly_trans.iter().sum();

    let inc_cost = (inc_total + full_trans) * cost;
    let weekly_cost = (weekly_total + full_trans) * cost;
    let monthly_cost = (monthly_total + full_trans) * cost;
    let yearly_cost = (yearly_total + full_trans) * cost;

    println!(
        "Incremental transactions: {} points: {}",
        (full_trans + inc_total).to_string().green(),
        inc_trans.len().to_string().green()
    );
    println!(
        "Weekly transactions: {} points: {}",
        (full_trans + weekly_total).to_string().green(),
        weekly_trans.len().to_string().green()
    );
    println!(
        "Monthly transactions: {} points: {}",
        (full_trans + monthly_total).to_string().green(),
        month_trans.len().to_string().green()
    );
    println!(
        "Yearly transactions: {} points: {}",
        (full_trans + yearly_total).to_string().green(),
        yearly_trans.len().to_string().green()
    );
    println!("");
    println!("Inc costs {}", inc_cost.to_string().red());
    println!("Weekly costs {}", weekly_cost.to_string().red());
    println!("Monthly costs {}", monthly_cost.to_string().red());
    println!("Yearly costs {}", yearly_cost.to_string().red());
}
