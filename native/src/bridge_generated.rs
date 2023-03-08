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
// Generated by `flutter_rust_bridge`@ 1.68.0.

use crate::r_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::types::Address;
use crate::types::Balance;
use crate::types::ChannelInfo;
use crate::types::LdkInvoice;
use crate::types::LogEntry;
use crate::types::Network;
use crate::types::NodeInfo;
use crate::types::PaymentHash;
use crate::types::PaymentInfo;
use crate::types::PaymentPreimage;
use crate::types::PaymentSecret;
use crate::types::PaymentStatus;
use crate::types::PublicKey;

// Section: wire functions

fn wire_init_builder_impl(port_: MessagePort, config: impl Wire2Api<LdkConfig> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_builder",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_config = config.wire2api();
            move |task_callback| Ok(init_builder(api_config))
        },
    )
}
fn wire_start_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(start(api_ldk_node))
        },
    )
}
fn wire_node_id_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "node_id",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(node_id(api_ldk_node))
        },
    )
}
fn wire_get_balance_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_balance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(get_balance(api_ldk_node))
        },
    )
}
fn wire_new_funding_address_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_funding_address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(new_funding_address(api_ldk_node))
        },
    )
}
fn wire_sync_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sync",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(sync(api_ldk_node))
        },
    )
}
fn wire_get_node_addr_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_node_addr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(get_node_addr(api_ldk_node))
        },
    )
}
fn wire_next_event_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "next_event",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(next_event(api_ldk_node))
        },
    )
}
fn wire_stop_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "stop",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(stop(api_ldk_node))
        },
    )
}
fn wire_handle_event_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "handle_event",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(handle_event(api_ldk_node))
        },
    )
}
fn wire_node_info_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "node_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            move |task_callback| Ok(node_info(api_ldk_node))
        },
    )
}
fn wire_receive_payment_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    amount_msat: impl Wire2Api<Option<u64>> + UnwindSafe,
    description: impl Wire2Api<String> + UnwindSafe,
    expiry_secs: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "receive_payment",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            let api_amount_msat = amount_msat.wire2api();
            let api_description = description.wire2api();
            let api_expiry_secs = expiry_secs.wire2api();
            move |task_callback| {
                Ok(receive_payment(
                    api_ldk_node,
                    api_amount_msat,
                    api_description,
                    api_expiry_secs,
                ))
            }
        },
    )
}
fn wire_send_payment_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_payment",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(send_payment(api_ldk_node, api_invoice))
        },
    )
}
fn wire_send_spontaneous_payment_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    amount_msat: impl Wire2Api<u64> + UnwindSafe,
    node_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_spontaneous_payment",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            let api_amount_msat = amount_msat.wire2api();
            let api_node_id = node_id.wire2api();
            move |task_callback| {
                Ok(send_spontaneous_payment(
                    api_ldk_node,
                    api_amount_msat,
                    api_node_id,
                ))
            }
        },
    )
}
fn wire_payment_info_impl(
    port_: MessagePort,
    ldk_node: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    payment_hash: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "payment_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_node = ldk_node.wire2api();
            let api_payment_hash = payment_hash.wire2api();
            move |task_callback| Ok(payment_info(api_ldk_node, api_payment_hash))
        },
    )
}
fn wire_connect_open_channel_impl(
    port_: MessagePort,
    ldk_lite: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    node_pubkey_and_address: impl Wire2Api<String> + UnwindSafe,
    channel_amount_sats: impl Wire2Api<u64> + UnwindSafe,
    announce_channel: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "connect_open_channel",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_lite = ldk_lite.wire2api();
            let api_node_pubkey_and_address = node_pubkey_and_address.wire2api();
            let api_channel_amount_sats = channel_amount_sats.wire2api();
            let api_announce_channel = announce_channel.wire2api();
            move |task_callback| {
                Ok(connect_open_channel(
                    api_ldk_lite,
                    api_node_pubkey_and_address,
                    api_channel_amount_sats,
                    api_announce_channel,
                ))
            }
        },
    )
}
fn wire_close_channel_impl(
    port_: MessagePort,
    ldk_lite: impl Wire2Api<RustOpaque<LdkNodeInstance>> + UnwindSafe,
    channel_id: impl Wire2Api<String> + UnwindSafe,
    counterparty_node_id: impl Wire2Api<PublicKey> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "close_channel",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ldk_lite = ldk_lite.wire2api();
            let api_channel_id = channel_id.wire2api();
            let api_counterparty_node_id = counterparty_node_id.wire2api();
            move |task_callback| {
                Ok(close_channel(
                    api_ldk_lite,
                    api_channel_id,
                    api_counterparty_node_id,
                ))
            }
        },
    )
}
fn wire_create_log_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create_log_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| create_log_stream(task_callback.stream_sink()),
    )
}
fn wire_rust_set_up_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "rust_set_up",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(rust_set_up()),
    )
}
fn wire_create__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::create(api_invoice))
        },
    )
}
fn wire_amount_milli_satoshis__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "amount_milli_satoshis__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::amount_milli_satoshis(api_invoice))
        },
    )
}
fn wire_is_expired__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "is_expired__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::is_expired(api_invoice))
        },
    )
}
fn wire_expiry_time__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "expiry_time__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::expiry_time(api_invoice))
        },
    )
}
fn wire_payment_hash__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "payment_hash__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::payment_hash(api_invoice))
        },
    )
}
fn wire_payee_pub_key__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "payee_pub_key__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::payee_pub_key(api_invoice))
        },
    )
}
fn wire_payment_secret__static_method__LdkInvoice_impl(
    port_: MessagePort,
    invoice: impl Wire2Api<LdkInvoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "payment_secret__static_method__LdkInvoice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| Ok(LdkInvoice::payment_secret(api_invoice))
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

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<Network> for i32 {
    fn wire2api(self) -> Network {
        match self {
            0 => Network::Bitcoin,
            1 => Network::Testnet,
            2 => Network::Signet,
            3 => Network::Regtest,
            _ => unreachable!("Invalid variant for Network: {}", self),
        }
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Address {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_string.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Address {}

impl support::IntoDart for Balance {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.total.into_dart(),
            self.immature.into_dart(),
            self.trusted_pending.into_dart(),
            self.untrusted_pending.into_dart(),
            self.confirmed.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balance {}

impl support::IntoDart for ChannelInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.channel_id.into_dart(),
            self.funding_txid.into_dart(),
            self.peer_pubkey.into_dart(),
            self.peer_alias.into_dart(),
            self.short_channel_id.into_dart(),
            self.is_channel_ready.into_dart(),
            self.channel_value_satoshis.into_dart(),
            self.local_balance_msat.into_dart(),
            self.available_balance_for_send_msat.into_dart(),
            self.available_balance_for_recv_msat.into_dart(),
            self.channel_can_send_payments.into_dart(),
            self.public.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ChannelInfo {}

impl support::IntoDart for LdkInvoice {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_string.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LdkInvoice {}

impl support::IntoDart for LogEntry {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.level.into_dart(),
            self.tag.into_dart(),
            self.msg.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LogEntry {}

impl support::IntoDart for NodeInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.node_pub_key.into_dart(),
            self.channels.into_dart(),
            self.peers.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for NodeInfo {}

impl support::IntoDart for PaymentHash {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_string.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentHash {}

impl support::IntoDart for PaymentInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.preimage.into_dart(),
            self.secret.into_dart(),
            self.status.into_dart(),
            self.amount_msat.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentInfo {}

impl support::IntoDart for PaymentPreimage {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_u_array.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentPreimage {}

impl support::IntoDart for PaymentSecret {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_u_array.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentSecret {}

impl support::IntoDart for PaymentStatus {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Pending => 0,
            Self::Succeeded => 1,
            Self::Failed => 2,
        }
        .into_dart()
    }
}
impl support::IntoDart for PublicKey {
    fn into_dart(self) -> support::DartAbi {
        vec![self.as_string.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PublicKey {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
