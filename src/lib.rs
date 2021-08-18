{% if smartstream-type == "filter" %}
use fluvio_smartstream::{smartstream, Result, Record};

#[smartstream(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
{% elsif smartstream-type == "map" %}
use fluvio_smartstream::{smartstream, Result, Record, RecordData};
#[smartstream(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let string = std::str::from_utf8(record.value.as_ref())?;
    let int = string.parse::<i32>()?;
    let value = (int * 2).to_string();

    Ok((key, value.into()))
}
{% elsif smartstream-type == "aggregate" %}
use fluvio_smartstream::{smartstream, Result, Record, RecordData};

#[smartstream(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> Result<RecordData> {
    let mut acc = String::from_utf8(accumulator.as_ref().to_vec())?;
    let next = std::str::from_utf8(current.value.as_ref())?;
    acc.push_str(next);
    Ok(acc.into())
}
{% endif %}
