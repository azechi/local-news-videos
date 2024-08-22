use chrono::DateTime;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

#[wasm_bindgen_test]
fn pass123() {

    let s = "2024-08-16T05:37:03Z";
    let d = DateTime::parse_from_rfc3339(s).unwrap().timestamp_millis();

    let n = TryInto::<i64>::try_into(worker::Date::now().as_millis()).unwrap();

    console_log!("{:#?} {:#?}", d, n);

    let dd = DateTime::from_timestamp_millis(n);

    console_log!("{:#?}", dd);


    //console_log!("")
    assert!(true);
}


#[wasm_bindgen_test]
fn pass() {
    console_log!("{:#?}", chrono::Utc::now());

}