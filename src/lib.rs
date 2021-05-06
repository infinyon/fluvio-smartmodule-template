use fluvio_smartstream::{smartstream, SimpleRecord};

#[smartstream(filter)]
fn my_filter(_record: &SimpleRecord) -> bool {
    true
}