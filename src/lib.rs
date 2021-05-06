use fluvio_smartstream::{smartstream, SimpleRecord};

{% if smartstream-type == "filter" %}
#[smartstream(filter)]
fn my_filter(record: &SimpleRecord) -> bool {
    String::from_utf8_lossy(&record.value).contains('a')
}
{% else %}
#[smartstream(map)]
fn my_filter(mut record: SimpleRecord) -> SimpleRecord {
    record.value.make_ascii_uppercase();
    record
}
{% endif %}
