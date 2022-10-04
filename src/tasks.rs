use graphs_at_ssu::*;

/// Find nodes reachable from u, but unreachable from v
pub fn solve1a1(gr: &Graph, u: String, v: String) -> GraphResult<Vec<String>> {
    let al = gr.get_adjacency_list();
    check_if_contains(gr, &u, &v)?;

    Ok(gr
        .get_nodes()
        .into_iter()
        .filter(|n| al[&u].contains_key(n) && !al[&v].contains_key(n))
        .collect())
}

/// Find nodes reachable from u and v
pub fn solve1a2(gr: &Graph, u: String, v: String) -> GraphResult<Vec<String>> {
    let al = gr.get_adjacency_list();
    check_if_contains(gr, &u, &v)?;

    Ok(gr
        .get_nodes()
        .into_iter()
        .filter(|n| al[&u].contains_key(n) && al[&v].contains_key(n))
        .collect())
}

fn check_if_contains(gr: &Graph, u: &String, v: &String) -> GraphResult<()> {
    let al = gr.get_adjacency_list();
    if !al.contains_key(u) {
        return Err(GraphError {
            msg: format!("Node u {u:?} does not exist"),
        });
    }
    if !al.contains_key(v) {
        return Err(GraphError {
            msg: format!("Node v {v:?} does not exist"),
        });
    }

    Ok(())
}