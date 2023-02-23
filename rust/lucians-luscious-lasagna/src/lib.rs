pub fn expected_minutes_in_oven() -> i32 {
    let expected_minutes_in_oven: i32 = 40;
    return expected_minutes_in_oven;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return expected_minutes_in_oven() - actual_minutes_in_oven;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let preparation_time_in_minutes_per_layer: i32 = 2;
    return number_of_layers * preparation_time_in_minutes_per_layer;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
}
