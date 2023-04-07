use polkadot_js_api::{Api, errors::Result};
use std::sync::Arc;

// 订阅的事件名称和要订阅的区块高度
const EVENT_NAME: &str = "balances.Transfer";
const START_BLOCK: u64 = 0;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建 API 实例并连接到本地节点
    let api = Api::new(format!("ws://{}", "127.0.0.1:9944")).await?;

    // 获取最新的区块高度
    let current_block = api.get_finalized_head().await?.unwrap().number.unwrap().as_u64();

    // 订阅事件
    let event_stream = Arc::new(api.events(EVENT_NAME)?);

    // 打印区块高度和事件信息
    event_stream
        .filter_map(move |event| {
            let header = api.get_header(&event.block_hash).map(|header| header.unwrap());
            async move {
                match header {
                    Ok(header) if header.number.unwrap().as_u64() >= START_BLOCK => {
                        Some((header.number.unwrap().as_u64(), event))
                    }
                    _ => None,
                }
            }
        })
        .for_each(|(block_number, event)| async move {
            println!(
                "New event: block_number = {}, event = {:?}",
                block_number, event
            );
        })
        .await;

    Ok(())
}
