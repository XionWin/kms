use std::time::SystemTime;

pub fn pretty_print_system_time(t: SystemTime) -> String {
    let utc = time::OffsetDateTime::UNIX_EPOCH
        + time::Duration::try_from(t.duration_since(std::time::UNIX_EPOCH).unwrap()).unwrap();
    let local = utc.to_offset(time::UtcOffset::local_offset_at(utc).unwrap());
    let mut buf = Vec::new();
    local
        .format_into(
            &mut buf,
            time::macros::format_description!(
                "[day]-[month repr:short]-[year] [hour]:[minute]:[second]"
            ),
        )
        .unwrap();

    match std::str::from_utf8(&buf) {
        Ok(v) => String::from(v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}