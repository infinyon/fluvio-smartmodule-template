use fluvio_smartstream::{smartstream, Record};

{% if smartstream-type == "filter" %}
#[smartstream(filter)]
pub fn filter(record: &Record) -> bool {
    let str_result = std::str::from_utf8(record.value.as_ref());
    let string = match str_result {
        Ok(s) => s,
        _ => return false,
    };

    string.contains('a')
}
{% endif %}
