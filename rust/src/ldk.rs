use crate::types::*;
pub use anyhow::anyhow;
use flutter_rust_bridge::*;
pub use ldk_node::io::SqliteStore;
use ldk_node::lightning::util::ser::Writeable;
use ldk_node::Builder;
pub use ldk_node::Node;
pub use std::sync::{Arc, Mutex};

pub fn generate_entropy_mnemonic() -> Mnemonic {
    let mnemonic: Mnemonic = ldk_node::generate_entropy_mnemonic().into();
    mnemonic
}
pub fn build_node(
    config: Config,
    chain_data_source_config: Option<ChainDataSourceConfig>,
    entropy_source_config: Option<EntropySourceConfig>,
    gossip_source_config: Option<GossipSourceConfig>,
) -> anyhow::Result<NodePointer> {
    let builder = build_builder(
        config,
        chain_data_source_config,
        entropy_source_config,
        gossip_source_config,
    );

    match builder.build() {
        Ok(e) => Ok(NodePointer(RustOpaque::new(Mutex::from(e)))),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}
fn build_builder(
    config: Config,
    chain_data_source_config: Option<ChainDataSourceConfig>,
    entropy_source_config: Option<EntropySourceConfig>,
    gossip_source_config: Option<GossipSourceConfig>,
) -> Builder {
    let mut builder = Builder::from_config(config.into());
    if let Some(source) = entropy_source_config {
        match source {
            EntropySourceConfig::SeedFile(e) => builder.set_entropy_seed_path(e),
            EntropySourceConfig::SeedBytes(e) => builder
                .set_entropy_seed_bytes(e.encode())
                .expect("InvalidSeedBytes"),
            EntropySourceConfig::Bip39Mnemonic {
                mnemonic,
                passphrase,
            } => builder.set_entropy_bip39_mnemonic(mnemonic.into(), passphrase),
        };
    }
    if let Some(source) = chain_data_source_config {
        match source {
            ChainDataSourceConfig::Esplora(e) => builder.set_esplora_server(e),
        };
    }

    if let Some(source) = gossip_source_config {
        match source {
            GossipSourceConfig::P2PNetwork => builder.set_gossip_source_p2p(),
            GossipSourceConfig::RapidGossipSync(e) => builder.set_gossip_source_rgs(e),
        };
    }
    builder
}

pub struct NodePointer(pub RustOpaque<Mutex<Node<SqliteStore>>>);
impl NodePointer {
    /// Starts the necessary background tasks, such as handling events coming from user input,
    /// LDK/BDK, and the peer-to-peer network.
    ///
    /// After this returns, the [Node] instance can be controlled via the provided API methods in
    /// a thread-safe manner.
    pub fn start(&self) -> anyhow::Result<()> {
        self.0
            .lock()
            .unwrap()
            .start()
            .map_err(|e| anyhow!(e.to_string()))
    }

    /// Disconnects all peers, stops all running background tasks, and shuts down [Node].
    ///
    /// After this returns most API methods will throw NotRunning Exception.
    pub fn stop(&self) -> anyhow::Result<()> {
        self.0
            .lock()
            .unwrap()
            .stop()
            .map_err(|e| anyhow!(e.to_string()))
    }

    /// Blocks until the next event is available.
    ///
    /// **Note:** this will always return the same event until handling is confirmed via `node.eventHandled()`.
    pub fn event_handled(&self) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        Ok(node_lock.event_handled())
    }

    /// Confirm the last retrieved event handled.
    ///
    /// **Note:** This **MUST** be called after each event has been handled.
    pub fn next_event(&self) -> Option<Event> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.next_event() {
            None => None,
            Some(e) => Some(e.into()),
        }
    }
    /// Returns the next event in the event queue.
    ///
    /// Will block the current thread until the next event is available.
    ///
    /// **Note:** this will always return the same event until handling is confirmed via `node.eventHandled()`.
    ///
    pub fn wait_until_next_event(&self) -> Event {
        let node_lock = self.0.lock().unwrap();
        (node_lock.wait_next_event()).into()
    }
    /// Returns our own node id
    pub fn node_id(&self) -> anyhow::Result<PublicKey> {
        let node_lock = self.0.lock().unwrap();
        Ok(PublicKey {
            internal: node_lock.node_id().to_string(),
        })
    }

    /// Returns our own listening address.
    pub fn listening_address(&self) -> Option<NetAddress> {
        let node_lock = self.0.lock().unwrap();
        node_lock.listening_address().map(|x| x.to_owned().into())
    }

    /// Retrieve a new on-chain/funding address.
    pub fn new_onchain_address(&self) -> anyhow::Result<Address> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.new_onchain_address() {
            Ok(e) => Ok(Address {
                internal: e.to_string(),
            }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    /// Retrieve the currently spendable on-chain balance in satoshis.
    pub fn spendable_onchain_balance_sats(&self) -> anyhow::Result<u64> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.spendable_onchain_balance_sats() {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Retrieve the current total on-chain balance in satoshis.
    pub fn total_onchain_balance_sats(&self) -> anyhow::Result<u64> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.total_onchain_balance_sats() {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Send an on-chain payment to the given address.
    pub fn send_to_onchain_address(
        &self,
        address: Address,
        amount_sats: u64,
    ) -> anyhow::Result<Txid> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.send_to_onchain_address(&address.into(), amount_sats) {
            Ok(e) => Ok(Txid {
                internal: e.to_string(),
            }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Send an on-chain payment to the given address, draining all the available funds.
    pub fn send_all_to_onchain_address(&self, address: Address) -> anyhow::Result<Txid> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.send_all_to_onchain_address(&address.into()) {
            Ok(e) => Ok(Txid {
                internal: e.to_string(),
            }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    ///Retrieve a list of known channels.
    ///
    pub fn list_channels(&self) -> Vec<ChannelDetails> {
        let node_lock = self.0.lock().unwrap();
        node_lock.list_channels().iter().map(|x| x.into()).collect()
    }
    /// Connect to a node on the peer-to-peer network.
    ///
    /// If `permanently` is set to `true`, we'll remember the peer and reconnect to it on restart.
    pub fn connect(
        &self,
        node_id: PublicKey,
        address: NetAddress,
        persist: bool,
    ) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.connect(node_id.into(), address.into(), persist) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Disconnects the peer with the given node id.
    ///
    /// Will also remove the peer from the peer store, i.e., after this has been called we won't
    /// try to reconnect on restart.
    pub fn disconnect(&self, counterparty_node_id: PublicKey) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.disconnect(counterparty_node_id.into()) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Connect to a node and open a new channel. Disconnects and re-connects are handled automatically
    ///
    /// Disconnects and reconnects are handled automatically.
    ///
    /// If `pushToCounterpartyMsat` is set, the given value will be pushed (read: sent) to the
    /// channel counterparty on channel open. This can be useful to start out with the balance not
    /// entirely shifted to one side, therefore allowing to receive payments from the getgo.
    ///
    /// Returns a temporary channel id.
    pub fn connect_open_channel(
        &self,
        address: NetAddress,
        node_id: PublicKey,
        channel_amount_sats: u64,
        push_to_counterparty_msat: Option<u64>,
        announce_channel: bool,
        channel_config: Option<ChannelConfig>,
    ) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.connect_open_channel(
            node_id.into(),
            address.into(),
            channel_amount_sats,
            push_to_counterparty_msat,
            channel_config.map(|x| x.into()),
            announce_channel,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    ///Sync the LDK and BDK wallets with the current chain state.
    // Note that the wallets will be also synced regularly in the background
    pub fn sync_wallets(&self) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.sync_wallets() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    /// Close a previously opened channel.
    pub fn close_channel(
        &self,
        channel_id: ChannelId,
        counterparty_node_id: PublicKey,
    ) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.close_channel(&(channel_id.into()), counterparty_node_id.into()) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    ///Update the config for a previously opened channel.
    ///
    pub fn update_channel_config(
        &self,
        channel_id: ChannelId,
        counterparty_node_id: PublicKey,
        channel_config: ChannelConfig,
    ) -> anyhow::Result<()> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.update_channel_config(
            &(channel_id.into()),
            counterparty_node_id.into(),
            &(channel_config).into(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    /// Send a payement given an invoice.
    pub fn send_payment(&self, invoice: Invoice) -> anyhow::Result<PaymentHash> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.send_payment(&invoice.into()) {
            Ok(e) => Ok(PaymentHash { internal: e.0 }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Send a payment given an invoice and an amount in millisatoshi.
    ///
    /// This will fail if the amount given is less than the value required by the given invoice.
    ///
    /// This can be used to pay a so-called "zero-amount" invoice, i.e., an invoice that leaves the
    /// amount paid to be determined by the user.
    pub fn send_payment_using_amount(
        &self,
        invoice: Invoice,
        amount_msat: u64,
    ) -> anyhow::Result<PaymentHash> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.send_payment_using_amount(&invoice.into(), amount_msat) {
            Ok(e) => Ok(PaymentHash { internal: e.0 }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Send a spontaneous, aka. "keysend", payment
    pub fn send_spontaneous_payment(
        &self,
        amount_msat: u64,
        node_id: PublicKey,
    ) -> anyhow::Result<PaymentHash> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.send_spontaneous_payment(amount_msat, node_id.into()) {
            Ok(e) => Ok(PaymentHash { internal: e.0 }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Returns a payable invoice that can be used to request and receive a payment of the amount
    /// given.
    pub fn receive_payment(
        &self,
        amount_msat: u64,
        description: String,
        expiry_secs: u32,
    ) -> anyhow::Result<Invoice> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.receive_payment(amount_msat, description.as_str(), expiry_secs) {
            Ok(e) => Ok(Invoice {
                internal: e.to_string(),
            }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    /// Returns a payable invoice that can be used to request and receive a payment for which the
    /// amount is to be determined by the user, also known as a "zero-amount" invoice.
    pub fn receive_variable_amount_payment(
        &self,
        description: String,
        expiry_secs: u32,
    ) -> anyhow::Result<Invoice> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.receive_variable_amount_payment(description.as_str(), expiry_secs) {
            Ok(e) => Ok(Invoice {
                internal: e.to_string(),
            }),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Retrieve the details of a specific payment with the given hash.
    ///
    /// Returns `PaymentDetails` if the payment was known and `null` otherwise.
    pub fn payment(&self, payment_hash: PaymentHash) -> Option<PaymentDetails> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.payment(&ldk_node::lightning::ln::PaymentHash(payment_hash.internal)) {
            None => None,
            Some(e) => Some(e.into()),
        }
    }

    /// Remove the payment with the given hash from the store.
    ///
    /// Returns `true` if the payment was present and `false` otherwise.
    pub fn remove_payment(&self, payment_hash: PaymentHash) -> anyhow::Result<bool> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.remove_payment(&ldk_node::lightning::ln::PaymentHash(payment_hash.internal))
        {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Retrieves all payments that match the given predicate.
    ///
    pub fn list_payments_with_filter(
        &self,
        payment_direction: PaymentDirection,
    ) -> Vec<PaymentDetails> {
        let node_lock = self.0.lock().unwrap();
        let payment_details =
            node_lock.list_payments_with_filter(|p| p.direction == payment_direction.into());
        payment_details
            .iter()
            .map(|x| x.to_owned().into())
            .collect()
    }
    /// Retrieves all payments.
    pub fn list_payments(&self) -> Vec<PaymentDetails> {
        let node_lock = self.0.lock().unwrap();
        node_lock
            .list_payments()
            .iter()
            .map(|x| x.to_owned().into())
            .collect()
    }
    /// Retrieves a list of known peers.
    pub fn list_peers(&self) -> Vec<PeerDetails> {
        let node_lock = self.0.lock().unwrap();
        node_lock
            .list_peers()
            .iter()
            .map(|x| x.to_owned().into())
            .collect()
    }
    /// Creates a digital ECDSA signature of a message with the node's secret key.
    ///
    /// A receiver knowing the corresponding `PublicKey` (e.g. the node’s id) and the message
    /// can be sure that the signature was generated by the caller.
    /// Signatures are EC recoverable, meaning that given the message and the
    /// signature the PublicKey of the signer can be extracted.
    pub fn sign_message(&self, msg: Vec<u8>) -> anyhow::Result<String> {
        let node_lock = self.0.lock().unwrap();
        match node_lock.sign_message(msg.as_slice()) {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Verifies that the given ECDSA signature was created for the given message with the
    /// secret key corresponding to the given public key.
    pub fn verify_signature(
        &self,
        msg: Vec<u8>,
        sig: String,
        pkey: PublicKey,
    ) -> anyhow::Result<bool> {
        let node_lock = self.0.lock().unwrap();
        Ok(node_lock.verify_signature(msg.as_slice(), sig.as_str(), &(pkey.into())))
    }
}
