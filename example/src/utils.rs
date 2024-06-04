use alloy_primitives::U256;
use ethereum_types::{Address, H256};
use ethers_core::types::BlockId;
use ethers_providers::{Http, Middleware, Provider};
use tokio::runtime::{Builder, Handle, RuntimeFlavor};
use rustsol::types::SlotsGetter;

/// internal utility function to call tokio feature and wait for output
/// Taken from revm project.
#[inline]
fn block_on<F>(f: F) -> F::Output
    where
        F: core::future::Future + Send,
        F::Output: Send,
{
    match Handle::try_current() {
        Ok(handle) => match handle.runtime_flavor() {
            // This essentially equals to tokio::task::spawn_blocking because tokio doesn't
            // allow current_thread runtime to block_in_place
            RuntimeFlavor::CurrentThread => std::thread::scope(move |s| {
                s.spawn(move || {
                    Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(f)
                })
                    .join()
                    .unwrap()
            }),
            _ => tokio::task::block_in_place(move || handle.block_on(f)),
        },
        Err(_) => Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f),
    }
}


pub struct EthersSlotsGetterContext {
    pub contract: Address,
    pub block: Option<BlockId>,
}

pub struct EthersSlotsGetter {
    pub provider: Provider<Http>,
    pub context: EthersSlotsGetterContext,
}

#[allow(dead_code)]
impl EthersSlotsGetter {
    pub fn new(provider_url: &str, context: EthersSlotsGetterContext) -> Result<Self, String> {
        let provider = Provider::<Http>::try_from(provider_url)
            .map_err(|e| format!("Failed to create provider: {}", e))?;
        Ok(Self { provider, context })
    }
}

impl SlotsGetter for EthersSlotsGetter {
    fn get_slots(&self, start: U256, n: usize) -> Result<Vec<U256>, String> {
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            let slot = start + U256::from(i);
            let slot_h256 = H256::from(slot.to_be_bytes());
            let slot_value = block_on(self.provider.get_storage_at(self.context.contract, slot_h256, self.context.block))
                .map_err(|e| format!("Error fetching slot {}: {}", slot, e))?;
            res.push(U256::from_be_bytes(slot_value.to_fixed_bytes()));
            // println!("SlotsGetter: got slot {} = {:?}", slot, slot_value.to_be_bytes::<{ U256::BYTES }>());
            println!("[debug] SlotsGetter: got slot {}/{}: {} = {:#x}", i+1, n, slot, slot_value);
        }
        Ok(res)
    }
}


pub struct DummySlotsGetter;
impl SlotsGetter for DummySlotsGetter {
    fn get_slots(&self, start: U256, n: usize) -> Result<Vec<U256>, String> {
        let mut start_ = start;
        if start > U256::from(1000) {
            start_ = U256::from(1);
        }
        let mut res = Vec::with_capacity(n); // U256 is 32 bytes
        for i in 0..n {
            res.push(start_ + U256::from(i));
        }
        Ok(res)
    }
}
