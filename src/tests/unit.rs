use crate::ibc_hooks::{Coin, MsgTransfer};
use cosmwasm_std::CosmosMsg::Stargate;
use prost::Message;

#[test]
fn convert_to_protobuf() {
    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-2".to_string(),
        token: Some(Coin { denom: "ibc/4CD525F166D32B0132C095F353F4C6F033B0FF5C49141470D1EFDA1D63303D04".to_string(), amount: "1".to_string() }),
        sender: "terra1wc053w5h0zmju4vr6z0f9syf74mmds99xnt6wxj6egr95jl92xzs3jf6ge".to_string(),
        receiver: "juno13msy7cpmp6ah7zg44g2pvtlxtt7s9cuh4gx2psmp74zxpd0ezjvq2ak8ud".to_string(),
        timeout_timestamp: 1692302903000000000,
        // memo: "{\"wasm\":{\"contract\":\"juno1xr2264tpuwhh4fp6ge8t7yl2l3hwrg4tprfmpe42zsvu5p53gdqq0g4e00\",\"msg\":{\"execute_msgs\":{\"msgs\":[{\"wasm\":{\"instantiate\":{\"admin\":\"juno1xr2264tpuwhh4fp6ge8t7yl2l3hwrg4tprfmpe42zsvu5p53gdqq0g4e00\",\"code_id\":3445,\"msg\":\"e30=\",\"funds\":[{\"denom\":\"ujuno\", \"amount\": \"1\"}],\"label\":\"proxy_contract\"}}}]}}}}".to_string(),
        // memo: "{\"wasm\":{\"contract\":\"juno13msy7cpmp6ah7zg44g2pvtlxtt7s9cuh4gx2psmp74zxpd0ezjvq2ak8ud\",\"msg\":{\"execute_msgs\":{\"msgs\":[{\"msg\":{\"wasm\":{\"instantiate\":{\"admin\":\"juno1d03ftpw5tefzq22akwq839kyudsf88yvkq00dq\",\"code_id\":3449,\"msg\":\"eyJvd25lciI6Imp1bm8xZDAzZnRwdzV0ZWZ6cTIyYWt3cTgzOWt5dWRzZjg4eXZrcTAwZHEifQ==\",\"funds\":[{\"denom\":\"ujuno\",\"amount\":\"1\"}],\"label\":\"proxy_contract_2\"}}},\"reply_callback\":{\"callback_id\":1, \"ibc_channel\":\"channel-86\",\"receiver\":\"terra1wyseu58j0z6e83ztgwz962lhajmu4afwu2jdq68lrlahfneu0p3q4nlqjd\"}}]}}}}".to_string(),
        memo: "{\"wasm\":{\"contract\":\"juno13msy7cpmp6ah7zg44g2pvtlxtt7s9cuh4gx2psmp74zxpd0ezjvq2ak8ud\",\"msg\":{\"execute_msgs\":{\"msgs\":[{\"msg\":{\"wasm\":{\"instantiate\":{\"admin\":\"juno1d03ftpw5tefzq22akwq839kyudsf88yvkq00dq\",\"code_id\":3455,\"msg\":\"eyJvd25lciI6Imp1bm8xZDAzZnRwdzV0ZWZ6cTIyYWt3cTgzOWt5dWRzZjg4eXZrcTAwZHEifQ==\",\"funds\":[],\"label\":\"proxy_contract_latest\"}}},\"reply_callback\":{\"callback_id\":1, \"ibc_channel\":\"channel-86\",\"receiver\":\"terra1wc053w5h0zmju4vr6z0f9syf74mmds99xnt6wxj6egr95jl92xzs3jf6ge\"}}]}}}}".to_string(),
        // memo: "{\"wasm\":{\"contract\":\"juno13msy7cpmp6ah7zg44g2pvtlxtt7s9cuh4gx2psmp74zxpd0ezjvq2ak8ud\",\"msg\":{\"write_state\":{\"message\":\"remotely written state\"}}}}".to_string(),
        // memo: "{\"wasm\":{\"contract\":\"juno1zpv9920jhe3uhgk7ktfce6et7fdv9gvhu52uqr0mpm7dxk8an0gspgg65d\",\"msg\":{\"execute_msgs\":{\"msgs\":[{\"msg\":{\"wasm\":{\"instantiate\":{\"admin\":\"juno1d03ftpw5tefzq22akwq839kyudsf88yvkq00dq\",\"code_id\":3449,\"msg\":\"eyJvd25lciI6Imp1bm8xZDAzZnRwdzV0ZWZ6cTIyYWt3cTgzOWt5dWRzZjg4eXZrcTAwZHEifQ==\",\"funds\":[{\"denom\":\"ujuno\",\"amount\":\"1\"}],\"label\":\"proxy_contract_2\"}}}}]}}}}".to_string(),
        // memo: "".to_string(),
    }.encode_to_vec();

    println!(
        "{}",
        serde_json_wasm::to_string(&Stargate::<String> {
            type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
            value: msg.into()
        })
        .unwrap()
    );
}
