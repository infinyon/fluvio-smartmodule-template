use fluvio_smartstream::{smartstream, Record};

{% if smartstream-type == "filter" %}
#[smartstream(filter)]
fn my_filter(record: &Record) -> bool {
    String::from_utf8_lossy(&record.value).contains('a')
}
{% endif %}
