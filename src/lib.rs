use fluvio_smartstream::{smartstream, Record};

{% if smartstream-type == "filter" %}
#[smartstream(filter)]
fn my_filter(record: &Record) -> bool {
    String::from_utf8_lossy(&record.value).contains('a')
}
{% else %}
#[smartstream(map)]
fn my_filter(mut record: Record) -> Record {
    record.value.make_ascii_uppercase();
    record
}
{% endif %}
