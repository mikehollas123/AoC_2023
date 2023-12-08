pub fn process() -> String {
    let best_time = 49787980i64;
    let distance = 298118510661181i64;

    let mut win_count = 0;

    for j in 1..=best_time{
        if wins(distance, best_time, j){
            win_count += 1;
        }
    }

    return format!("{}", win_count);
}

fn wins(distance: i64, max_time: i64, hold_time : i64) -> bool{
    (distance / hold_time) < max_time - hold_time
}