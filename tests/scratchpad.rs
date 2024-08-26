use wasm_bindgen_test::{console_log, wasm_bindgen_test};

fn pred(s: &str, limit_seconds: usize) -> bool {
    let Some((head, tail)) = s.as_bytes().split_first_chunk::<2>() else {
        panic!("");
    };

    if head.ne(&[b'P', b'T']) {
        panic!("");
    }

    let mut units = [(b'H', 60 * 60), (b'M', 60), (b'S', 1usize)].into_iter();

    // TODO: 指定した秒数を超えたらfoldを中断したい try_fold
    let seconds = tail.chunk_by(|a, _| a.is_ascii_digit())
        .fold(0, |total, iter| {
            let st = iter.iter().cloned().fold(0usize, |st, i| {
                if i.is_ascii_digit() {
                    (st * 10) + usize::from(i - b'0')
                } else {
                    units
                        .find(|(x, _)| *x == i)
                        .map(|(_, x)| x * st)
                        .unwrap()
                }
            });

            total + st
        });
    
    console_log!("{:#?}", seconds);
    seconds <= limit_seconds
}

#[wasm_bindgen_test]
fn isoduration() {
    assert_eq!(pred("PT150S", 150), true);
    assert_eq!(pred("PT1H50S", 150), false);
    assert_eq!(pred("PT2M30S", 150), true);
    assert_eq!(pred("PT2M31S", 150), false);

}
