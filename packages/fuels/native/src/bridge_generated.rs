#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.57.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::model::transaction::Create;
use crate::model::transaction::Input;
use crate::model::transaction::Mint;
use crate::model::transaction::Output;
use crate::model::transaction::Script;
use crate::model::transaction::StorageSlot;
use crate::model::transaction::Transaction;
use crate::model::transaction::TransactionResponse;
use crate::model::transaction::TransactionStatus;
use crate::model::transaction::TxPointer;
use crate::model::transaction::UtxoId;
use crate::model::transaction::Witness;

// Section: wire functions

fn wire_new_random__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    api_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_random__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_api_url = api_url.wire2api();
            move |task_callback| Ok(WalletUnlocked::new_random(api_api_url))
        },
    )
}
fn wire_from_mnemonic_phrase__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
    api_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "from_mnemonic_phrase__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            let api_api_url = api_url.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::from_mnemonic_phrase(
                    api_phrase,
                    api_api_url,
                ))
            }
        },
    )
}
fn wire_address__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "address__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WalletUnlocked::address(&api_that))
        },
    )
}
fn wire_get_asset_balance__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    asset: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_asset_balance__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_asset = asset.wire2api();
            move |task_callback| Ok(WalletUnlocked::get_asset_balance(&api_that, api_asset))
        },
    )
}
fn wire_get_balances__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_balances__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WalletUnlocked::get_balances(&api_that))
        },
    )
}
fn wire_get_transactions__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    page_size: impl Wire2Api<usize> + UnwindSafe,
    cursor: impl Wire2Api<Option<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_transactions__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_page_size = page_size.wire2api();
            let api_cursor = cursor.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::get_transactions(
                    &api_that,
                    api_page_size,
                    api_cursor,
                ))
            }
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Balances {
    fn into_dart(self) -> support::DartAbi {
        vec![self.assets.into_dart(), self.balances.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balances {}

impl support::IntoDart for Create {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.gas_price.into_dart(),
            self.gas_limit.into_dart(),
            self.maturity.into_dart(),
            self.bytecode_length.into_dart(),
            self.bytecode_witness_index.into_dart(),
            self.storage_slots.into_dart(),
            self.inputs.into_dart(),
            self.outputs.into_dart(),
            self.witnesses.into_dart(),
            self.salt.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Create {}

impl support::IntoDart for Input {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::CoinSigned {
                utxo_id,
                owner,
                amount,
                asset_id,
                tx_pointer,
                witness_index,
                maturity,
            } => vec![
                0.into_dart(),
                utxo_id.into_dart(),
                owner.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
                tx_pointer.into_dart(),
                witness_index.into_dart(),
                maturity.into_dart(),
            ],
            Self::CoinPredicate {
                utxo_id,
                owner,
                amount,
                asset_id,
                tx_pointer,
                maturity,
                predicate,
                predicate_data,
            } => vec![
                1.into_dart(),
                utxo_id.into_dart(),
                owner.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
                tx_pointer.into_dart(),
                maturity.into_dart(),
                predicate.into_dart(),
                predicate_data.into_dart(),
            ],
            Self::Contract {
                utxo_id,
                balance_root,
                state_root,
                tx_pointer,
                contract_id,
            } => vec![
                2.into_dart(),
                utxo_id.into_dart(),
                balance_root.into_dart(),
                state_root.into_dart(),
                tx_pointer.into_dart(),
                contract_id.into_dart(),
            ],
            Self::MessageSigned {
                message_id,
                sender,
                recipient,
                amount,
                nonce,
                witness_index,
                data,
            } => vec![
                3.into_dart(),
                message_id.into_dart(),
                sender.into_dart(),
                recipient.into_dart(),
                amount.into_dart(),
                nonce.into_dart(),
                witness_index.into_dart(),
                data.into_dart(),
            ],
            Self::MessagePredicate {
                message_id,
                sender,
                recipient,
                amount,
                nonce,
                data,
                predicate,
                predicate_data,
            } => vec![
                4.into_dart(),
                message_id.into_dart(),
                sender.into_dart(),
                recipient.into_dart(),
                amount.into_dart(),
                nonce.into_dart(),
                data.into_dart(),
                predicate.into_dart(),
                predicate_data.into_dart(),
            ],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Input {}

impl support::IntoDart for Mint {
    fn into_dart(self) -> support::DartAbi {
        vec![self.tx_pointer.into_dart(), self.outputs.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Mint {}

impl support::IntoDart for Output {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Coin {
                to,
                amount,
                asset_id,
            } => vec![
                0.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
            ],
            Self::Contract {
                input_index,
                balance_root,
                state_root,
            } => vec![
                1.into_dart(),
                input_index.into_dart(),
                balance_root.into_dart(),
                state_root.into_dart(),
            ],
            Self::Message { recipient, amount } => {
                vec![2.into_dart(), recipient.into_dart(), amount.into_dart()]
            }
            Self::Change {
                to,
                amount,
                asset_id,
            } => vec![
                3.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
            ],
            Self::Variable {
                to,
                amount,
                asset_id,
            } => vec![
                4.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
            ],
            Self::ContractCreated {
                contract_id,
                state_root,
            } => vec![
                5.into_dart(),
                contract_id.into_dart(),
                state_root.into_dart(),
            ],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Output {}
impl support::IntoDart for Script {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.gas_price.into_dart(),
            self.gas_limit.into_dart(),
            self.maturity.into_dart(),
            self.script.into_dart(),
            self.script_data.into_dart(),
            self.inputs.into_dart(),
            self.outputs.into_dart(),
            self.witnesses.into_dart(),
            self.receipts_root.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Script {}

impl support::IntoDart for StorageSlot {
    fn into_dart(self) -> support::DartAbi {
        vec![self.key.into_dart(), self.value.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for StorageSlot {}

impl support::IntoDart for Transaction {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Script(field0) => vec![0.into_dart(), field0.into_dart()],
            Self::Create(field0) => vec![1.into_dart(), field0.into_dart()],
            Self::Mint(field0) => vec![2.into_dart(), field0.into_dart()],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Transaction {}
impl support::IntoDart for TransactionResponse {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.transaction.into_dart(),
            self.status.into_dart(),
            self.block_id.into_dart(),
            self.time.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TransactionResponse {}

impl support::IntoDart for TransactionStatus {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Submitted => 0,
            Self::Success => 1,
            Self::Failure => 2,
            Self::SqueezedOut => 3,
        }
        .into_dart()
    }
}
impl support::IntoDart for TxPointer {
    fn into_dart(self) -> support::DartAbi {
        vec![self.block_height.into_dart(), self.tx_index.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TxPointer {}

impl support::IntoDart for UtxoId {
    fn into_dart(self) -> support::DartAbi {
        vec![self.tx_id.into_dart(), self.output_index.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for UtxoId {}

impl support::IntoDart for WalletUnlocked {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.wallet_unlocked.into_dart(),
            self.private_key.into_dart(),
            self.mnemonic_phrase.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for WalletUnlocked {}

impl support::IntoDart for Witness {
    fn into_dart(self) -> support::DartAbi {
        vec![self.data.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Witness {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;