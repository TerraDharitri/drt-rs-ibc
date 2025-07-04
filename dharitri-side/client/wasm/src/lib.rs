// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  13

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    client
    (
        init => init
        upgrade => upgrade
        createClient => create_client
        updateClient => update_client
        updateClientCommitments => update_client_commitments
        setExpectedTimePerBlock => set_expected_time_per_block
        registerClient => register_client
        bindPort => bind_port
        getHostTimestamp => get_host_timestamp
        getCommitmentPrefix => get_commitment_prefix
        checkAndGetClient => check_and_get_client
        getCommitment => get_commitment
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
