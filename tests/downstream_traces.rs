mod common;

use common::Replica;
use traces::SequentialTrace;

fn test_trace(trace: &SequentialTrace) {
    let trace = trace.chars_to_bytes();

    let trace = trace.chars_to_bytes();

    let mut upstream = Replica::new(0, trace.start_content());

    let edits = trace
        .edits()
        .flat_map(|(start, end, text)| {
            let mut edits = Vec::new();
            if end > start {
                edits.push(upstream.delete(start..end));
            }
            if !text.is_empty() {
                edits.push(upstream.insert(start, text));
            }
            edits
        })
        .collect::<Vec<_>>();

    let upstream = Replica::new(0, trace.start_content());

    let mut downstream = upstream.fork(1);

    for edit in &edits {
        downstream.merge(edit);
    }

    assert_eq!(downstream, trace.end_content());
}

#[test]
fn downstream_automerge() {
    test_trace(&traces::automerge());
}

#[test]
fn downstream_rustcode() {
    test_trace(&traces::rustcode());
}

#[test]
fn downstream_seph_blog() {
    test_trace(&traces::seph_blog());
}

#[test]
fn downstream_sveltecomponent() {
    test_trace(&traces::sveltecomponent());
}
