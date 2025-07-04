// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  16

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    channel
    (
        init => init
        upgrade => upgrade
        timeoutPacket => timeout_packet
        timeoutOnClose => timeout_on_close
        sendPacket => send_packet
        recvPacket => recieve_packet
        writeAcknowledgement => write_ack_endpoint
        acknowledgePacket => ack_packet
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
