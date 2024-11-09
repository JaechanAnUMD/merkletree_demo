use risc0_zkvm::guest::env;

fn main() {
    // Private input data
    let input: [char; 3] = env::read();

    println!("Input: {} {} {}", input[0], input[1], input[2]);

    // TODO: derive some network performance (e.g., packet loss rate, RTT, etc.)
    let metric1: String = input.iter().collect();

    /**
     * Pseudocode for future work
     */
    /*
    let input: (Query, src, dest) = env::read();

    // 1. Compute routing path
    //  - return the merkle root for routing table as well
    path, routing_root = routing_table.lookup(src, dest); // merkle tree lookup

    // Array for saving each node roots
    node_roots = [];

    // 2. For each node in the routing path,
    while node /* node */ in path:
        // Lookup intermediate measurements
        //  - return the merkle root for each node's table as  well
        data_for_query, node_root = node.lookup(src, dest); // merkle tree lookup

        // Execute the logic to calculate the final query result
        // (e.g., latency, packet loss rate, etc.) and save the result
        query_result += logic_for_query(Query, data_for_query);

        node_roots.append(node_root);

    result = {
        routing_root,
        node_roots,
        query_result
    };

    commit(query_result);
    */

    // write public output to the journal
    env::commit(&metric1);
}
