#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    #[prost(uint64, tag = "1")]
    pub msat: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountOrAll {
    #[prost(oneof = "amount_or_all::Value", tags = "1, 2")]
    pub value: ::core::option::Option<amount_or_all::Value>,
}
/// Nested message and enum types in `AmountOrAll`.
pub mod amount_or_all {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Amount(super::Amount),
        #[prost(bool, tag = "2")]
        All(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountOrAny {
    #[prost(oneof = "amount_or_any::Value", tags = "1, 2")]
    pub value: ::core::option::Option<amount_or_any::Value>,
}
/// Nested message and enum types in `AmountOrAny`.
pub mod amount_or_any {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Amount(super::Amount),
        #[prost(bool, tag = "2")]
        Any(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelStateChangeCause {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Outpoint {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feerate {
    #[prost(oneof = "feerate::Style", tags = "1, 2, 3, 4, 5")]
    pub style: ::core::option::Option<feerate::Style>,
}
/// Nested message and enum types in `Feerate`.
pub mod feerate {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Style {
        #[prost(bool, tag = "1")]
        Slow(bool),
        #[prost(bool, tag = "2")]
        Normal(bool),
        #[prost(bool, tag = "3")]
        Urgent(bool),
        #[prost(uint32, tag = "4")]
        Perkb(u32),
        #[prost(uint32, tag = "5")]
        Perkw(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputDesc {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteHop {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub scid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub feebase: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "4")]
    pub feeprop: u32,
    #[prost(uint32, tag = "5")]
    pub expirydelta: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Routehint {
    #[prost(message, repeated, tag = "1")]
    pub hops: ::prost::alloc::vec::Vec<RouteHop>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutehintList {
    #[prost(message, repeated, tag = "2")]
    pub hints: ::prost::alloc::vec::Vec<Routehint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeRouteHop {
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub short_channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "4")]
    pub fee_proportional_millionths: u32,
    #[prost(uint32, tag = "5")]
    pub cltv_expiry_delta: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeRoutehint {
    #[prost(message, repeated, tag = "1")]
    pub hops: ::prost::alloc::vec::Vec<DecodeRouteHop>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeRoutehintList {
    #[prost(message, repeated, tag = "2")]
    pub hints: ::prost::alloc::vec::Vec<DecodeRoutehint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlvEntry {
    #[prost(uint64, tag = "1")]
    pub r#type: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlvStream {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<TlvEntry>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelSide {
    Local = 0,
    Remote = 1,
}
impl ChannelSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelSide::Local => "LOCAL",
            ChannelSide::Remote => "REMOTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCAL" => Some(Self::Local),
            "REMOTE" => Some(Self::Remote),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelState {
    Openingd = 0,
    ChanneldAwaitingLockin = 1,
    ChanneldNormal = 2,
    ChanneldShuttingDown = 3,
    ClosingdSigexchange = 4,
    ClosingdComplete = 5,
    AwaitingUnilateral = 6,
    FundingSpendSeen = 7,
    Onchain = 8,
    DualopendOpenInit = 9,
    DualopendAwaitingLockin = 10,
    ChanneldAwaitingSplice = 11,
}
impl ChannelState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelState::Openingd => "Openingd",
            ChannelState::ChanneldAwaitingLockin => "ChanneldAwaitingLockin",
            ChannelState::ChanneldNormal => "ChanneldNormal",
            ChannelState::ChanneldShuttingDown => "ChanneldShuttingDown",
            ChannelState::ClosingdSigexchange => "ClosingdSigexchange",
            ChannelState::ClosingdComplete => "ClosingdComplete",
            ChannelState::AwaitingUnilateral => "AwaitingUnilateral",
            ChannelState::FundingSpendSeen => "FundingSpendSeen",
            ChannelState::Onchain => "Onchain",
            ChannelState::DualopendOpenInit => "DualopendOpenInit",
            ChannelState::DualopendAwaitingLockin => "DualopendAwaitingLockin",
            ChannelState::ChanneldAwaitingSplice => "ChanneldAwaitingSplice",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Openingd" => Some(Self::Openingd),
            "ChanneldAwaitingLockin" => Some(Self::ChanneldAwaitingLockin),
            "ChanneldNormal" => Some(Self::ChanneldNormal),
            "ChanneldShuttingDown" => Some(Self::ChanneldShuttingDown),
            "ClosingdSigexchange" => Some(Self::ClosingdSigexchange),
            "ClosingdComplete" => Some(Self::ClosingdComplete),
            "AwaitingUnilateral" => Some(Self::AwaitingUnilateral),
            "FundingSpendSeen" => Some(Self::FundingSpendSeen),
            "Onchain" => Some(Self::Onchain),
            "DualopendOpenInit" => Some(Self::DualopendOpenInit),
            "DualopendAwaitingLockin" => Some(Self::DualopendAwaitingLockin),
            "ChanneldAwaitingSplice" => Some(Self::ChanneldAwaitingSplice),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HtlcState {
    SentAddHtlc = 0,
    SentAddCommit = 1,
    RcvdAddRevocation = 2,
    RcvdAddAckCommit = 3,
    SentAddAckRevocation = 4,
    RcvdAddAckRevocation = 5,
    RcvdRemoveHtlc = 6,
    RcvdRemoveCommit = 7,
    SentRemoveRevocation = 8,
    SentRemoveAckCommit = 9,
    RcvdRemoveAckRevocation = 10,
    RcvdAddHtlc = 11,
    RcvdAddCommit = 12,
    SentAddRevocation = 13,
    SentAddAckCommit = 14,
    SentRemoveHtlc = 15,
    SentRemoveCommit = 16,
    RcvdRemoveRevocation = 17,
    RcvdRemoveAckCommit = 18,
    SentRemoveAckRevocation = 19,
}
impl HtlcState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HtlcState::SentAddHtlc => "SentAddHtlc",
            HtlcState::SentAddCommit => "SentAddCommit",
            HtlcState::RcvdAddRevocation => "RcvdAddRevocation",
            HtlcState::RcvdAddAckCommit => "RcvdAddAckCommit",
            HtlcState::SentAddAckRevocation => "SentAddAckRevocation",
            HtlcState::RcvdAddAckRevocation => "RcvdAddAckRevocation",
            HtlcState::RcvdRemoveHtlc => "RcvdRemoveHtlc",
            HtlcState::RcvdRemoveCommit => "RcvdRemoveCommit",
            HtlcState::SentRemoveRevocation => "SentRemoveRevocation",
            HtlcState::SentRemoveAckCommit => "SentRemoveAckCommit",
            HtlcState::RcvdRemoveAckRevocation => "RcvdRemoveAckRevocation",
            HtlcState::RcvdAddHtlc => "RcvdAddHtlc",
            HtlcState::RcvdAddCommit => "RcvdAddCommit",
            HtlcState::SentAddRevocation => "SentAddRevocation",
            HtlcState::SentAddAckCommit => "SentAddAckCommit",
            HtlcState::SentRemoveHtlc => "SentRemoveHtlc",
            HtlcState::SentRemoveCommit => "SentRemoveCommit",
            HtlcState::RcvdRemoveRevocation => "RcvdRemoveRevocation",
            HtlcState::RcvdRemoveAckCommit => "RcvdRemoveAckCommit",
            HtlcState::SentRemoveAckRevocation => "SentRemoveAckRevocation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SentAddHtlc" => Some(Self::SentAddHtlc),
            "SentAddCommit" => Some(Self::SentAddCommit),
            "RcvdAddRevocation" => Some(Self::RcvdAddRevocation),
            "RcvdAddAckCommit" => Some(Self::RcvdAddAckCommit),
            "SentAddAckRevocation" => Some(Self::SentAddAckRevocation),
            "RcvdAddAckRevocation" => Some(Self::RcvdAddAckRevocation),
            "RcvdRemoveHtlc" => Some(Self::RcvdRemoveHtlc),
            "RcvdRemoveCommit" => Some(Self::RcvdRemoveCommit),
            "SentRemoveRevocation" => Some(Self::SentRemoveRevocation),
            "SentRemoveAckCommit" => Some(Self::SentRemoveAckCommit),
            "RcvdRemoveAckRevocation" => Some(Self::RcvdRemoveAckRevocation),
            "RcvdAddHtlc" => Some(Self::RcvdAddHtlc),
            "RcvdAddCommit" => Some(Self::RcvdAddCommit),
            "SentAddRevocation" => Some(Self::SentAddRevocation),
            "SentAddAckCommit" => Some(Self::SentAddAckCommit),
            "SentRemoveHtlc" => Some(Self::SentRemoveHtlc),
            "SentRemoveCommit" => Some(Self::SentRemoveCommit),
            "RcvdRemoveRevocation" => Some(Self::RcvdRemoveRevocation),
            "RcvdRemoveAckCommit" => Some(Self::RcvdRemoveAckCommit),
            "SentRemoveAckRevocation" => Some(Self::SentRemoveAckRevocation),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelTypeName {
    StaticRemotekeyEven = 0,
    AnchorOutputsEven = 1,
    AnchorsZeroFeeHtlcTxEven = 2,
    ScidAliasEven = 3,
    ZeroconfEven = 4,
}
impl ChannelTypeName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelTypeName::StaticRemotekeyEven => "static_remotekey_even",
            ChannelTypeName::AnchorOutputsEven => "anchor_outputs_even",
            ChannelTypeName::AnchorsZeroFeeHtlcTxEven => "anchors_zero_fee_htlc_tx_even",
            ChannelTypeName::ScidAliasEven => "scid_alias_even",
            ChannelTypeName::ZeroconfEven => "zeroconf_even",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "static_remotekey_even" => Some(Self::StaticRemotekeyEven),
            "anchor_outputs_even" => Some(Self::AnchorOutputsEven),
            "anchors_zero_fee_htlc_tx_even" => Some(Self::AnchorsZeroFeeHtlcTxEven),
            "scid_alias_even" => Some(Self::ScidAliasEven),
            "zeroconf_even" => Some(Self::ZeroconfEven),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutocleanSubsystem {
    Succeededforwards = 0,
    Failedforwards = 1,
    Succeededpays = 2,
    Failedpays = 3,
    Paidinvoices = 4,
    Expiredinvoices = 5,
}
impl AutocleanSubsystem {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutocleanSubsystem::Succeededforwards => "SUCCEEDEDFORWARDS",
            AutocleanSubsystem::Failedforwards => "FAILEDFORWARDS",
            AutocleanSubsystem::Succeededpays => "SUCCEEDEDPAYS",
            AutocleanSubsystem::Failedpays => "FAILEDPAYS",
            AutocleanSubsystem::Paidinvoices => "PAIDINVOICES",
            AutocleanSubsystem::Expiredinvoices => "EXPIREDINVOICES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCEEDEDFORWARDS" => Some(Self::Succeededforwards),
            "FAILEDFORWARDS" => Some(Self::Failedforwards),
            "SUCCEEDEDPAYS" => Some(Self::Succeededpays),
            "FAILEDPAYS" => Some(Self::Failedpays),
            "PAIDINVOICES" => Some(Self::Paidinvoices),
            "EXPIREDINVOICES" => Some(Self::Expiredinvoices),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PluginSubcommand {
    Start = 0,
    Stop = 1,
    Rescan = 2,
    Startdir = 3,
    List = 4,
}
impl PluginSubcommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PluginSubcommand::Start => "START",
            PluginSubcommand::Stop => "STOP",
            PluginSubcommand::Rescan => "RESCAN",
            PluginSubcommand::Startdir => "STARTDIR",
            PluginSubcommand::List => "LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "START" => Some(Self::Start),
            "STOP" => Some(Self::Stop),
            "RESCAN" => Some(Self::Rescan),
            "STARTDIR" => Some(Self::Startdir),
            "LIST" => Some(Self::List),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetinfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetinfoResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "2")]
    pub alias: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "3")]
    pub color: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub num_peers: u32,
    #[prost(uint32, tag = "5")]
    pub num_pending_channels: u32,
    #[prost(uint32, tag = "6")]
    pub num_active_channels: u32,
    #[prost(uint32, tag = "7")]
    pub num_inactive_channels: u32,
    #[prost(string, tag = "8")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub lightning_dir: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub our_features: ::core::option::Option<GetinfoOurFeatures>,
    #[prost(uint32, tag = "11")]
    pub blockheight: u32,
    #[prost(string, tag = "12")]
    pub network: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "13")]
    pub fees_collected_msat: ::core::option::Option<Amount>,
    #[prost(message, repeated, tag = "14")]
    pub address: ::prost::alloc::vec::Vec<GetinfoAddress>,
    #[prost(message, repeated, tag = "15")]
    pub binding: ::prost::alloc::vec::Vec<GetinfoBinding>,
    #[prost(string, optional, tag = "16")]
    pub warning_bitcoind_sync: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub warning_lightningd_sync: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetinfoOurFeatures {
    #[prost(bytes = "vec", tag = "1")]
    pub init: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub node: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub invoice: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetinfoAddress {
    #[prost(enumeration = "getinfo_address::GetinfoAddressType", tag = "1")]
    pub item_type: i32,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(string, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetinfoAddress`.
pub mod getinfo_address {
    /// Getinfo.address\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GetinfoAddressType {
        Dns = 0,
        Ipv4 = 1,
        Ipv6 = 2,
        Torv2 = 3,
        Torv3 = 4,
    }
    impl GetinfoAddressType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GetinfoAddressType::Dns => "DNS",
                GetinfoAddressType::Ipv4 => "IPV4",
                GetinfoAddressType::Ipv6 => "IPV6",
                GetinfoAddressType::Torv2 => "TORV2",
                GetinfoAddressType::Torv3 => "TORV3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DNS" => Some(Self::Dns),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "TORV2" => Some(Self::Torv2),
                "TORV3" => Some(Self::Torv3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetinfoBinding {
    #[prost(enumeration = "getinfo_binding::GetinfoBindingType", tag = "1")]
    pub item_type: i32,
    #[prost(string, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub port: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub socket: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub subtype: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetinfoBinding`.
pub mod getinfo_binding {
    /// Getinfo.binding\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GetinfoBindingType {
        LocalSocket = 0,
        Ipv4 = 1,
        Ipv6 = 2,
        Torv2 = 3,
        Torv3 = 4,
        Websocket = 5,
    }
    impl GetinfoBindingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GetinfoBindingType::LocalSocket => "LOCAL_SOCKET",
                GetinfoBindingType::Ipv4 => "IPV4",
                GetinfoBindingType::Ipv6 => "IPV6",
                GetinfoBindingType::Torv2 => "TORV2",
                GetinfoBindingType::Torv3 => "TORV3",
                GetinfoBindingType::Websocket => "WEBSOCKET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCAL_SOCKET" => Some(Self::LocalSocket),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "TORV2" => Some(Self::Torv2),
                "TORV3" => Some(Self::Torv3),
                "WEBSOCKET" => Some(Self::Websocket),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeersRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "listpeers_request::ListpeersLevel", optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ListpeersRequest`.
pub mod listpeers_request {
    /// ListPeers.level
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpeersLevel {
        Io = 0,
        Debug = 1,
        Info = 2,
        Unusual = 3,
        Trace = 4,
    }
    impl ListpeersLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpeersLevel::Io => "IO",
                ListpeersLevel::Debug => "DEBUG",
                ListpeersLevel::Info => "INFO",
                ListpeersLevel::Unusual => "UNUSUAL",
                ListpeersLevel::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IO" => Some(Self::Io),
                "DEBUG" => Some(Self::Debug),
                "INFO" => Some(Self::Info),
                "UNUSUAL" => Some(Self::Unusual),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeersResponse {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<ListpeersPeers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeersPeers {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub connected: bool,
    #[prost(message, repeated, tag = "3")]
    pub log: ::prost::alloc::vec::Vec<ListpeersPeersLog>,
    #[prost(string, repeated, tag = "5")]
    pub netaddr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "7")]
    pub remote_addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "8")]
    pub num_channels: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeersPeersLog {
    #[prost(enumeration = "listpeers_peers_log::ListpeersPeersLogType", tag = "1")]
    pub item_type: i32,
    #[prost(uint32, optional, tag = "2")]
    pub num_skipped: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub log: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `ListpeersPeersLog`.
pub mod listpeers_peers_log {
    /// ListPeers.peers\[\].log\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpeersPeersLogType {
        Skipped = 0,
        Broken = 1,
        Unusual = 2,
        Info = 3,
        Debug = 4,
        IoIn = 5,
        IoOut = 6,
        Trace = 7,
    }
    impl ListpeersPeersLogType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpeersPeersLogType::Skipped => "SKIPPED",
                ListpeersPeersLogType::Broken => "BROKEN",
                ListpeersPeersLogType::Unusual => "UNUSUAL",
                ListpeersPeersLogType::Info => "INFO",
                ListpeersPeersLogType::Debug => "DEBUG",
                ListpeersPeersLogType::IoIn => "IO_IN",
                ListpeersPeersLogType::IoOut => "IO_OUT",
                ListpeersPeersLogType::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SKIPPED" => Some(Self::Skipped),
                "BROKEN" => Some(Self::Broken),
                "UNUSUAL" => Some(Self::Unusual),
                "INFO" => Some(Self::Info),
                "DEBUG" => Some(Self::Debug),
                "IO_IN" => Some(Self::IoIn),
                "IO_OUT" => Some(Self::IoOut),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListfundsRequest {
    #[prost(bool, optional, tag = "1")]
    pub spent: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListfundsResponse {
    #[prost(message, repeated, tag = "1")]
    pub outputs: ::prost::alloc::vec::Vec<ListfundsOutputs>,
    #[prost(message, repeated, tag = "2")]
    pub channels: ::prost::alloc::vec::Vec<ListfundsChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListfundsOutputs {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub output: u32,
    #[prost(message, optional, tag = "3")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", tag = "4")]
    pub scriptpubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "5")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub redeemscript: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "listfunds_outputs::ListfundsOutputsStatus", tag = "7")]
    pub status: i32,
    #[prost(uint32, optional, tag = "8")]
    pub blockheight: ::core::option::Option<u32>,
    #[prost(bool, tag = "9")]
    pub reserved: bool,
    #[prost(uint32, optional, tag = "10")]
    pub reserved_to_block: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ListfundsOutputs`.
pub mod listfunds_outputs {
    /// ListFunds.outputs\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListfundsOutputsStatus {
        Unconfirmed = 0,
        Confirmed = 1,
        Spent = 2,
        Immature = 3,
    }
    impl ListfundsOutputsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListfundsOutputsStatus::Unconfirmed => "UNCONFIRMED",
                ListfundsOutputsStatus::Confirmed => "CONFIRMED",
                ListfundsOutputsStatus::Spent => "SPENT",
                ListfundsOutputsStatus::Immature => "IMMATURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNCONFIRMED" => Some(Self::Unconfirmed),
                "CONFIRMED" => Some(Self::Confirmed),
                "SPENT" => Some(Self::Spent),
                "IMMATURE" => Some(Self::Immature),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListfundsChannels {
    #[prost(bytes = "vec", tag = "1")]
    pub peer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub our_amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", tag = "4")]
    pub funding_txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "5")]
    pub funding_output: u32,
    #[prost(bool, tag = "6")]
    pub connected: bool,
    #[prost(enumeration = "ChannelState", tag = "7")]
    pub state: i32,
    #[prost(string, optional, tag = "8")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub channel_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendpayRequest {
    #[prost(message, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<SendpayRoute>,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub payment_secret: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "7")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "10")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub localinvreqid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub payment_metadata: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "13")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendpayResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, optional, tag = "2")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "sendpay_response::SendpayStatus", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "7")]
    pub created_at: u64,
    #[prost(message, optional, tag = "8")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "9")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub partid: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "11")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "14")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "15")]
    pub completed_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "16")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "17")]
    pub updated_index: ::core::option::Option<u64>,
}
/// Nested message and enum types in `SendpayResponse`.
pub mod sendpay_response {
    /// SendPay.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SendpayStatus {
        Pending = 0,
        Complete = 1,
    }
    impl SendpayStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SendpayStatus::Pending => "PENDING",
                SendpayStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendpayRoute {
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub delay: u32,
    #[prost(string, tag = "4")]
    pub channel: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListchannelsRequest {
    #[prost(string, optional, tag = "1")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub source: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListchannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<ListchannelsChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListchannelsChannels {
    #[prost(bytes = "vec", tag = "1")]
    pub source: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub destination: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub short_channel_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub public: bool,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "6")]
    pub message_flags: u32,
    #[prost(uint32, tag = "7")]
    pub channel_flags: u32,
    #[prost(bool, tag = "8")]
    pub active: bool,
    #[prost(uint32, tag = "9")]
    pub last_update: u32,
    #[prost(uint32, tag = "10")]
    pub base_fee_millisatoshi: u32,
    #[prost(uint32, tag = "11")]
    pub fee_per_millionth: u32,
    #[prost(uint32, tag = "12")]
    pub delay: u32,
    #[prost(message, optional, tag = "13")]
    pub htlc_minimum_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "14")]
    pub htlc_maximum_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", tag = "15")]
    pub features: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "16")]
    pub direction: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddgossipRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddgossipResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddpsbtoutputRequest {
    #[prost(message, optional, tag = "1")]
    pub satoshi: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "2")]
    pub locktime: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub initialpsbt: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddpsbtoutputResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub estimated_added_weight: u32,
    #[prost(uint32, tag = "3")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceRequest {
    #[prost(enumeration = "AutocleanSubsystem", tag = "1")]
    pub subsystem: i32,
    #[prost(uint64, tag = "2")]
    pub age: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceResponse {
    #[prost(message, optional, tag = "1")]
    pub autoclean: ::core::option::Option<AutocleanonceAutoclean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutoclean {
    #[prost(message, optional, tag = "1")]
    pub succeededforwards: ::core::option::Option<
        AutocleanonceAutocleanSucceededforwards,
    >,
    #[prost(message, optional, tag = "2")]
    pub failedforwards: ::core::option::Option<AutocleanonceAutocleanFailedforwards>,
    #[prost(message, optional, tag = "3")]
    pub succeededpays: ::core::option::Option<AutocleanonceAutocleanSucceededpays>,
    #[prost(message, optional, tag = "4")]
    pub failedpays: ::core::option::Option<AutocleanonceAutocleanFailedpays>,
    #[prost(message, optional, tag = "5")]
    pub paidinvoices: ::core::option::Option<AutocleanonceAutocleanPaidinvoices>,
    #[prost(message, optional, tag = "6")]
    pub expiredinvoices: ::core::option::Option<AutocleanonceAutocleanExpiredinvoices>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanSucceededforwards {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanFailedforwards {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanSucceededpays {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanFailedpays {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanPaidinvoices {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanonceAutocleanExpiredinvoices {
    #[prost(uint64, tag = "1")]
    pub cleaned: u64,
    #[prost(uint64, tag = "2")]
    pub uncleaned: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusRequest {
    #[prost(enumeration = "AutocleanSubsystem", optional, tag = "1")]
    pub subsystem: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusResponse {
    #[prost(message, optional, tag = "1")]
    pub autoclean: ::core::option::Option<AutocleanstatusAutoclean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutoclean {
    #[prost(message, optional, tag = "1")]
    pub succeededforwards: ::core::option::Option<
        AutocleanstatusAutocleanSucceededforwards,
    >,
    #[prost(message, optional, tag = "2")]
    pub failedforwards: ::core::option::Option<AutocleanstatusAutocleanFailedforwards>,
    #[prost(message, optional, tag = "3")]
    pub succeededpays: ::core::option::Option<AutocleanstatusAutocleanSucceededpays>,
    #[prost(message, optional, tag = "4")]
    pub failedpays: ::core::option::Option<AutocleanstatusAutocleanFailedpays>,
    #[prost(message, optional, tag = "5")]
    pub paidinvoices: ::core::option::Option<AutocleanstatusAutocleanPaidinvoices>,
    #[prost(message, optional, tag = "6")]
    pub expiredinvoices: ::core::option::Option<AutocleanstatusAutocleanExpiredinvoices>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanSucceededforwards {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanFailedforwards {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanSucceededpays {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanFailedpays {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanPaidinvoices {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocleanstatusAutocleanExpiredinvoices {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint64, tag = "2")]
    pub cleaned: u64,
    #[prost(uint64, optional, tag = "3")]
    pub age: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckmessageRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub zbase: ::prost::alloc::string::String,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub pubkey: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckmessageResponse {
    #[prost(bool, tag = "1")]
    pub verified: bool,
    #[prost(bytes = "vec", tag = "2")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "2")]
    pub unilateraltimeout: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub fee_negotiation_step: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub wrong_funding: ::core::option::Option<Outpoint>,
    #[prost(bool, optional, tag = "6")]
    pub force_lease_closed: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub feerange: ::prost::alloc::vec::Vec<Feerate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseResponse {
    #[prost(enumeration = "close_response::CloseType", tag = "1")]
    pub item_type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub tx: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `CloseResponse`.
pub mod close_response {
    /// Close.type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CloseType {
        Mutual = 0,
        Unilateral = 1,
        Unopened = 2,
    }
    impl CloseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CloseType::Mutual => "MUTUAL",
                CloseType::Unilateral => "UNILATERAL",
                CloseType::Unopened => "UNOPENED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MUTUAL" => Some(Self::Mutual),
                "UNILATERAL" => Some(Self::Unilateral),
                "UNOPENED" => Some(Self::Unopened),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub port: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub features: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "connect_response::ConnectDirection", tag = "3")]
    pub direction: i32,
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<ConnectAddress>,
}
/// Nested message and enum types in `ConnectResponse`.
pub mod connect_response {
    /// Connect.direction
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ConnectDirection {
        In = 0,
        Out = 1,
    }
    impl ConnectDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectDirection::In => "IN",
                ConnectDirection::Out => "OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IN" => Some(Self::In),
                "OUT" => Some(Self::Out),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectAddress {
    #[prost(enumeration = "connect_address::ConnectAddressType", tag = "1")]
    pub item_type: i32,
    #[prost(string, optional, tag = "2")]
    pub socket: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub port: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ConnectAddress`.
pub mod connect_address {
    /// Connect.address.type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ConnectAddressType {
        LocalSocket = 0,
        Ipv4 = 1,
        Ipv6 = 2,
        Torv2 = 3,
        Torv3 = 4,
    }
    impl ConnectAddressType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectAddressType::LocalSocket => "LOCAL_SOCKET",
                ConnectAddressType::Ipv4 => "IPV4",
                ConnectAddressType::Ipv6 => "IPV6",
                ConnectAddressType::Torv2 => "TORV2",
                ConnectAddressType::Torv3 => "TORV3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCAL_SOCKET" => Some(Self::LocalSocket),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "TORV2" => Some(Self::Torv2),
                "TORV3" => Some(Self::Torv3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateinvoiceRequest {
    #[prost(string, tag = "1")]
    pub invstring: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateinvoiceResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "4")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(enumeration = "createinvoice_response::CreateinvoiceStatus", tag = "6")]
    pub status: i32,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub expires_at: u64,
    #[prost(uint64, optional, tag = "9")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "10")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "11")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub local_offer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "15")]
    pub invreq_payer_note: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "16")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "17")]
    pub paid_outpoint: ::core::option::Option<CreateinvoicePaidOutpoint>,
}
/// Nested message and enum types in `CreateinvoiceResponse`.
pub mod createinvoice_response {
    /// CreateInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CreateinvoiceStatus {
        Paid = 0,
        Expired = 1,
        Unpaid = 2,
    }
    impl CreateinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CreateinvoiceStatus::Paid => "PAID",
                CreateinvoiceStatus::Expired => "EXPIRED",
                CreateinvoiceStatus::Unpaid => "UNPAID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                "UNPAID" => Some(Self::Unpaid),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateinvoicePaidOutpoint {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreRequest {
    #[prost(bytes = "vec", optional, tag = "2")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "datastore_request::DatastoreMode", optional, tag = "3")]
    pub mode: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "4")]
    pub generation: ::core::option::Option<u64>,
    #[prost(string, repeated, tag = "5")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DatastoreRequest`.
pub mod datastore_request {
    /// Datastore.mode
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DatastoreMode {
        MustCreate = 0,
        MustReplace = 1,
        CreateOrReplace = 2,
        MustAppend = 3,
        CreateOrAppend = 4,
    }
    impl DatastoreMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DatastoreMode::MustCreate => "MUST_CREATE",
                DatastoreMode::MustReplace => "MUST_REPLACE",
                DatastoreMode::CreateOrReplace => "CREATE_OR_REPLACE",
                DatastoreMode::MustAppend => "MUST_APPEND",
                DatastoreMode::CreateOrAppend => "CREATE_OR_APPEND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MUST_CREATE" => Some(Self::MustCreate),
                "MUST_REPLACE" => Some(Self::MustReplace),
                "CREATE_OR_REPLACE" => Some(Self::CreateOrReplace),
                "MUST_APPEND" => Some(Self::MustAppend),
                "CREATE_OR_APPEND" => Some(Self::CreateOrAppend),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreResponse {
    #[prost(uint64, optional, tag = "2")]
    pub generation: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "4")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreusageRequest {
    #[prost(string, repeated, tag = "1")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreusageResponse {
    #[prost(message, optional, tag = "1")]
    pub datastoreusage: ::core::option::Option<DatastoreusageDatastoreusage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreusageDatastoreusage {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub total_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateonionRequest {
    #[prost(message, repeated, tag = "1")]
    pub hops: ::prost::alloc::vec::Vec<CreateonionHops>,
    #[prost(bytes = "vec", tag = "2")]
    pub assocdata: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub session_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub onion_size: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateonionResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub onion: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub shared_secrets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateonionHops {
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeldatastoreRequest {
    #[prost(uint64, optional, tag = "2")]
    pub generation: ::core::option::Option<u64>,
    #[prost(string, repeated, tag = "3")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeldatastoreResponse {
    #[prost(uint64, optional, tag = "2")]
    pub generation: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "4")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelinvoiceRequest {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(enumeration = "delinvoice_request::DelinvoiceStatus", tag = "2")]
    pub status: i32,
    #[prost(bool, optional, tag = "3")]
    pub desconly: ::core::option::Option<bool>,
}
/// Nested message and enum types in `DelinvoiceRequest`.
pub mod delinvoice_request {
    /// DelInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DelinvoiceStatus {
        Paid = 0,
        Expired = 1,
        Unpaid = 2,
    }
    impl DelinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DelinvoiceStatus::Paid => "PAID",
                DelinvoiceStatus::Expired => "EXPIRED",
                DelinvoiceStatus::Unpaid => "UNPAID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                "UNPAID" => Some(Self::Unpaid),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelinvoiceResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "6")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "delinvoice_response::DelinvoiceStatus", tag = "7")]
    pub status: i32,
    #[prost(uint64, tag = "8")]
    pub expires_at: u64,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub local_offer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "11")]
    pub invreq_payer_note: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "12")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "13")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "14")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "15")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "16")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "17")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `DelinvoiceResponse`.
pub mod delinvoice_response {
    /// DelInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DelinvoiceStatus {
        Paid = 0,
        Expired = 1,
        Unpaid = 2,
    }
    impl DelinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DelinvoiceStatus::Paid => "PAID",
                DelinvoiceStatus::Expired => "EXPIRED",
                DelinvoiceStatus::Unpaid => "UNPAID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                "UNPAID" => Some(Self::Unpaid),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevforgetchannelRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "2")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub channel_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "4")]
    pub force: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevforgetchannelResponse {
    #[prost(bool, tag = "1")]
    pub forced: bool,
    #[prost(bool, tag = "2")]
    pub funding_unspent: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub funding_txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyrecoverRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyrecoverResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub stubs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverRequest {
    #[prost(string, tag = "1")]
    pub hsmsecret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverResponse {
    #[prost(enumeration = "recover_response::RecoverResult", optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RecoverResponse`.
pub mod recover_response {
    /// Recover.result
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RecoverResult {
        RecoveryRestartInProgress = 0,
    }
    impl RecoverResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecoverResult::RecoveryRestartInProgress => {
                    "RECOVERY_RESTART_IN_PROGRESS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECOVERY_RESTART_IN_PROGRESS" => Some(Self::RecoveryRestartInProgress),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverchannelRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub scb: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverchannelResponse {
    #[prost(string, repeated, tag = "1")]
    pub stubs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceRequest {
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub fallbacks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "6")]
    pub cltv: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "7")]
    pub expiry: ::core::option::Option<u64>,
    #[prost(string, repeated, tag = "8")]
    pub exposeprivatechannels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub deschashonly: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "10")]
    pub amount_msat: ::core::option::Option<AmountOrAny>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceResponse {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_secret: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub expires_at: u64,
    #[prost(string, optional, tag = "5")]
    pub warning_capacity: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub warning_offline: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub warning_deadends: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub warning_private_unused: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub warning_mpp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub created_index: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoicerequestRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<Amount>,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub issuer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "5")]
    pub absolute_expiry: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "6")]
    pub single_use: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoicerequestResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub invreq_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(string, optional, tag = "6")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableinvoicerequestRequest {
    #[prost(string, tag = "1")]
    pub invreq_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableinvoicerequestResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub invreq_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(string, optional, tag = "6")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicerequestsRequest {
    #[prost(string, optional, tag = "1")]
    pub invreq_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub active_only: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicerequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub invoicerequests: ::prost::alloc::vec::Vec<ListinvoicerequestsInvoicerequests>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicerequestsInvoicerequests {
    #[prost(bytes = "vec", tag = "1")]
    pub invreq_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(string, optional, tag = "6")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListdatastoreRequest {
    #[prost(string, repeated, tag = "2")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListdatastoreResponse {
    #[prost(message, repeated, tag = "1")]
    pub datastore: ::prost::alloc::vec::Vec<ListdatastoreDatastore>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListdatastoreDatastore {
    #[prost(string, repeated, tag = "1")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "2")]
    pub generation: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "4")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicesRequest {
    #[prost(string, optional, tag = "1")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub invstring: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "4")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "listinvoices_request::ListinvoicesIndex",
        optional,
        tag = "5"
    )]
    pub index: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "6")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "7")]
    pub limit: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ListinvoicesRequest`.
pub mod listinvoices_request {
    /// ListInvoices.index
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListinvoicesIndex {
        Created = 0,
        Updated = 1,
    }
    impl ListinvoicesIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListinvoicesIndex::Created => "CREATED",
                ListinvoicesIndex::Updated => "UPDATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATED" => Some(Self::Created),
                "UPDATED" => Some(Self::Updated),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicesResponse {
    #[prost(message, repeated, tag = "1")]
    pub invoices: ::prost::alloc::vec::Vec<ListinvoicesInvoices>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicesInvoices {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(
        enumeration = "listinvoices_invoices::ListinvoicesInvoicesStatus",
        tag = "4"
    )]
    pub status: i32,
    #[prost(uint64, tag = "5")]
    pub expires_at: u64,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub local_offer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "11")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "12")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "13")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "14")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "15")]
    pub invreq_payer_note: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "16")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "17")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "18")]
    pub paid_outpoint: ::core::option::Option<ListinvoicesInvoicesPaidOutpoint>,
}
/// Nested message and enum types in `ListinvoicesInvoices`.
pub mod listinvoices_invoices {
    /// ListInvoices.invoices\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListinvoicesInvoicesStatus {
        Unpaid = 0,
        Paid = 1,
        Expired = 2,
    }
    impl ListinvoicesInvoicesStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListinvoicesInvoicesStatus::Unpaid => "UNPAID",
                ListinvoicesInvoicesStatus::Paid => "PAID",
                ListinvoicesInvoicesStatus::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNPAID" => Some(Self::Unpaid),
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListinvoicesInvoicesPaidOutpoint {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendonionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub onion: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub first_hop: ::core::option::Option<SendonionFirstHop>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "4")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub shared_secrets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "6")]
    pub partid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "11")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "12")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub localinvreqid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "14")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendonionResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "sendonion_response::SendonionStatus", tag = "3")]
    pub status: i32,
    #[prost(message, optional, tag = "4")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "6")]
    pub created_at: u64,
    #[prost(message, optional, tag = "7")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "8")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "12")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "13")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "14")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "15")]
    pub updated_index: ::core::option::Option<u64>,
}
/// Nested message and enum types in `SendonionResponse`.
pub mod sendonion_response {
    /// SendOnion.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SendonionStatus {
        Pending = 0,
        Complete = 1,
    }
    impl SendonionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SendonionStatus::Pending => "PENDING",
                SendonionStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendonionFirstHop {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "3")]
    pub delay: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListsendpaysRequest {
    #[prost(string, optional, tag = "1")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(
        enumeration = "listsendpays_request::ListsendpaysStatus",
        optional,
        tag = "3"
    )]
    pub status: ::core::option::Option<i32>,
    #[prost(
        enumeration = "listsendpays_request::ListsendpaysIndex",
        optional,
        tag = "4"
    )]
    pub index: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "5")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "6")]
    pub limit: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ListsendpaysRequest`.
pub mod listsendpays_request {
    /// ListSendPays.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListsendpaysStatus {
        Pending = 0,
        Complete = 1,
        Failed = 2,
    }
    impl ListsendpaysStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListsendpaysStatus::Pending => "PENDING",
                ListsendpaysStatus::Complete => "COMPLETE",
                ListsendpaysStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "COMPLETE" => Some(Self::Complete),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// ListSendPays.index
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListsendpaysIndex {
        Created = 0,
        Updated = 1,
    }
    impl ListsendpaysIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListsendpaysIndex::Created => "CREATED",
                ListsendpaysIndex::Updated => "UPDATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATED" => Some(Self::Created),
                "UPDATED" => Some(Self::Updated),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListsendpaysResponse {
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<ListsendpaysPayments>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListsendpaysPayments {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub groupid: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(
        enumeration = "listsendpays_payments::ListsendpaysPaymentsStatus",
        tag = "4"
    )]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "7")]
    pub created_at: u64,
    #[prost(message, optional, tag = "8")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "9")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub erroronion: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "14")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "15")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "16")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "17")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "18")]
    pub completed_at: ::core::option::Option<u64>,
}
/// Nested message and enum types in `ListsendpaysPayments`.
pub mod listsendpays_payments {
    /// ListSendPays.payments\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListsendpaysPaymentsStatus {
        Pending = 0,
        Failed = 1,
        Complete = 2,
    }
    impl ListsendpaysPaymentsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListsendpaysPaymentsStatus::Pending => "PENDING",
                ListsendpaysPaymentsStatus::Failed => "FAILED",
                ListsendpaysPaymentsStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListtransactionsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListtransactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<ListtransactionsTransactions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListtransactionsTransactions {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub rawtx: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub blockheight: u32,
    #[prost(uint32, tag = "4")]
    pub txindex: u32,
    #[prost(uint32, tag = "7")]
    pub locktime: u32,
    #[prost(uint32, tag = "8")]
    pub version: u32,
    #[prost(message, repeated, tag = "9")]
    pub inputs: ::prost::alloc::vec::Vec<ListtransactionsTransactionsInputs>,
    #[prost(message, repeated, tag = "10")]
    pub outputs: ::prost::alloc::vec::Vec<ListtransactionsTransactionsOutputs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListtransactionsTransactionsInputs {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(uint32, tag = "3")]
    pub sequence: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListtransactionsTransactionsOutputs {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub script_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakesecretRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "2")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakesecretResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub secret: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayRequest {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "4")]
    pub maxfeepercent: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "5")]
    pub retry_for: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub maxdelay: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub exemptfee: ::core::option::Option<Amount>,
    #[prost(double, optional, tag = "8")]
    pub riskfactor: ::core::option::Option<f64>,
    #[prost(string, repeated, tag = "10")]
    pub exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "11")]
    pub maxfee: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "12")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "13")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "14")]
    pub localinvreqid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "15")]
    pub partial_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_preimage: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "4")]
    pub created_at: f64,
    #[prost(uint32, tag = "5")]
    pub parts: u32,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "7")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "8")]
    pub warning_partial_completion: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "pay_response::PayStatus", tag = "9")]
    pub status: i32,
}
/// Nested message and enum types in `PayResponse`.
pub mod pay_response {
    /// Pay.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PayStatus {
        Complete = 0,
        Pending = 1,
        Failed = 2,
    }
    impl PayStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PayStatus::Complete => "COMPLETE",
                PayStatus::Pending => "PENDING",
                PayStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListnodesRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListnodesResponse {
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<ListnodesNodes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListnodesNodes {
    #[prost(bytes = "vec", tag = "1")]
    pub nodeid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "2")]
    pub last_timestamp: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub alias: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub color: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "6")]
    pub addresses: ::prost::alloc::vec::Vec<ListnodesNodesAddresses>,
    #[prost(message, optional, tag = "7")]
    pub option_will_fund: ::core::option::Option<ListnodesNodesOptionWillFund>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListnodesNodesOptionWillFund {
    #[prost(message, optional, tag = "1")]
    pub lease_fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "2")]
    pub lease_fee_basis: u32,
    #[prost(uint32, tag = "3")]
    pub funding_weight: u32,
    #[prost(message, optional, tag = "4")]
    pub channel_fee_max_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "5")]
    pub channel_fee_max_proportional_thousandths: u32,
    #[prost(bytes = "vec", tag = "6")]
    pub compact_lease: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListnodesNodesAddresses {
    #[prost(
        enumeration = "listnodes_nodes_addresses::ListnodesNodesAddressesType",
        tag = "1"
    )]
    pub item_type: i32,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(string, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ListnodesNodesAddresses`.
pub mod listnodes_nodes_addresses {
    /// ListNodes.nodes\[\].addresses\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListnodesNodesAddressesType {
        Dns = 0,
        Ipv4 = 1,
        Ipv6 = 2,
        Torv2 = 3,
        Torv3 = 4,
    }
    impl ListnodesNodesAddressesType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListnodesNodesAddressesType::Dns => "DNS",
                ListnodesNodesAddressesType::Ipv4 => "IPV4",
                ListnodesNodesAddressesType::Ipv6 => "IPV6",
                ListnodesNodesAddressesType::Torv2 => "TORV2",
                ListnodesNodesAddressesType::Torv3 => "TORV3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DNS" => Some(Self::Dns),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "TORV2" => Some(Self::Torv2),
                "TORV3" => Some(Self::Torv3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitanyinvoiceRequest {
    #[prost(uint64, optional, tag = "1")]
    pub lastpay_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub timeout: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitanyinvoiceResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "waitanyinvoice_response::WaitanyinvoiceStatus", tag = "4")]
    pub status: i32,
    #[prost(uint64, tag = "5")]
    pub expires_at: u64,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "9")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "10")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "11")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "13")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "14")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "15")]
    pub paid_outpoint: ::core::option::Option<WaitanyinvoicePaidOutpoint>,
}
/// Nested message and enum types in `WaitanyinvoiceResponse`.
pub mod waitanyinvoice_response {
    /// WaitAnyInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitanyinvoiceStatus {
        Paid = 0,
        Expired = 1,
    }
    impl WaitanyinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitanyinvoiceStatus::Paid => "PAID",
                WaitanyinvoiceStatus::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitanyinvoicePaidOutpoint {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitinvoiceRequest {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitinvoiceResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "waitinvoice_response::WaitinvoiceStatus", tag = "4")]
    pub status: i32,
    #[prost(uint64, tag = "5")]
    pub expires_at: u64,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "9")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "10")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "11")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "13")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "14")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "15")]
    pub paid_outpoint: ::core::option::Option<WaitinvoicePaidOutpoint>,
}
/// Nested message and enum types in `WaitinvoiceResponse`.
pub mod waitinvoice_response {
    /// WaitInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitinvoiceStatus {
        Paid = 0,
        Expired = 1,
    }
    impl WaitinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitinvoiceStatus::Paid => "PAID",
                WaitinvoiceStatus::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitinvoicePaidOutpoint {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitsendpayRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, optional, tag = "2")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub timeout: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "4")]
    pub groupid: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitsendpayResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, optional, tag = "2")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "waitsendpay_response::WaitsendpayStatus", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "7")]
    pub created_at: u64,
    #[prost(message, optional, tag = "8")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "9")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub partid: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "11")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(double, optional, tag = "14")]
    pub completed_at: ::core::option::Option<f64>,
    #[prost(uint64, optional, tag = "15")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "16")]
    pub updated_index: ::core::option::Option<u64>,
}
/// Nested message and enum types in `WaitsendpayResponse`.
pub mod waitsendpay_response {
    /// WaitSendPay.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitsendpayStatus {
        Complete = 0,
    }
    impl WaitsendpayStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitsendpayStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewaddrRequest {
    #[prost(enumeration = "newaddr_request::NewaddrAddresstype", optional, tag = "1")]
    pub addresstype: ::core::option::Option<i32>,
}
/// Nested message and enum types in `NewaddrRequest`.
pub mod newaddr_request {
    /// NewAddr.addresstype
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NewaddrAddresstype {
        Bech32 = 0,
        All = 2,
        P2tr = 3,
    }
    impl NewaddrAddresstype {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NewaddrAddresstype::Bech32 => "BECH32",
                NewaddrAddresstype::All => "ALL",
                NewaddrAddresstype::P2tr => "P2TR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BECH32" => Some(Self::Bech32),
                "ALL" => Some(Self::All),
                "P2TR" => Some(Self::P2tr),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewaddrResponse {
    #[prost(string, optional, tag = "1")]
    pub bech32: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub p2tr: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRequest {
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub satoshi: ::core::option::Option<AmountOrAll>,
    #[prost(uint32, optional, tag = "3")]
    pub minconf: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
    #[prost(message, optional, tag = "5")]
    pub feerate: ::core::option::Option<Feerate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysendRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub destination: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "4")]
    pub maxfeepercent: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "5")]
    pub retry_for: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub maxdelay: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub exemptfee: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub routehints: ::core::option::Option<RoutehintList>,
    #[prost(message, optional, tag = "9")]
    pub extratlvs: ::core::option::Option<TlvStream>,
    #[prost(message, optional, tag = "10")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysendResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_preimage: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "4")]
    pub created_at: f64,
    #[prost(uint32, tag = "5")]
    pub parts: u32,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "7")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "8")]
    pub warning_partial_completion: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "keysend_response::KeysendStatus", tag = "9")]
    pub status: i32,
}
/// Nested message and enum types in `KeysendResponse`.
pub mod keysend_response {
    /// KeySend.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum KeysendStatus {
        Complete = 0,
    }
    impl KeysendStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeysendStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundpsbtRequest {
    #[prost(message, optional, tag = "1")]
    pub satoshi: ::core::option::Option<AmountOrAll>,
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(uint32, tag = "3")]
    pub startweight: u32,
    #[prost(uint32, optional, tag = "4")]
    pub minconf: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub reserve: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub locktime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub min_witness_weight: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub excess_as_change: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub nonwrapped: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub opening_anchor_channel: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundpsbtResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub feerate_per_kw: u32,
    #[prost(uint32, tag = "3")]
    pub estimated_final_weight: u32,
    #[prost(message, optional, tag = "4")]
    pub excess_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "5")]
    pub change_outnum: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub reservations: ::prost::alloc::vec::Vec<FundpsbtReservations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundpsbtReservations {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(bool, tag = "3")]
    pub was_reserved: bool,
    #[prost(bool, tag = "4")]
    pub reserved: bool,
    #[prost(uint32, tag = "5")]
    pub reserved_to_block: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendpsbtRequest {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "2")]
    pub reserve: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendpsbtResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignpsbtRequest {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag = "2")]
    pub signonly: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignpsbtResponse {
    #[prost(string, tag = "1")]
    pub signed_psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxopsbtRequest {
    #[prost(message, optional, tag = "1")]
    pub satoshi: ::core::option::Option<AmountOrAll>,
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(uint32, tag = "3")]
    pub startweight: u32,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
    #[prost(uint32, optional, tag = "5")]
    pub reserve: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub locktime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub min_witness_weight: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub reservedok: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub excess_as_change: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub opening_anchor_channel: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxopsbtResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub feerate_per_kw: u32,
    #[prost(uint32, tag = "3")]
    pub estimated_final_weight: u32,
    #[prost(message, optional, tag = "4")]
    pub excess_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "5")]
    pub change_outnum: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub reservations: ::prost::alloc::vec::Vec<UtxopsbtReservations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxopsbtReservations {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(bool, tag = "3")]
    pub was_reserved: bool,
    #[prost(bool, tag = "4")]
    pub reserved: bool,
    #[prost(uint32, tag = "5")]
    pub reserved_to_block: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxdiscardRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxdiscardResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub unsigned_tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxprepareRequest {
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(uint32, optional, tag = "3")]
    pub minconf: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<OutputDesc>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxprepareResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub unsigned_tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxsendRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxsendResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<ListpeerchannelsChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannels {
    #[prost(bytes = "vec", tag = "1")]
    pub peer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub peer_connected: bool,
    #[prost(
        enumeration = "listpeerchannels_channels::ListpeerchannelsChannelsState",
        tag = "3"
    )]
    pub state: i32,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub scratch_txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "6")]
    pub feerate: ::core::option::Option<ListpeerchannelsChannelsFeerate>,
    #[prost(string, optional, tag = "7")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub channel_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub funding_txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "11")]
    pub funding_outnum: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "12")]
    pub initial_feerate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub last_feerate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub next_feerate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "15")]
    pub next_fee_step: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "16")]
    pub inflight: ::prost::alloc::vec::Vec<ListpeerchannelsChannelsInflight>,
    #[prost(bytes = "vec", optional, tag = "17")]
    pub close_to: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "18")]
    pub private: ::core::option::Option<bool>,
    #[prost(enumeration = "ChannelSide", tag = "19")]
    pub opener: i32,
    #[prost(enumeration = "ChannelSide", optional, tag = "20")]
    pub closer: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "22")]
    pub funding: ::core::option::Option<ListpeerchannelsChannelsFunding>,
    #[prost(message, optional, tag = "23")]
    pub to_us_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "24")]
    pub min_to_us_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "25")]
    pub max_to_us_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "26")]
    pub total_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "27")]
    pub fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "28")]
    pub fee_proportional_millionths: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "29")]
    pub dust_limit_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "30")]
    pub max_total_htlc_in_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "31")]
    pub their_reserve_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "32")]
    pub our_reserve_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "33")]
    pub spendable_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "34")]
    pub receivable_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "35")]
    pub minimum_htlc_in_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "36")]
    pub minimum_htlc_out_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "37")]
    pub maximum_htlc_out_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "38")]
    pub their_to_self_delay: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "39")]
    pub our_to_self_delay: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub max_accepted_htlcs: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "41")]
    pub alias: ::core::option::Option<ListpeerchannelsChannelsAlias>,
    #[prost(string, repeated, tag = "43")]
    pub status: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "44")]
    pub in_payments_offered: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "45")]
    pub in_offered_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "46")]
    pub in_payments_fulfilled: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "47")]
    pub in_fulfilled_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "48")]
    pub out_payments_offered: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "49")]
    pub out_offered_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "50")]
    pub out_payments_fulfilled: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "51")]
    pub out_fulfilled_msat: ::core::option::Option<Amount>,
    #[prost(message, repeated, tag = "52")]
    pub htlcs: ::prost::alloc::vec::Vec<ListpeerchannelsChannelsHtlcs>,
    #[prost(string, optional, tag = "53")]
    pub close_to_addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "54")]
    pub ignore_fee_limits: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "55")]
    pub updates: ::core::option::Option<ListpeerchannelsChannelsUpdates>,
    #[prost(uint64, optional, tag = "56")]
    pub last_stable_connection: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "57")]
    pub lost_state: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "58")]
    pub reestablished: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "59")]
    pub last_tx_fee_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "60")]
    pub direction: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ListpeerchannelsChannels`.
pub mod listpeerchannels_channels {
    /// ListPeerChannels.channels\[\].state
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpeerchannelsChannelsState {
        Openingd = 0,
        ChanneldAwaitingLockin = 1,
        ChanneldNormal = 2,
        ChanneldShuttingDown = 3,
        ClosingdSigexchange = 4,
        ClosingdComplete = 5,
        AwaitingUnilateral = 6,
        FundingSpendSeen = 7,
        Onchain = 8,
        DualopendOpenInit = 9,
        DualopendAwaitingLockin = 10,
        ChanneldAwaitingSplice = 11,
        DualopendOpenCommitted = 12,
        DualopendOpenCommitReady = 13,
    }
    impl ListpeerchannelsChannelsState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpeerchannelsChannelsState::Openingd => "OPENINGD",
                ListpeerchannelsChannelsState::ChanneldAwaitingLockin => {
                    "CHANNELD_AWAITING_LOCKIN"
                }
                ListpeerchannelsChannelsState::ChanneldNormal => "CHANNELD_NORMAL",
                ListpeerchannelsChannelsState::ChanneldShuttingDown => {
                    "CHANNELD_SHUTTING_DOWN"
                }
                ListpeerchannelsChannelsState::ClosingdSigexchange => {
                    "CLOSINGD_SIGEXCHANGE"
                }
                ListpeerchannelsChannelsState::ClosingdComplete => "CLOSINGD_COMPLETE",
                ListpeerchannelsChannelsState::AwaitingUnilateral => {
                    "AWAITING_UNILATERAL"
                }
                ListpeerchannelsChannelsState::FundingSpendSeen => "FUNDING_SPEND_SEEN",
                ListpeerchannelsChannelsState::Onchain => "ONCHAIN",
                ListpeerchannelsChannelsState::DualopendOpenInit => "DUALOPEND_OPEN_INIT",
                ListpeerchannelsChannelsState::DualopendAwaitingLockin => {
                    "DUALOPEND_AWAITING_LOCKIN"
                }
                ListpeerchannelsChannelsState::ChanneldAwaitingSplice => {
                    "CHANNELD_AWAITING_SPLICE"
                }
                ListpeerchannelsChannelsState::DualopendOpenCommitted => {
                    "DUALOPEND_OPEN_COMMITTED"
                }
                ListpeerchannelsChannelsState::DualopendOpenCommitReady => {
                    "DUALOPEND_OPEN_COMMIT_READY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPENINGD" => Some(Self::Openingd),
                "CHANNELD_AWAITING_LOCKIN" => Some(Self::ChanneldAwaitingLockin),
                "CHANNELD_NORMAL" => Some(Self::ChanneldNormal),
                "CHANNELD_SHUTTING_DOWN" => Some(Self::ChanneldShuttingDown),
                "CLOSINGD_SIGEXCHANGE" => Some(Self::ClosingdSigexchange),
                "CLOSINGD_COMPLETE" => Some(Self::ClosingdComplete),
                "AWAITING_UNILATERAL" => Some(Self::AwaitingUnilateral),
                "FUNDING_SPEND_SEEN" => Some(Self::FundingSpendSeen),
                "ONCHAIN" => Some(Self::Onchain),
                "DUALOPEND_OPEN_INIT" => Some(Self::DualopendOpenInit),
                "DUALOPEND_AWAITING_LOCKIN" => Some(Self::DualopendAwaitingLockin),
                "CHANNELD_AWAITING_SPLICE" => Some(Self::ChanneldAwaitingSplice),
                "DUALOPEND_OPEN_COMMITTED" => Some(Self::DualopendOpenCommitted),
                "DUALOPEND_OPEN_COMMIT_READY" => Some(Self::DualopendOpenCommitReady),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsUpdates {
    #[prost(message, optional, tag = "1")]
    pub local: ::core::option::Option<ListpeerchannelsChannelsUpdatesLocal>,
    #[prost(message, optional, tag = "2")]
    pub remote: ::core::option::Option<ListpeerchannelsChannelsUpdatesRemote>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsUpdatesLocal {
    #[prost(message, optional, tag = "1")]
    pub htlc_minimum_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub htlc_maximum_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "3")]
    pub cltv_expiry_delta: u32,
    #[prost(message, optional, tag = "4")]
    pub fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "5")]
    pub fee_proportional_millionths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsUpdatesRemote {
    #[prost(message, optional, tag = "1")]
    pub htlc_minimum_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub htlc_maximum_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "3")]
    pub cltv_expiry_delta: u32,
    #[prost(message, optional, tag = "4")]
    pub fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "5")]
    pub fee_proportional_millionths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsFeerate {
    #[prost(uint32, tag = "1")]
    pub perkw: u32,
    #[prost(uint32, tag = "2")]
    pub perkb: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsInflight {
    #[prost(bytes = "vec", tag = "1")]
    pub funding_txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub funding_outnum: u32,
    #[prost(string, tag = "3")]
    pub feerate: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub total_funding_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub our_funding_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub scratch_txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(sint64, optional, tag = "7")]
    pub splice_amount: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsFunding {
    #[prost(message, optional, tag = "1")]
    pub pushed_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub local_funds_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub remote_funds_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub fee_paid_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub fee_rcvd_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsAlias {
    #[prost(string, optional, tag = "1")]
    pub local: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub remote: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpeerchannelsChannelsHtlcs {
    #[prost(
        enumeration = "listpeerchannels_channels_htlcs::ListpeerchannelsChannelsHtlcsDirection",
        tag = "1"
    )]
    pub direction: i32,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "4")]
    pub expiry: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, optional, tag = "6")]
    pub local_trimmed: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "HtlcState", tag = "8")]
    pub state: i32,
}
/// Nested message and enum types in `ListpeerchannelsChannelsHtlcs`.
pub mod listpeerchannels_channels_htlcs {
    /// ListPeerChannels.channels\[\].htlcs\[\].direction
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpeerchannelsChannelsHtlcsDirection {
        In = 0,
        Out = 1,
    }
    impl ListpeerchannelsChannelsHtlcsDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpeerchannelsChannelsHtlcsDirection::In => "IN",
                ListpeerchannelsChannelsHtlcsDirection::Out => "OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IN" => Some(Self::In),
                "OUT" => Some(Self::Out),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListclosedchannelsRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListclosedchannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub closedchannels: ::prost::alloc::vec::Vec<ListclosedchannelsClosedchannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListclosedchannelsClosedchannels {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub peer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "3")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub alias: ::core::option::Option<ListclosedchannelsClosedchannelsAlias>,
    #[prost(enumeration = "ChannelSide", tag = "5")]
    pub opener: i32,
    #[prost(enumeration = "ChannelSide", optional, tag = "6")]
    pub closer: ::core::option::Option<i32>,
    #[prost(bool, tag = "7")]
    pub private: bool,
    #[prost(uint64, tag = "9")]
    pub total_local_commitments: u64,
    #[prost(uint64, tag = "10")]
    pub total_remote_commitments: u64,
    #[prost(uint64, tag = "11")]
    pub total_htlcs_sent: u64,
    #[prost(bytes = "vec", tag = "12")]
    pub funding_txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "13")]
    pub funding_outnum: u32,
    #[prost(bool, tag = "14")]
    pub leased: bool,
    #[prost(message, optional, tag = "15")]
    pub funding_fee_paid_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "16")]
    pub funding_fee_rcvd_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "17")]
    pub funding_pushed_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "18")]
    pub total_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "19")]
    pub final_to_us_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "20")]
    pub min_to_us_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "21")]
    pub max_to_us_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "22")]
    pub last_commitment_txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "23")]
    pub last_commitment_fee_msat: ::core::option::Option<Amount>,
    #[prost(
        enumeration = "listclosedchannels_closedchannels::ListclosedchannelsClosedchannelsCloseCause",
        tag = "24"
    )]
    pub close_cause: i32,
    #[prost(uint64, optional, tag = "25")]
    pub last_stable_connection: ::core::option::Option<u64>,
}
/// Nested message and enum types in `ListclosedchannelsClosedchannels`.
pub mod listclosedchannels_closedchannels {
    /// ListClosedChannels.closedchannels\[\].close_cause
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListclosedchannelsClosedchannelsCloseCause {
        Unknown = 0,
        Local = 1,
        User = 2,
        Remote = 3,
        Protocol = 4,
        Onchain = 5,
    }
    impl ListclosedchannelsClosedchannelsCloseCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListclosedchannelsClosedchannelsCloseCause::Unknown => "UNKNOWN",
                ListclosedchannelsClosedchannelsCloseCause::Local => "LOCAL",
                ListclosedchannelsClosedchannelsCloseCause::User => "USER",
                ListclosedchannelsClosedchannelsCloseCause::Remote => "REMOTE",
                ListclosedchannelsClosedchannelsCloseCause::Protocol => "PROTOCOL",
                ListclosedchannelsClosedchannelsCloseCause::Onchain => "ONCHAIN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "LOCAL" => Some(Self::Local),
                "USER" => Some(Self::User),
                "REMOTE" => Some(Self::Remote),
                "PROTOCOL" => Some(Self::Protocol),
                "ONCHAIN" => Some(Self::Onchain),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListclosedchannelsClosedchannelsAlias {
    #[prost(string, optional, tag = "1")]
    pub local: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub remote: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodepayRequest {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodepayResponse {
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub created_at: u64,
    #[prost(uint64, tag = "3")]
    pub expiry: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub payee: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", tag = "6")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub description_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, tag = "10")]
    pub min_final_cltv_expiry: u32,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub payment_secret: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub payment_metadata: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "14")]
    pub fallbacks: ::prost::alloc::vec::Vec<DecodepayFallbacks>,
    #[prost(message, repeated, tag = "16")]
    pub extra: ::prost::alloc::vec::Vec<DecodepayExtra>,
    #[prost(message, optional, tag = "17")]
    pub routes: ::core::option::Option<DecodeRoutehintList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodepayFallbacks {
    #[prost(enumeration = "decodepay_fallbacks::DecodepayFallbacksType", tag = "1")]
    pub item_type: i32,
    #[prost(string, optional, tag = "2")]
    pub addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "3")]
    pub hex: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `DecodepayFallbacks`.
pub mod decodepay_fallbacks {
    /// DecodePay.fallbacks\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DecodepayFallbacksType {
        P2pkh = 0,
        P2sh = 1,
        P2wpkh = 2,
        P2wsh = 3,
        P2tr = 4,
    }
    impl DecodepayFallbacksType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DecodepayFallbacksType::P2pkh => "P2PKH",
                DecodepayFallbacksType::P2sh => "P2SH",
                DecodepayFallbacksType::P2wpkh => "P2WPKH",
                DecodepayFallbacksType::P2wsh => "P2WSH",
                DecodepayFallbacksType::P2tr => "P2TR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "P2PKH" => Some(Self::P2pkh),
                "P2SH" => Some(Self::P2sh),
                "P2WPKH" => Some(Self::P2wpkh),
                "P2WSH" => Some(Self::P2wsh),
                "P2TR" => Some(Self::P2tr),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodepayExtra {
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeRequest {
    #[prost(string, tag = "1")]
    pub string: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeResponse {
    #[prost(enumeration = "decode_response::DecodeType", tag = "1")]
    pub item_type: i32,
    #[prost(bool, tag = "2")]
    pub valid: bool,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub offer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub offer_chains: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub offer_metadata: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "6")]
    pub offer_currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub warning_unknown_offer_currency: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(uint32, optional, tag = "8")]
    pub currency_minor_unit: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "9")]
    pub offer_amount: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "10")]
    pub offer_amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "11")]
    pub offer_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub offer_issuer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub offer_features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "14")]
    pub offer_absolute_expiry: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "15")]
    pub offer_quantity_max: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "16")]
    pub offer_paths: ::prost::alloc::vec::Vec<DecodeOfferPaths>,
    #[prost(bytes = "vec", optional, tag = "17")]
    pub offer_node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "20")]
    pub warning_missing_offer_node_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "21")]
    pub warning_invalid_offer_description: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "22")]
    pub warning_missing_offer_description: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "23")]
    pub warning_invalid_offer_currency: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "24")]
    pub warning_invalid_offer_issuer: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "vec", optional, tag = "25")]
    pub invreq_metadata: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "26")]
    pub invreq_payer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "27")]
    pub invreq_chain: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "28")]
    pub invreq_amount_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "29")]
    pub invreq_features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "30")]
    pub invreq_quantity: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "31")]
    pub invreq_payer_note: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "32")]
    pub invreq_recurrence_counter: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub invreq_recurrence_start: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "35")]
    pub warning_missing_invreq_metadata: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "36")]
    pub warning_missing_invreq_payer_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "37")]
    pub warning_invalid_invreq_payer_note: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "38")]
    pub warning_missing_invoice_request_signature: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "39")]
    pub warning_invalid_invoice_request_signature: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(uint64, optional, tag = "41")]
    pub invoice_created_at: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "42")]
    pub invoice_relative_expiry: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "43")]
    pub invoice_payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "44")]
    pub invoice_amount_msat: ::core::option::Option<Amount>,
    #[prost(message, repeated, tag = "45")]
    pub invoice_fallbacks: ::prost::alloc::vec::Vec<DecodeInvoiceFallbacks>,
    #[prost(bytes = "vec", optional, tag = "46")]
    pub invoice_features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "47")]
    pub invoice_node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "48")]
    pub invoice_recurrence_basetime: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "50")]
    pub warning_missing_invoice_paths: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "51")]
    pub warning_missing_invoice_blindedpay: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "52")]
    pub warning_missing_invoice_created_at: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "53")]
    pub warning_missing_invoice_payment_hash: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "54")]
    pub warning_missing_invoice_amount: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "55")]
    pub warning_missing_invoice_recurrence_basetime: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "56")]
    pub warning_missing_invoice_node_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "57")]
    pub warning_missing_invoice_signature: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "58")]
    pub warning_invalid_invoice_signature: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "59")]
    pub fallbacks: ::prost::alloc::vec::Vec<DecodeFallbacks>,
    #[prost(uint64, optional, tag = "60")]
    pub created_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "61")]
    pub expiry: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "62")]
    pub payee: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "63")]
    pub payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "64")]
    pub description_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "65")]
    pub min_final_cltv_expiry: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "66")]
    pub payment_secret: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "67")]
    pub payment_metadata: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "69")]
    pub extra: ::prost::alloc::vec::Vec<DecodeExtra>,
    #[prost(string, optional, tag = "70")]
    pub unique_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "71")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "72")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "73")]
    pub restrictions: ::prost::alloc::vec::Vec<DecodeRestrictions>,
    #[prost(string, optional, tag = "74")]
    pub warning_rune_invalid_utf8: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "vec", optional, tag = "75")]
    pub hex: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "76")]
    pub decrypted: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "77")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "78")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "79")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "80")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "81")]
    pub features: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "82")]
    pub routes: ::core::option::Option<DecodeRoutehintList>,
    #[prost(bytes = "vec", optional, tag = "83")]
    pub offer_issuer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "84")]
    pub warning_missing_offer_issuer_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "85")]
    pub invreq_paths: ::prost::alloc::vec::Vec<DecodeInvreqPaths>,
    #[prost(string, optional, tag = "86")]
    pub warning_empty_blinded_path: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `DecodeResponse`.
pub mod decode_response {
    /// Decode.type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DecodeType {
        Bolt12Offer = 0,
        Bolt12Invoice = 1,
        Bolt12InvoiceRequest = 2,
        Bolt11Invoice = 3,
        Rune = 4,
        EmergencyRecover = 5,
    }
    impl DecodeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DecodeType::Bolt12Offer => "BOLT12_OFFER",
                DecodeType::Bolt12Invoice => "BOLT12_INVOICE",
                DecodeType::Bolt12InvoiceRequest => "BOLT12_INVOICE_REQUEST",
                DecodeType::Bolt11Invoice => "BOLT11_INVOICE",
                DecodeType::Rune => "RUNE",
                DecodeType::EmergencyRecover => "EMERGENCY_RECOVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BOLT12_OFFER" => Some(Self::Bolt12Offer),
                "BOLT12_INVOICE" => Some(Self::Bolt12Invoice),
                "BOLT12_INVOICE_REQUEST" => Some(Self::Bolt12InvoiceRequest),
                "BOLT11_INVOICE" => Some(Self::Bolt11Invoice),
                "RUNE" => Some(Self::Rune),
                "EMERGENCY_RECOVER" => Some(Self::EmergencyRecover),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeOfferPaths {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub first_node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "2")]
    pub blinding: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "4")]
    pub first_scid_dir: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub first_scid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeOfferRecurrencePaywindow {
    #[prost(uint32, tag = "1")]
    pub seconds_before: u32,
    #[prost(uint32, tag = "2")]
    pub seconds_after: u32,
    #[prost(bool, optional, tag = "3")]
    pub proportional_amount: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeInvreqPaths {
    #[prost(uint32, optional, tag = "1")]
    pub first_scid_dir: ::core::option::Option<u32>,
    #[prost(bytes = "vec", tag = "2")]
    pub blinding: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub first_node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "4")]
    pub first_scid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub path: ::prost::alloc::vec::Vec<DecodeInvreqPathsPath>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeInvreqPathsPath {
    #[prost(bytes = "vec", tag = "1")]
    pub blinded_node_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_recipient_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeInvoicePathsPath {
    #[prost(bytes = "vec", tag = "1")]
    pub blinded_node_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_recipient_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeInvoiceFallbacks {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub hex: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeFallbacks {
    #[prost(string, optional, tag = "1")]
    pub warning_invoice_fallbacks_version_invalid: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "decode_fallbacks::DecodeFallbacksType", tag = "2")]
    pub item_type: i32,
    #[prost(string, optional, tag = "3")]
    pub addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "4")]
    pub hex: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `DecodeFallbacks`.
pub mod decode_fallbacks {
    /// Decode.fallbacks\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DecodeFallbacksType {
        P2pkh = 0,
        P2sh = 1,
        P2wpkh = 2,
        P2wsh = 3,
        P2tr = 4,
    }
    impl DecodeFallbacksType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DecodeFallbacksType::P2pkh => "P2PKH",
                DecodeFallbacksType::P2sh => "P2SH",
                DecodeFallbacksType::P2wpkh => "P2WPKH",
                DecodeFallbacksType::P2wsh => "P2WSH",
                DecodeFallbacksType::P2tr => "P2TR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "P2PKH" => Some(Self::P2pkh),
                "P2SH" => Some(Self::P2sh),
                "P2WPKH" => Some(Self::P2wpkh),
                "P2WSH" => Some(Self::P2wsh),
                "P2TR" => Some(Self::P2tr),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeExtra {
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeRestrictions {
    #[prost(string, repeated, tag = "1")]
    pub alternatives: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelpayRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "delpay_request::DelpayStatus", tag = "2")]
    pub status: i32,
    #[prost(uint64, optional, tag = "3")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub groupid: ::core::option::Option<u64>,
}
/// Nested message and enum types in `DelpayRequest`.
pub mod delpay_request {
    /// DelPay.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DelpayStatus {
        Complete = 0,
        Failed = 1,
    }
    impl DelpayStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DelpayStatus::Complete => "COMPLETE",
                DelpayStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelpayResponse {
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<DelpayPayments>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelpayPayments {
    #[prost(uint64, optional, tag = "1")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "delpay_payments::DelpayPaymentsStatus", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "6")]
    pub partid: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "8")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint64, tag = "9")]
    pub created_at: u64,
    #[prost(uint64, optional, tag = "10")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "11")]
    pub completed_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "12")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "14")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "17")]
    pub erroronion: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `DelpayPayments`.
pub mod delpay_payments {
    /// DelPay.payments\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DelpayPaymentsStatus {
        Pending = 0,
        Failed = 1,
        Complete = 2,
    }
    impl DelpayPaymentsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DelpayPaymentsStatus::Pending => "PENDING",
                DelpayPaymentsStatus::Failed => "FAILED",
                DelpayPaymentsStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelforwardRequest {
    #[prost(string, tag = "1")]
    pub in_channel: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub in_htlc_id: u64,
    #[prost(enumeration = "delforward_request::DelforwardStatus", tag = "3")]
    pub status: i32,
}
/// Nested message and enum types in `DelforwardRequest`.
pub mod delforward_request {
    /// DelForward.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DelforwardStatus {
        Settled = 0,
        LocalFailed = 1,
        Failed = 2,
    }
    impl DelforwardStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DelforwardStatus::Settled => "SETTLED",
                DelforwardStatus::LocalFailed => "LOCAL_FAILED",
                DelforwardStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SETTLED" => Some(Self::Settled),
                "LOCAL_FAILED" => Some(Self::LocalFailed),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelforwardResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableofferRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub offer_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableofferResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub offer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(string, optional, tag = "6")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, optional, tag = "2")]
    pub force: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesRequest {
    #[prost(enumeration = "feerates_request::FeeratesStyle", tag = "1")]
    pub style: i32,
}
/// Nested message and enum types in `FeeratesRequest`.
pub mod feerates_request {
    /// Feerates.style
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FeeratesStyle {
        Perkb = 0,
        Perkw = 1,
    }
    impl FeeratesStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeeratesStyle::Perkb => "PERKB",
                FeeratesStyle::Perkw => "PERKW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERKB" => Some(Self::Perkb),
                "PERKW" => Some(Self::Perkw),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesResponse {
    #[prost(string, optional, tag = "1")]
    pub warning_missing_feerates: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub perkb: ::core::option::Option<FeeratesPerkb>,
    #[prost(message, optional, tag = "3")]
    pub perkw: ::core::option::Option<FeeratesPerkw>,
    #[prost(message, optional, tag = "4")]
    pub onchain_fee_estimates: ::core::option::Option<FeeratesOnchainFeeEstimates>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesPerkb {
    #[prost(uint32, tag = "1")]
    pub min_acceptable: u32,
    #[prost(uint32, tag = "2")]
    pub max_acceptable: u32,
    #[prost(uint32, optional, tag = "3")]
    pub opening: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub mutual_close: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub unilateral_close: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub delayed_to_us: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub htlc_resolution: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub penalty: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "9")]
    pub estimates: ::prost::alloc::vec::Vec<FeeratesPerkbEstimates>,
    #[prost(uint32, optional, tag = "10")]
    pub floor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub unilateral_anchor_close: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesPerkbEstimates {
    #[prost(uint32, tag = "1")]
    pub blockcount: u32,
    #[prost(uint32, tag = "2")]
    pub feerate: u32,
    #[prost(uint32, tag = "3")]
    pub smoothed_feerate: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesPerkw {
    #[prost(uint32, tag = "1")]
    pub min_acceptable: u32,
    #[prost(uint32, tag = "2")]
    pub max_acceptable: u32,
    #[prost(uint32, optional, tag = "3")]
    pub opening: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub mutual_close: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub unilateral_close: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub delayed_to_us: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub htlc_resolution: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub penalty: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "9")]
    pub estimates: ::prost::alloc::vec::Vec<FeeratesPerkwEstimates>,
    #[prost(uint32, optional, tag = "10")]
    pub floor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub unilateral_anchor_close: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesPerkwEstimates {
    #[prost(uint32, tag = "1")]
    pub blockcount: u32,
    #[prost(uint32, tag = "2")]
    pub feerate: u32,
    #[prost(uint32, tag = "3")]
    pub smoothed_feerate: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeratesOnchainFeeEstimates {
    #[prost(uint64, tag = "1")]
    pub opening_channel_satoshis: u64,
    #[prost(uint64, tag = "2")]
    pub mutual_close_satoshis: u64,
    #[prost(uint64, tag = "3")]
    pub unilateral_close_satoshis: u64,
    #[prost(uint64, tag = "4")]
    pub htlc_timeout_satoshis: u64,
    #[prost(uint64, tag = "5")]
    pub htlc_success_satoshis: u64,
    #[prost(uint64, optional, tag = "6")]
    pub unilateral_close_nonanchor_satoshis: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchinvoiceRequest {
    #[prost(string, tag = "1")]
    pub offer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "3")]
    pub quantity: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub recurrence_counter: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "5")]
    pub recurrence_start: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "6")]
    pub recurrence_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "7")]
    pub timeout: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "8")]
    pub payer_note: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchinvoiceResponse {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub changes: ::core::option::Option<FetchinvoiceChanges>,
    #[prost(message, optional, tag = "3")]
    pub next_period: ::core::option::Option<FetchinvoiceNextPeriod>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchinvoiceChanges {
    #[prost(string, optional, tag = "1")]
    pub description_appended: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub vendor_removed: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub vendor: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchinvoiceNextPeriod {
    #[prost(uint64, tag = "1")]
    pub counter: u64,
    #[prost(uint64, tag = "2")]
    pub starttime: u64,
    #[prost(uint64, tag = "3")]
    pub endtime: u64,
    #[prost(uint64, tag = "4")]
    pub paywindow_start: u64,
    #[prost(uint64, tag = "5")]
    pub paywindow_end: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelCancelRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelCancelResponse {
    #[prost(string, tag = "1")]
    pub cancelled: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelCompleteRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelCompleteResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub commitments_secured: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<AmountOrAll>,
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(bool, optional, tag = "3")]
    pub announce: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub push_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "6")]
    pub close_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub request_amt: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "8")]
    pub compact_lease: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "9")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "10")]
    pub minconf: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "11")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
    #[prost(uint32, optional, tag = "12")]
    pub mindepth: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "13")]
    pub reserve: ::core::option::Option<Amount>,
    #[prost(uint32, repeated, tag = "14")]
    pub channel_type: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub outnum: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub close_to: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "6")]
    pub mindepth: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub channel_type: ::core::option::Option<FundchannelChannelType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelStartRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(bool, optional, tag = "4")]
    pub announce: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub close_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub push_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "7")]
    pub mindepth: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub reserve: ::core::option::Option<Amount>,
    #[prost(uint32, repeated, tag = "9")]
    pub channel_type: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelStartResponse {
    #[prost(string, tag = "1")]
    pub funding_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub scriptpubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub channel_type: ::core::option::Option<FundchannelStartChannelType>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub close_to: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "5")]
    pub warning_usage: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "6")]
    pub mindepth: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundchannelStartChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetlogRequest {
    #[prost(enumeration = "getlog_request::GetlogLevel", optional, tag = "1")]
    pub level: ::core::option::Option<i32>,
}
/// Nested message and enum types in `GetlogRequest`.
pub mod getlog_request {
    /// GetLog.level
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GetlogLevel {
        Broken = 0,
        Unusual = 1,
        Info = 2,
        Debug = 3,
        Io = 4,
        Trace = 5,
    }
    impl GetlogLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GetlogLevel::Broken => "BROKEN",
                GetlogLevel::Unusual => "UNUSUAL",
                GetlogLevel::Info => "INFO",
                GetlogLevel::Debug => "DEBUG",
                GetlogLevel::Io => "IO",
                GetlogLevel::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BROKEN" => Some(Self::Broken),
                "UNUSUAL" => Some(Self::Unusual),
                "INFO" => Some(Self::Info),
                "DEBUG" => Some(Self::Debug),
                "IO" => Some(Self::Io),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetlogResponse {
    #[prost(string, tag = "1")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub bytes_used: u32,
    #[prost(uint32, tag = "3")]
    pub bytes_max: u32,
    #[prost(message, repeated, tag = "4")]
    pub log: ::prost::alloc::vec::Vec<GetlogLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetlogLog {
    #[prost(enumeration = "getlog_log::GetlogLogType", tag = "1")]
    pub item_type: i32,
    #[prost(uint32, optional, tag = "2")]
    pub num_skipped: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub log: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub node_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `GetlogLog`.
pub mod getlog_log {
    /// GetLog.log\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GetlogLogType {
        Skipped = 0,
        Broken = 1,
        Unusual = 2,
        Info = 3,
        Debug = 4,
        IoIn = 5,
        IoOut = 6,
        Trace = 7,
    }
    impl GetlogLogType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GetlogLogType::Skipped => "SKIPPED",
                GetlogLogType::Broken => "BROKEN",
                GetlogLogType::Unusual => "UNUSUAL",
                GetlogLogType::Info => "INFO",
                GetlogLogType::Debug => "DEBUG",
                GetlogLogType::IoIn => "IO_IN",
                GetlogLogType::IoOut => "IO_OUT",
                GetlogLogType::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SKIPPED" => Some(Self::Skipped),
                "BROKEN" => Some(Self::Broken),
                "UNUSUAL" => Some(Self::Unusual),
                "INFO" => Some(Self::Info),
                "DEBUG" => Some(Self::Debug),
                "IO_IN" => Some(Self::IoIn),
                "IO_OUT" => Some(Self::IoOut),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunderupdateRequest {
    #[prost(
        enumeration = "funderupdate_request::FunderupdatePolicy",
        optional,
        tag = "1"
    )]
    pub policy: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub policy_mod: ::core::option::Option<Amount>,
    #[prost(bool, optional, tag = "3")]
    pub leases_only: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "4")]
    pub min_their_funding_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub max_their_funding_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub per_channel_min_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "7")]
    pub per_channel_max_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub reserve_tank_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "9")]
    pub fuzz_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub fund_probability: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "11")]
    pub lease_fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "12")]
    pub lease_fee_basis: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub funding_weight: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "14")]
    pub channel_fee_max_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "15")]
    pub channel_fee_max_proportional_thousandths: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "16")]
    pub compact_lease: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `FunderupdateRequest`.
pub mod funderupdate_request {
    /// FunderUpdate.policy
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FunderupdatePolicy {
        Match = 0,
        Available = 1,
        Fixed = 2,
    }
    impl FunderupdatePolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FunderupdatePolicy::Match => "MATCH",
                FunderupdatePolicy::Available => "AVAILABLE",
                FunderupdatePolicy::Fixed => "FIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCH" => Some(Self::Match),
                "AVAILABLE" => Some(Self::Available),
                "FIXED" => Some(Self::Fixed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunderupdateResponse {
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
    #[prost(enumeration = "funderupdate_response::FunderupdatePolicy", tag = "2")]
    pub policy: i32,
    #[prost(uint32, tag = "3")]
    pub policy_mod: u32,
    #[prost(bool, tag = "4")]
    pub leases_only: bool,
    #[prost(message, optional, tag = "5")]
    pub min_their_funding_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub max_their_funding_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "7")]
    pub per_channel_min_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub per_channel_max_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "9")]
    pub reserve_tank_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "10")]
    pub fuzz_percent: u32,
    #[prost(uint32, tag = "11")]
    pub fund_probability: u32,
    #[prost(message, optional, tag = "12")]
    pub lease_fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "13")]
    pub lease_fee_basis: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub funding_weight: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "15")]
    pub channel_fee_max_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "16")]
    pub channel_fee_max_proportional_thousandths: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "17")]
    pub compact_lease: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `FunderupdateResponse`.
pub mod funderupdate_response {
    /// FunderUpdate.policy
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FunderupdatePolicy {
        Match = 0,
        Available = 1,
        Fixed = 2,
    }
    impl FunderupdatePolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FunderupdatePolicy::Match => "MATCH",
                FunderupdatePolicy::Available => "AVAILABLE",
                FunderupdatePolicy::Fixed => "FIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCH" => Some(Self::Match),
                "AVAILABLE" => Some(Self::Available),
                "FIXED" => Some(Self::Fixed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetrouteRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub riskfactor: u64,
    #[prost(uint32, optional, tag = "4")]
    pub cltv: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub fromid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "6")]
    pub fuzzpercent: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "7")]
    pub exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "8")]
    pub maxhops: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "9")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetrouteResponse {
    #[prost(message, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<GetrouteRoute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetrouteRoute {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub direction: u32,
    #[prost(message, optional, tag = "4")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "5")]
    pub delay: u32,
    #[prost(enumeration = "getroute_route::GetrouteRouteStyle", tag = "6")]
    pub style: i32,
}
/// Nested message and enum types in `GetrouteRoute`.
pub mod getroute_route {
    /// GetRoute.route\[\].style
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GetrouteRouteStyle {
        Tlv = 0,
    }
    impl GetrouteRouteStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GetrouteRouteStyle::Tlv => "TLV",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TLV" => Some(Self::Tlv),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListforwardsRequest {
    #[prost(
        enumeration = "listforwards_request::ListforwardsStatus",
        optional,
        tag = "1"
    )]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub in_channel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub out_channel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "listforwards_request::ListforwardsIndex",
        optional,
        tag = "4"
    )]
    pub index: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "5")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "6")]
    pub limit: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ListforwardsRequest`.
pub mod listforwards_request {
    /// ListForwards.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListforwardsStatus {
        Offered = 0,
        Settled = 1,
        LocalFailed = 2,
        Failed = 3,
    }
    impl ListforwardsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListforwardsStatus::Offered => "OFFERED",
                ListforwardsStatus::Settled => "SETTLED",
                ListforwardsStatus::LocalFailed => "LOCAL_FAILED",
                ListforwardsStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OFFERED" => Some(Self::Offered),
                "SETTLED" => Some(Self::Settled),
                "LOCAL_FAILED" => Some(Self::LocalFailed),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// ListForwards.index
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListforwardsIndex {
        Created = 0,
        Updated = 1,
    }
    impl ListforwardsIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListforwardsIndex::Created => "CREATED",
                ListforwardsIndex::Updated => "UPDATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATED" => Some(Self::Created),
                "UPDATED" => Some(Self::Updated),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListforwardsResponse {
    #[prost(message, repeated, tag = "1")]
    pub forwards: ::prost::alloc::vec::Vec<ListforwardsForwards>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListforwardsForwards {
    #[prost(string, tag = "1")]
    pub in_channel: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub in_msat: ::core::option::Option<Amount>,
    #[prost(
        enumeration = "listforwards_forwards::ListforwardsForwardsStatus",
        tag = "3"
    )]
    pub status: i32,
    #[prost(double, tag = "4")]
    pub received_time: f64,
    #[prost(string, optional, tag = "5")]
    pub out_channel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub fee_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub out_msat: ::core::option::Option<Amount>,
    #[prost(
        enumeration = "listforwards_forwards::ListforwardsForwardsStyle",
        optional,
        tag = "9"
    )]
    pub style: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "10")]
    pub in_htlc_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "11")]
    pub out_htlc_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "12")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "13")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "14")]
    pub resolved_time: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "15")]
    pub failcode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "16")]
    pub failreason: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ListforwardsForwards`.
pub mod listforwards_forwards {
    /// ListForwards.forwards\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListforwardsForwardsStatus {
        Offered = 0,
        Settled = 1,
        LocalFailed = 2,
        Failed = 3,
    }
    impl ListforwardsForwardsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListforwardsForwardsStatus::Offered => "OFFERED",
                ListforwardsForwardsStatus::Settled => "SETTLED",
                ListforwardsForwardsStatus::LocalFailed => "LOCAL_FAILED",
                ListforwardsForwardsStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OFFERED" => Some(Self::Offered),
                "SETTLED" => Some(Self::Settled),
                "LOCAL_FAILED" => Some(Self::LocalFailed),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// ListForwards.forwards\[\].style
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListforwardsForwardsStyle {
        Legacy = 0,
        Tlv = 1,
    }
    impl ListforwardsForwardsStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListforwardsForwardsStyle::Legacy => "LEGACY",
                ListforwardsForwardsStyle::Tlv => "TLV",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LEGACY" => Some(Self::Legacy),
                "TLV" => Some(Self::Tlv),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListoffersRequest {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub offer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "2")]
    pub active_only: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListoffersResponse {
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<ListoffersOffers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListoffersOffers {
    #[prost(bytes = "vec", tag = "1")]
    pub offer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(string, optional, tag = "6")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpaysRequest {
    #[prost(string, optional, tag = "1")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "listpays_request::ListpaysStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ListpaysRequest`.
pub mod listpays_request {
    /// ListPays.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpaysStatus {
        Pending = 0,
        Complete = 1,
        Failed = 2,
    }
    impl ListpaysStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpaysStatus::Pending => "PENDING",
                ListpaysStatus::Complete => "COMPLETE",
                ListpaysStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "COMPLETE" => Some(Self::Complete),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpaysResponse {
    #[prost(message, repeated, tag = "1")]
    pub pays: ::prost::alloc::vec::Vec<ListpaysPays>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListpaysPays {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "listpays_pays::ListpaysPaysStatus", tag = "2")]
    pub status: i32,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "4")]
    pub created_at: u64,
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "9")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub erroronion: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "11")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "12")]
    pub completed_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "14")]
    pub number_of_parts: ::core::option::Option<u64>,
}
/// Nested message and enum types in `ListpaysPays`.
pub mod listpays_pays {
    /// ListPays.pays\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListpaysPaysStatus {
        Pending = 0,
        Failed = 1,
        Complete = 2,
    }
    impl ListpaysPaysStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListpaysPaysStatus::Pending => "PENDING",
                ListpaysPaysStatus::Failed => "FAILED",
                ListpaysPaysStatus::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListhtlcsRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListhtlcsResponse {
    #[prost(message, repeated, tag = "1")]
    pub htlcs: ::prost::alloc::vec::Vec<ListhtlcsHtlcs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListhtlcsHtlcs {
    #[prost(string, tag = "1")]
    pub short_channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(uint32, tag = "3")]
    pub expiry: u32,
    #[prost(message, optional, tag = "4")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(enumeration = "listhtlcs_htlcs::ListhtlcsHtlcsDirection", tag = "5")]
    pub direction: i32,
    #[prost(bytes = "vec", tag = "6")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "HtlcState", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `ListhtlcsHtlcs`.
pub mod listhtlcs_htlcs {
    /// ListHtlcs.htlcs\[\].direction
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListhtlcsHtlcsDirection {
        Out = 0,
        In = 1,
    }
    impl ListhtlcsHtlcsDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListhtlcsHtlcsDirection::Out => "OUT",
                ListhtlcsHtlcsDirection::In => "IN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OUT" => Some(Self::Out),
                "IN" => Some(Self::In),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelRequest {
    #[prost(message, repeated, tag = "1")]
    pub destinations: ::prost::alloc::vec::Vec<MultifundchannelDestinations>,
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(sint64, optional, tag = "3")]
    pub minconf: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
    #[prost(sint64, optional, tag = "5")]
    pub minchannels: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "6")]
    pub commitment_feerate: ::core::option::Option<Feerate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub channel_ids: ::prost::alloc::vec::Vec<MultifundchannelChannelIds>,
    #[prost(message, repeated, tag = "4")]
    pub failed: ::prost::alloc::vec::Vec<MultifundchannelFailed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelDestinations {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<AmountOrAll>,
    #[prost(bool, optional, tag = "3")]
    pub announce: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "4")]
    pub push_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "5")]
    pub close_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub request_amt: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub compact_lease: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "8")]
    pub mindepth: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "9")]
    pub reserve: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelChannelIds {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub channel_type: ::core::option::Option<MultifundchannelChannelIdsChannelType>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub close_to: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelChannelIdsChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelFailed {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(
        enumeration = "multifundchannel_failed::MultifundchannelFailedMethod",
        tag = "2"
    )]
    pub method: i32,
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<MultifundchannelFailedError>,
}
/// Nested message and enum types in `MultifundchannelFailed`.
pub mod multifundchannel_failed {
    /// MultiFundChannel.failed\[\].method
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MultifundchannelFailedMethod {
        Connect = 0,
        OpenchannelInit = 1,
        FundchannelStart = 2,
        FundchannelComplete = 3,
    }
    impl MultifundchannelFailedMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MultifundchannelFailedMethod::Connect => "CONNECT",
                MultifundchannelFailedMethod::OpenchannelInit => "OPENCHANNEL_INIT",
                MultifundchannelFailedMethod::FundchannelStart => "FUNDCHANNEL_START",
                MultifundchannelFailedMethod::FundchannelComplete => {
                    "FUNDCHANNEL_COMPLETE"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT" => Some(Self::Connect),
                "OPENCHANNEL_INIT" => Some(Self::OpenchannelInit),
                "FUNDCHANNEL_START" => Some(Self::FundchannelStart),
                "FUNDCHANNEL_COMPLETE" => Some(Self::FundchannelComplete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultifundchannelFailedError {
    #[prost(sint64, tag = "1")]
    pub code: i64,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiwithdrawRequest {
    #[prost(message, repeated, tag = "1")]
    pub outputs: ::prost::alloc::vec::Vec<OutputDesc>,
    #[prost(message, optional, tag = "2")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(uint32, optional, tag = "3")]
    pub minconf: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Outpoint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiwithdrawResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferRequest {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub issuer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "5")]
    pub quantity_max: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "6")]
    pub absolute_expiry: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "7")]
    pub recurrence: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub recurrence_base: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub recurrence_paywindow: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "10")]
    pub recurrence_limit: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "11")]
    pub single_use: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "12")]
    pub recurrence_start_any_period: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub offer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub single_use: bool,
    #[prost(string, tag = "4")]
    pub bolt12: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub used: bool,
    #[prost(bool, tag = "6")]
    pub created: bool,
    #[prost(string, optional, tag = "7")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelAbortRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelAbortResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub channel_canceled: bool,
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelBumpRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub initialpsbt: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub funding_feerate: ::core::option::Option<Feerate>,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelBumpResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub channel_type: ::core::option::Option<OpenchannelBumpChannelType>,
    #[prost(string, tag = "3")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub commitments_secured: bool,
    #[prost(uint64, tag = "5")]
    pub funding_serial: u64,
    #[prost(bool, optional, tag = "6")]
    pub requires_confirmed_inputs: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelBumpChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelInitRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub initialpsbt: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub commitment_feerate: ::core::option::Option<Feerate>,
    #[prost(message, optional, tag = "4")]
    pub funding_feerate: ::core::option::Option<Feerate>,
    #[prost(bool, optional, tag = "5")]
    pub announce: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub close_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub request_amt: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub compact_lease: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, tag = "9")]
    pub channel_type: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "10")]
    pub amount: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelInitResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub channel_type: ::core::option::Option<OpenchannelInitChannelType>,
    #[prost(bool, tag = "4")]
    pub commitments_secured: bool,
    #[prost(uint64, tag = "5")]
    pub funding_serial: u64,
    #[prost(bool, optional, tag = "6")]
    pub requires_confirmed_inputs: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelInitChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelSignedRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub signed_psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelSignedResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelUpdateRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelUpdateResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub channel_type: ::core::option::Option<OpenchannelUpdateChannelType>,
    #[prost(string, tag = "3")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub commitments_secured: bool,
    #[prost(uint32, tag = "5")]
    pub funding_outnum: u32,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub close_to: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "7")]
    pub requires_confirmed_inputs: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenchannelUpdateChannelType {
    #[prost(uint32, repeated, tag = "1")]
    pub bits: ::prost::alloc::vec::Vec<u32>,
    #[prost(enumeration = "ChannelTypeName", repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "2")]
    pub len: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub pongbytes: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(uint32, tag = "1")]
    pub totlen: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginRequest {
    #[prost(enumeration = "PluginSubcommand", tag = "1")]
    pub subcommand: i32,
    #[prost(string, optional, tag = "2")]
    pub plugin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub directory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginResponse {
    #[prost(enumeration = "PluginSubcommand", tag = "1")]
    pub command: i32,
    #[prost(message, repeated, tag = "2")]
    pub plugins: ::prost::alloc::vec::Vec<PluginPlugins>,
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginPlugins {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(bool, tag = "3")]
    pub dynamic: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenepaystatusRequest {
    #[prost(string, optional, tag = "1")]
    pub invstring: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenepaystatusResponse {
    #[prost(message, repeated, tag = "1")]
    pub paystatus: ::prost::alloc::vec::Vec<RenepaystatusPaystatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenepaystatusPaystatus {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "4")]
    pub created_at: f64,
    #[prost(uint32, tag = "5")]
    pub groupid: u32,
    #[prost(uint32, optional, tag = "6")]
    pub parts: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(
        enumeration = "renepaystatus_paystatus::RenepaystatusPaystatusStatus",
        tag = "9"
    )]
    pub status: i32,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "11")]
    pub notes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RenepaystatusPaystatus`.
pub mod renepaystatus_paystatus {
    /// RenePayStatus.paystatus\[\].status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RenepaystatusPaystatusStatus {
        Complete = 0,
        Pending = 1,
        Failed = 2,
    }
    impl RenepaystatusPaystatusStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RenepaystatusPaystatusStatus::Complete => "COMPLETE",
                RenepaystatusPaystatusStatus::Pending => "PENDING",
                RenepaystatusPaystatusStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenepayRequest {
    #[prost(string, tag = "1")]
    pub invstring: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub maxfee: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "4")]
    pub maxdelay: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub retry_for: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub dev_use_shadow: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenepayResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub payment_preimage: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "3")]
    pub created_at: f64,
    #[prost(uint32, tag = "4")]
    pub parts: u32,
    #[prost(message, optional, tag = "5")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub amount_sent_msat: ::core::option::Option<Amount>,
    #[prost(enumeration = "renepay_response::RenepayStatus", tag = "7")]
    pub status: i32,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub destination: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `RenepayResponse`.
pub mod renepay_response {
    /// RenePay.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RenepayStatus {
        Complete = 0,
        Pending = 1,
        Failed = 2,
    }
    impl RenepayStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RenepayStatus::Complete => "COMPLETE",
                RenepayStatus::Pending => "PENDING",
                RenepayStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETE" => Some(Self::Complete),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveinputsRequest {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "2")]
    pub exclusive: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "3")]
    pub reserve: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveinputsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::prost::alloc::vec::Vec<ReserveinputsReservations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveinputsReservations {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(bool, tag = "3")]
    pub was_reserved: bool,
    #[prost(bool, tag = "4")]
    pub reserved: bool,
    #[prost(uint32, tag = "5")]
    pub reserved_to_block: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendcustommsgRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendcustommsgResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendinvoiceRequest {
    #[prost(string, tag = "1")]
    pub invreq: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "4")]
    pub timeout: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "5")]
    pub quantity: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendinvoiceResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "sendinvoice_response::SendinvoiceStatus", tag = "4")]
    pub status: i32,
    #[prost(uint64, tag = "5")]
    pub expires_at: u64,
    #[prost(message, optional, tag = "6")]
    pub amount_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "8")]
    pub created_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub updated_index: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "10")]
    pub pay_index: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "11")]
    pub amount_received_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "12")]
    pub paid_at: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub payment_preimage: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `SendinvoiceResponse`.
pub mod sendinvoice_response {
    /// SendInvoice.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SendinvoiceStatus {
        Unpaid = 0,
        Paid = 1,
        Expired = 2,
    }
    impl SendinvoiceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SendinvoiceStatus::Unpaid => "UNPAID",
                SendinvoiceStatus::Paid => "PAID",
                SendinvoiceStatus::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNPAID" => Some(Self::Unpaid),
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetchannelRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub feebase: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "3")]
    pub feeppm: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub htlcmin: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub htlcmax: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "6")]
    pub enforcedelay: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub ignorefeelimits: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetchannelResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<SetchannelChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetchannelChannels {
    #[prost(bytes = "vec", tag = "1")]
    pub peer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "3")]
    pub short_channel_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub fee_base_msat: ::core::option::Option<Amount>,
    #[prost(uint32, tag = "5")]
    pub fee_proportional_millionths: u32,
    #[prost(message, optional, tag = "6")]
    pub minimum_htlc_out_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub warning_htlcmin_too_low: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub maximum_htlc_out_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "9")]
    pub warning_htlcmax_too_high: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "10")]
    pub ignore_fee_limits: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetconfigRequest {
    #[prost(string, tag = "1")]
    pub config: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetconfigResponse {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<SetconfigConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetconfigConfig {
    #[prost(string, tag = "1")]
    pub config: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub plugin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag = "4")]
    pub dynamic: bool,
    #[prost(bool, optional, tag = "5")]
    pub set: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub value_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub value_msat: ::core::option::Option<Amount>,
    #[prost(sint64, optional, tag = "8")]
    pub value_int: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "9")]
    pub value_bool: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetpsbtversionRequest {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub version: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetpsbtversionResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigninvoiceRequest {
    #[prost(string, tag = "1")]
    pub invstring: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigninvoiceResponse {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignmessageRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignmessageResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub recid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub zbase: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceInitRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "2")]
    pub relative_amount: i64,
    #[prost(string, optional, tag = "3")]
    pub initialpsbt: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub feerate_per_kw: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub force_feerate: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceInitResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceSignedRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "3")]
    pub sign_first: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceSignedResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "3")]
    pub outnum: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceUpdateRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub psbt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceUpdateResponse {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub commitments_secured: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnreserveinputsRequest {
    #[prost(string, tag = "1")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "2")]
    pub reserve: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnreserveinputsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::prost::alloc::vec::Vec<UnreserveinputsReservations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnreserveinputsReservations {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(bool, tag = "3")]
    pub was_reserved: bool,
    #[prost(bool, tag = "4")]
    pub reserved: bool,
    #[prost(uint32, optional, tag = "5")]
    pub reserved_to_block: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradewalletRequest {
    #[prost(message, optional, tag = "1")]
    pub feerate: ::core::option::Option<Feerate>,
    #[prost(bool, optional, tag = "2")]
    pub reservedok: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradewalletResponse {
    #[prost(uint64, optional, tag = "1")]
    pub upgraded_outs: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub psbt: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub tx: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitblockheightRequest {
    #[prost(uint32, tag = "1")]
    pub blockheight: u32,
    #[prost(uint32, optional, tag = "2")]
    pub timeout: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitblockheightResponse {
    #[prost(uint32, tag = "1")]
    pub blockheight: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitRequest {
    #[prost(enumeration = "wait_request::WaitSubsystem", tag = "1")]
    pub subsystem: i32,
    #[prost(enumeration = "wait_request::WaitIndexname", tag = "2")]
    pub indexname: i32,
    #[prost(uint64, tag = "3")]
    pub nextvalue: u64,
}
/// Nested message and enum types in `WaitRequest`.
pub mod wait_request {
    /// Wait.subsystem
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitSubsystem {
        Invoices = 0,
        Forwards = 1,
        Sendpays = 2,
    }
    impl WaitSubsystem {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitSubsystem::Invoices => "INVOICES",
                WaitSubsystem::Forwards => "FORWARDS",
                WaitSubsystem::Sendpays => "SENDPAYS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVOICES" => Some(Self::Invoices),
                "FORWARDS" => Some(Self::Forwards),
                "SENDPAYS" => Some(Self::Sendpays),
                _ => None,
            }
        }
    }
    /// Wait.indexname
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitIndexname {
        Created = 0,
        Updated = 1,
        Deleted = 2,
    }
    impl WaitIndexname {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitIndexname::Created => "CREATED",
                WaitIndexname::Updated => "UPDATED",
                WaitIndexname::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATED" => Some(Self::Created),
                "UPDATED" => Some(Self::Updated),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitResponse {
    #[prost(enumeration = "wait_response::WaitSubsystem", tag = "1")]
    pub subsystem: i32,
    #[prost(uint64, optional, tag = "2")]
    pub created: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub updated: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub deleted: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "5")]
    pub details: ::core::option::Option<WaitDetails>,
}
/// Nested message and enum types in `WaitResponse`.
pub mod wait_response {
    /// Wait.subsystem
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitSubsystem {
        Invoices = 0,
        Forwards = 1,
        Sendpays = 2,
    }
    impl WaitSubsystem {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitSubsystem::Invoices => "INVOICES",
                WaitSubsystem::Forwards => "FORWARDS",
                WaitSubsystem::Sendpays => "SENDPAYS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVOICES" => Some(Self::Invoices),
                "FORWARDS" => Some(Self::Forwards),
                "SENDPAYS" => Some(Self::Sendpays),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitDetails {
    #[prost(enumeration = "wait_details::WaitDetailsStatus", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub bolt11: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub bolt12: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "6")]
    pub partid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub groupid: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub payment_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "9")]
    pub in_channel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub in_htlc_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "11")]
    pub in_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "12")]
    pub out_channel: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WaitDetails`.
pub mod wait_details {
    /// Wait.details.status
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WaitDetailsStatus {
        Unpaid = 0,
        Paid = 1,
        Expired = 2,
        Pending = 3,
        Failed = 4,
        Complete = 5,
        Offered = 6,
        Settled = 7,
        LocalFailed = 8,
    }
    impl WaitDetailsStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WaitDetailsStatus::Unpaid => "UNPAID",
                WaitDetailsStatus::Paid => "PAID",
                WaitDetailsStatus::Expired => "EXPIRED",
                WaitDetailsStatus::Pending => "PENDING",
                WaitDetailsStatus::Failed => "FAILED",
                WaitDetailsStatus::Complete => "COMPLETE",
                WaitDetailsStatus::Offered => "OFFERED",
                WaitDetailsStatus::Settled => "SETTLED",
                WaitDetailsStatus::LocalFailed => "LOCAL_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNPAID" => Some(Self::Unpaid),
                "PAID" => Some(Self::Paid),
                "EXPIRED" => Some(Self::Expired),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "COMPLETE" => Some(Self::Complete),
                "OFFERED" => Some(Self::Offered),
                "SETTLED" => Some(Self::Settled),
                "LOCAL_FAILED" => Some(Self::LocalFailed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsRequest {
    #[prost(string, optional, tag = "1")]
    pub config: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsResponse {
    #[prost(message, optional, tag = "1")]
    pub configs: ::core::option::Option<ListconfigsConfigs>,
    #[prost(message, repeated, tag = "3")]
    pub plugins: ::prost::alloc::vec::Vec<ListconfigsPlugins>,
    #[prost(message, repeated, tag = "4")]
    pub important_plugins: ::prost::alloc::vec::Vec<ListconfigsImportantplugins>,
    #[prost(string, optional, tag = "5")]
    pub conf: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub lightning_dir: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub network: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub allow_deprecated_apis: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "9")]
    pub rpc_file: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "10")]
    pub disable_plugin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub bookkeeper_dir: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub bookkeeper_db: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "13")]
    pub always_use_proxy: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub daemon: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "15")]
    pub wallet: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16")]
    pub large_channels: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub experimental_dual_fund: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub experimental_splicing: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub experimental_onion_messages: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "20")]
    pub experimental_offers: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "21")]
    pub experimental_shutdown_wrong_funding: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "22")]
    pub experimental_peer_storage: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "23")]
    pub experimental_quiesce: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "24")]
    pub experimental_upgrade_protocol: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "25")]
    pub invoices_onchain_fallback: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "26")]
    pub database_upgrade: ::core::option::Option<bool>,
    #[prost(bytes = "vec", optional, tag = "27")]
    pub rgb: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "28")]
    pub alias: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "29")]
    pub pid_file: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "30")]
    pub ignore_fee_limits: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "31")]
    pub watchtime_blocks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub max_locktime_blocks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub funding_confirms: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "34")]
    pub cltv_delta: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "35")]
    pub cltv_final: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "36")]
    pub commit_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "37")]
    pub fee_base: ::core::option::Option<u32>,
    #[prost(sint64, optional, tag = "38")]
    pub rescan: ::core::option::Option<i64>,
    #[prost(uint32, optional, tag = "39")]
    pub fee_per_satoshi: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub max_concurrent_htlcs: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "41")]
    pub htlc_minimum_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "42")]
    pub htlc_maximum_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "43")]
    pub max_dust_htlc_exposure_msat: ::core::option::Option<Amount>,
    #[prost(uint64, optional, tag = "44")]
    pub min_capacity_sat: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "45")]
    pub addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "46")]
    pub announce_addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "47")]
    pub bind_addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "48")]
    pub offline: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "49")]
    pub autolisten: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "50")]
    pub proxy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "51")]
    pub disable_dns: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "52")]
    pub announce_addr_discovered: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(sint64, optional, tag = "53")]
    pub announce_addr_discovered_port: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "54")]
    pub encrypted_hsm: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "55")]
    pub rpc_file_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "56")]
    pub log_level: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "57")]
    pub log_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "58")]
    pub log_file: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "59")]
    pub log_timestamps: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "60")]
    pub force_feerates: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "61")]
    pub subdaemon: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "62")]
    pub fetchinvoice_noconnect: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "63")]
    pub accept_htlc_tlv_types: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "64")]
    pub tor_service_password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "65")]
    pub dev_allowdustreserve: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "66")]
    pub announce_addr_dns: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "67")]
    pub require_confirmed_inputs: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "68")]
    pub developer: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag = "69")]
    pub commit_fee: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "70")]
    pub min_emergency_msat: ::core::option::Option<Amount>,
    #[prost(uint32, optional, tag = "71")]
    pub commit_feerate_offset: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigs {
    #[prost(message, optional, tag = "1")]
    pub conf: ::core::option::Option<ListconfigsConfigsConf>,
    #[prost(message, optional, tag = "2")]
    pub developer: ::core::option::Option<ListconfigsConfigsDeveloper>,
    #[prost(message, optional, tag = "3")]
    pub clear_plugins: ::core::option::Option<ListconfigsConfigsClearplugins>,
    #[prost(message, optional, tag = "4")]
    pub disable_mpp: ::core::option::Option<ListconfigsConfigsDisablempp>,
    #[prost(message, optional, tag = "5")]
    pub mainnet: ::core::option::Option<ListconfigsConfigsMainnet>,
    #[prost(message, optional, tag = "6")]
    pub regtest: ::core::option::Option<ListconfigsConfigsRegtest>,
    #[prost(message, optional, tag = "7")]
    pub signet: ::core::option::Option<ListconfigsConfigsSignet>,
    #[prost(message, optional, tag = "8")]
    pub testnet: ::core::option::Option<ListconfigsConfigsTestnet>,
    #[prost(message, optional, tag = "9")]
    pub important_plugin: ::core::option::Option<ListconfigsConfigsImportantplugin>,
    #[prost(message, optional, tag = "10")]
    pub plugin: ::core::option::Option<ListconfigsConfigsPlugin>,
    #[prost(message, optional, tag = "11")]
    pub plugin_dir: ::core::option::Option<ListconfigsConfigsPlugindir>,
    #[prost(message, optional, tag = "12")]
    pub lightning_dir: ::core::option::Option<ListconfigsConfigsLightningdir>,
    #[prost(message, optional, tag = "13")]
    pub network: ::core::option::Option<ListconfigsConfigsNetwork>,
    #[prost(message, optional, tag = "14")]
    pub allow_deprecated_apis: ::core::option::Option<
        ListconfigsConfigsAllowdeprecatedapis,
    >,
    #[prost(message, optional, tag = "15")]
    pub rpc_file: ::core::option::Option<ListconfigsConfigsRpcfile>,
    #[prost(message, optional, tag = "16")]
    pub disable_plugin: ::core::option::Option<ListconfigsConfigsDisableplugin>,
    #[prost(message, optional, tag = "17")]
    pub always_use_proxy: ::core::option::Option<ListconfigsConfigsAlwaysuseproxy>,
    #[prost(message, optional, tag = "18")]
    pub daemon: ::core::option::Option<ListconfigsConfigsDaemon>,
    #[prost(message, optional, tag = "19")]
    pub wallet: ::core::option::Option<ListconfigsConfigsWallet>,
    #[prost(message, optional, tag = "20")]
    pub large_channels: ::core::option::Option<ListconfigsConfigsLargechannels>,
    #[prost(message, optional, tag = "21")]
    pub experimental_dual_fund: ::core::option::Option<
        ListconfigsConfigsExperimentaldualfund,
    >,
    #[prost(message, optional, tag = "22")]
    pub experimental_splicing: ::core::option::Option<
        ListconfigsConfigsExperimentalsplicing,
    >,
    #[prost(message, optional, tag = "23")]
    pub experimental_onion_messages: ::core::option::Option<
        ListconfigsConfigsExperimentalonionmessages,
    >,
    #[prost(message, optional, tag = "24")]
    pub experimental_offers: ::core::option::Option<
        ListconfigsConfigsExperimentaloffers,
    >,
    #[prost(message, optional, tag = "25")]
    pub experimental_shutdown_wrong_funding: ::core::option::Option<
        ListconfigsConfigsExperimentalshutdownwrongfunding,
    >,
    #[prost(message, optional, tag = "26")]
    pub experimental_peer_storage: ::core::option::Option<
        ListconfigsConfigsExperimentalpeerstorage,
    >,
    #[prost(message, optional, tag = "27")]
    pub experimental_anchors: ::core::option::Option<
        ListconfigsConfigsExperimentalanchors,
    >,
    #[prost(message, optional, tag = "28")]
    pub database_upgrade: ::core::option::Option<ListconfigsConfigsDatabaseupgrade>,
    #[prost(message, optional, tag = "29")]
    pub rgb: ::core::option::Option<ListconfigsConfigsRgb>,
    #[prost(message, optional, tag = "30")]
    pub alias: ::core::option::Option<ListconfigsConfigsAlias>,
    #[prost(message, optional, tag = "31")]
    pub pid_file: ::core::option::Option<ListconfigsConfigsPidfile>,
    #[prost(message, optional, tag = "32")]
    pub ignore_fee_limits: ::core::option::Option<ListconfigsConfigsIgnorefeelimits>,
    #[prost(message, optional, tag = "33")]
    pub watchtime_blocks: ::core::option::Option<ListconfigsConfigsWatchtimeblocks>,
    #[prost(message, optional, tag = "34")]
    pub max_locktime_blocks: ::core::option::Option<ListconfigsConfigsMaxlocktimeblocks>,
    #[prost(message, optional, tag = "35")]
    pub funding_confirms: ::core::option::Option<ListconfigsConfigsFundingconfirms>,
    #[prost(message, optional, tag = "36")]
    pub cltv_delta: ::core::option::Option<ListconfigsConfigsCltvdelta>,
    #[prost(message, optional, tag = "37")]
    pub cltv_final: ::core::option::Option<ListconfigsConfigsCltvfinal>,
    #[prost(message, optional, tag = "38")]
    pub commit_time: ::core::option::Option<ListconfigsConfigsCommittime>,
    #[prost(message, optional, tag = "39")]
    pub fee_base: ::core::option::Option<ListconfigsConfigsFeebase>,
    #[prost(message, optional, tag = "40")]
    pub rescan: ::core::option::Option<ListconfigsConfigsRescan>,
    #[prost(message, optional, tag = "41")]
    pub fee_per_satoshi: ::core::option::Option<ListconfigsConfigsFeepersatoshi>,
    #[prost(message, optional, tag = "42")]
    pub max_concurrent_htlcs: ::core::option::Option<
        ListconfigsConfigsMaxconcurrenthtlcs,
    >,
    #[prost(message, optional, tag = "43")]
    pub htlc_minimum_msat: ::core::option::Option<ListconfigsConfigsHtlcminimummsat>,
    #[prost(message, optional, tag = "44")]
    pub htlc_maximum_msat: ::core::option::Option<ListconfigsConfigsHtlcmaximummsat>,
    #[prost(message, optional, tag = "45")]
    pub max_dust_htlc_exposure_msat: ::core::option::Option<
        ListconfigsConfigsMaxdusthtlcexposuremsat,
    >,
    #[prost(message, optional, tag = "46")]
    pub min_capacity_sat: ::core::option::Option<ListconfigsConfigsMincapacitysat>,
    #[prost(message, optional, tag = "47")]
    pub addr: ::core::option::Option<ListconfigsConfigsAddr>,
    #[prost(message, optional, tag = "48")]
    pub announce_addr: ::core::option::Option<ListconfigsConfigsAnnounceaddr>,
    #[prost(message, optional, tag = "49")]
    pub bind_addr: ::core::option::Option<ListconfigsConfigsBindaddr>,
    #[prost(message, optional, tag = "50")]
    pub offline: ::core::option::Option<ListconfigsConfigsOffline>,
    #[prost(message, optional, tag = "51")]
    pub autolisten: ::core::option::Option<ListconfigsConfigsAutolisten>,
    #[prost(message, optional, tag = "52")]
    pub proxy: ::core::option::Option<ListconfigsConfigsProxy>,
    #[prost(message, optional, tag = "53")]
    pub disable_dns: ::core::option::Option<ListconfigsConfigsDisabledns>,
    #[prost(message, optional, tag = "54")]
    pub announce_addr_discovered: ::core::option::Option<
        ListconfigsConfigsAnnounceaddrdiscovered,
    >,
    #[prost(message, optional, tag = "55")]
    pub announce_addr_discovered_port: ::core::option::Option<
        ListconfigsConfigsAnnounceaddrdiscoveredport,
    >,
    #[prost(message, optional, tag = "56")]
    pub encrypted_hsm: ::core::option::Option<ListconfigsConfigsEncryptedhsm>,
    #[prost(message, optional, tag = "57")]
    pub rpc_file_mode: ::core::option::Option<ListconfigsConfigsRpcfilemode>,
    #[prost(message, optional, tag = "58")]
    pub log_level: ::core::option::Option<ListconfigsConfigsLoglevel>,
    #[prost(message, optional, tag = "59")]
    pub log_prefix: ::core::option::Option<ListconfigsConfigsLogprefix>,
    #[prost(message, optional, tag = "60")]
    pub log_file: ::core::option::Option<ListconfigsConfigsLogfile>,
    #[prost(message, optional, tag = "61")]
    pub log_timestamps: ::core::option::Option<ListconfigsConfigsLogtimestamps>,
    #[prost(message, optional, tag = "62")]
    pub force_feerates: ::core::option::Option<ListconfigsConfigsForcefeerates>,
    #[prost(message, optional, tag = "63")]
    pub subdaemon: ::core::option::Option<ListconfigsConfigsSubdaemon>,
    #[prost(message, optional, tag = "64")]
    pub fetchinvoice_noconnect: ::core::option::Option<
        ListconfigsConfigsFetchinvoicenoconnect,
    >,
    #[prost(message, optional, tag = "65")]
    pub accept_htlc_tlv_types: ::core::option::Option<
        ListconfigsConfigsAccepthtlctlvtypes,
    >,
    #[prost(message, optional, tag = "66")]
    pub tor_service_password: ::core::option::Option<
        ListconfigsConfigsTorservicepassword,
    >,
    #[prost(message, optional, tag = "67")]
    pub announce_addr_dns: ::core::option::Option<ListconfigsConfigsAnnounceaddrdns>,
    #[prost(message, optional, tag = "68")]
    pub require_confirmed_inputs: ::core::option::Option<
        ListconfigsConfigsRequireconfirmedinputs,
    >,
    #[prost(message, optional, tag = "69")]
    pub commit_fee: ::core::option::Option<ListconfigsConfigsCommitfee>,
    #[prost(message, optional, tag = "70")]
    pub commit_feerate_offset: ::core::option::Option<
        ListconfigsConfigsCommitfeerateoffset,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsConf {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(
        enumeration = "listconfigs_configs_conf::ListconfigsConfigsConfSource",
        tag = "2"
    )]
    pub source: i32,
}
/// Nested message and enum types in `ListconfigsConfigsConf`.
pub mod listconfigs_configs_conf {
    /// ListConfigs.configs.conf.source
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListconfigsConfigsConfSource {
        Cmdline = 0,
    }
    impl ListconfigsConfigsConfSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListconfigsConfigsConfSource::Cmdline => "CMDLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CMDLINE" => Some(Self::Cmdline),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDeveloper {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsClearplugins {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDisablempp {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub plugin: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsMainnet {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRegtest {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsSignet {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsTestnet {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsImportantplugin {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsPlugin {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsPlugindir {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLightningdir {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsNetwork {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAllowdeprecatedapis {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRpcfile {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDisableplugin {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAlwaysuseproxy {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDaemon {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsWallet {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLargechannels {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentaldualfund {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentalsplicing {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentalonionmessages {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentaloffers {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentalshutdownwrongfunding {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentalpeerstorage {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsExperimentalanchors {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDatabaseupgrade {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRgb {
    #[prost(bytes = "vec", tag = "1")]
    pub value_str: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAlias {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsPidfile {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsIgnorefeelimits {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsWatchtimeblocks {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsMaxlocktimeblocks {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsFundingconfirms {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsCltvdelta {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsCltvfinal {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsCommittime {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsFeebase {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRescan {
    #[prost(sint64, tag = "1")]
    pub value_int: i64,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsFeepersatoshi {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsMaxconcurrenthtlcs {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsHtlcminimummsat {
    #[prost(message, optional, tag = "1")]
    pub value_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsHtlcmaximummsat {
    #[prost(message, optional, tag = "1")]
    pub value_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsMaxdusthtlcexposuremsat {
    #[prost(message, optional, tag = "1")]
    pub value_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsMincapacitysat {
    #[prost(uint64, tag = "1")]
    pub value_int: u64,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "3")]
    pub dynamic: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAddr {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAnnounceaddr {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsBindaddr {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsOffline {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAutolisten {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsProxy {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsDisabledns {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAnnounceaddrdiscovered {
    #[prost(
        enumeration = "listconfigs_configs_announceaddrdiscovered::ListconfigsConfigsAnnounceaddrdiscoveredValueStr",
        tag = "1"
    )]
    pub value_str: i32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListconfigsConfigsAnnounceaddrdiscovered`.
pub mod listconfigs_configs_announceaddrdiscovered {
    /// ListConfigs.configs.announce-addr-discovered.value_str
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ListconfigsConfigsAnnounceaddrdiscoveredValueStr {
        True = 0,
        False = 1,
        Auto = 2,
    }
    impl ListconfigsConfigsAnnounceaddrdiscoveredValueStr {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListconfigsConfigsAnnounceaddrdiscoveredValueStr::True => "TRUE",
                ListconfigsConfigsAnnounceaddrdiscoveredValueStr::False => "FALSE",
                ListconfigsConfigsAnnounceaddrdiscoveredValueStr::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRUE" => Some(Self::True),
                "FALSE" => Some(Self::False),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAnnounceaddrdiscoveredport {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsEncryptedhsm {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRpcfilemode {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLoglevel {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLogprefix {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLogfile {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsLogtimestamps {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsForcefeerates {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsSubdaemon {
    #[prost(string, repeated, tag = "1")]
    pub values_str: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsFetchinvoicenoconnect {
    #[prost(bool, tag = "1")]
    pub set: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub plugin: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAccepthtlctlvtypes {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsTorservicepassword {
    #[prost(string, tag = "1")]
    pub value_str: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsAnnounceaddrdns {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsRequireconfirmedinputs {
    #[prost(bool, tag = "1")]
    pub value_bool: bool,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsCommitfee {
    #[prost(uint64, tag = "1")]
    pub value_int: u64,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsConfigsCommitfeerateoffset {
    #[prost(uint32, tag = "1")]
    pub value_int: u32,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsPlugins {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<ListconfigsPluginsOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsPluginsOptions {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsImportantplugins {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<ListconfigsImportantpluginsOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListconfigsImportantpluginsOptions {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(enumeration = "stop_response::StopResult", optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
}
/// Nested message and enum types in `StopResponse`.
pub mod stop_response {
    /// Stop.result
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum StopResult {
        ShutdownComplete = 0,
    }
    impl StopResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StopResult::ShutdownComplete => "SHUTDOWN_COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SHUTDOWN_COMPLETE" => Some(Self::ShutdownComplete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelpRequest {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelpResponse {
    #[prost(message, repeated, tag = "1")]
    pub help: ::prost::alloc::vec::Vec<HelpHelp>,
    #[prost(enumeration = "help_response::HelpFormathint", optional, tag = "2")]
    pub format_hint: ::core::option::Option<i32>,
}
/// Nested message and enum types in `HelpResponse`.
pub mod help_response {
    /// Help.format-hint
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum HelpFormathint {
        Simple = 0,
    }
    impl HelpFormathint {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HelpFormathint::Simple => "SIMPLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIMPLE" => Some(Self::Simple),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelpHelp {
    #[prost(string, tag = "1")]
    pub command: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreapprovekeysendRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub destination: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub amount_msat: ::core::option::Option<Amount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreapprovekeysendResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreapproveinvoiceRequest {
    #[prost(string, tag = "1")]
    pub bolt11: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreapproveinvoiceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticbackupRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticbackupResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub scb: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprchannelsapyRequest {
    #[prost(uint64, optional, tag = "1")]
    pub start_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub end_time: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprchannelsapyResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels_apy: ::prost::alloc::vec::Vec<BkprchannelsapyChannelsApy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprchannelsapyChannelsApy {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub routed_out_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub routed_in_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub lease_fee_paid_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub lease_fee_earned_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub pushed_out_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "7")]
    pub pushed_in_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub our_start_balance_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "9")]
    pub channel_start_balance_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "10")]
    pub fees_out_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "11")]
    pub fees_in_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "12")]
    pub utilization_out: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "13")]
    pub utilization_out_initial: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "14")]
    pub utilization_in: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "15")]
    pub utilization_in_initial: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "16")]
    pub apy_out: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "17")]
    pub apy_out_initial: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "18")]
    pub apy_in: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "19")]
    pub apy_in_initial: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "20")]
    pub apy_total: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "21")]
    pub apy_total_initial: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub apy_lease: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprdumpincomecsvRequest {
    #[prost(string, tag = "1")]
    pub csv_format: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub csv_file: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub consolidate_fees: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag = "4")]
    pub start_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub end_time: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprdumpincomecsvResponse {
    #[prost(string, tag = "1")]
    pub csv_file: ::prost::alloc::string::String,
    #[prost(
        enumeration = "bkprdumpincomecsv_response::BkprdumpincomecsvCsvFormat",
        tag = "2"
    )]
    pub csv_format: i32,
}
/// Nested message and enum types in `BkprdumpincomecsvResponse`.
pub mod bkprdumpincomecsv_response {
    /// Bkpr-DumpIncomeCsv.csv_format
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BkprdumpincomecsvCsvFormat {
        Cointracker = 0,
        Koinly = 1,
        Harmony = 2,
        Quickbooks = 3,
    }
    impl BkprdumpincomecsvCsvFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BkprdumpincomecsvCsvFormat::Cointracker => "COINTRACKER",
                BkprdumpincomecsvCsvFormat::Koinly => "KOINLY",
                BkprdumpincomecsvCsvFormat::Harmony => "HARMONY",
                BkprdumpincomecsvCsvFormat::Quickbooks => "QUICKBOOKS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COINTRACKER" => Some(Self::Cointracker),
                "KOINLY" => Some(Self::Koinly),
                "HARMONY" => Some(Self::Harmony),
                "QUICKBOOKS" => Some(Self::Quickbooks),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprinspectRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprinspectResponse {
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<BkprinspectTxs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprinspectTxs {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, optional, tag = "2")]
    pub blockheight: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "3")]
    pub fees_paid_msat: ::core::option::Option<Amount>,
    #[prost(message, repeated, tag = "4")]
    pub outputs: ::prost::alloc::vec::Vec<BkprinspectTxsOutputs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprinspectTxsOutputs {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub outnum: u32,
    #[prost(message, optional, tag = "3")]
    pub output_value_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "4")]
    pub currency: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub credit_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub debit_msat: ::core::option::Option<Amount>,
    #[prost(string, optional, tag = "7")]
    pub originating_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub output_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub spend_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub spending_txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub payment_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistaccounteventsRequest {
    #[prost(string, optional, tag = "1")]
    pub account: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistaccounteventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<BkprlistaccounteventsEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistaccounteventsEvents {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(
        enumeration = "bkprlistaccountevents_events::BkprlistaccounteventsEventsType",
        tag = "2"
    )]
    pub item_type: i32,
    #[prost(string, tag = "3")]
    pub tag: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub credit_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub debit_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub timestamp: u32,
    #[prost(string, optional, tag = "8")]
    pub outpoint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "9")]
    pub blockheight: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "10")]
    pub origin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub payment_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "13")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "14")]
    pub fees_msat: ::core::option::Option<Amount>,
    #[prost(bool, optional, tag = "15")]
    pub is_rebalance: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "16")]
    pub part_id: ::core::option::Option<u32>,
}
/// Nested message and enum types in `BkprlistaccounteventsEvents`.
pub mod bkprlistaccountevents_events {
    /// Bkpr-ListAccountEvents.events\[\].type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BkprlistaccounteventsEventsType {
        OnchainFee = 0,
        Chain = 1,
        Channel = 2,
    }
    impl BkprlistaccounteventsEventsType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BkprlistaccounteventsEventsType::OnchainFee => "ONCHAIN_FEE",
                BkprlistaccounteventsEventsType::Chain => "CHAIN",
                BkprlistaccounteventsEventsType::Channel => "CHANNEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ONCHAIN_FEE" => Some(Self::OnchainFee),
                "CHAIN" => Some(Self::Chain),
                "CHANNEL" => Some(Self::Channel),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistbalancesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistbalancesResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<BkprlistbalancesAccounts>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistbalancesAccounts {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<BkprlistbalancesAccountsBalances>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub peer_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "4")]
    pub we_opened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub account_closed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub account_resolved: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub resolved_at_block: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistbalancesAccountsBalances {
    #[prost(message, optional, tag = "1")]
    pub balance_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "2")]
    pub coin_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistincomeRequest {
    #[prost(bool, optional, tag = "1")]
    pub consolidate_fees: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "2")]
    pub start_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub end_time: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistincomeResponse {
    #[prost(message, repeated, tag = "1")]
    pub income_events: ::prost::alloc::vec::Vec<BkprlistincomeIncomeEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkprlistincomeIncomeEvents {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub credit_msat: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub debit_msat: ::core::option::Option<Amount>,
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub timestamp: u32,
    #[prost(string, optional, tag = "7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub outpoint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub txid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub payment_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlacklistruneRequest {
    #[prost(uint64, optional, tag = "1")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub end: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlacklistruneResponse {
    #[prost(message, repeated, tag = "1")]
    pub blacklist: ::prost::alloc::vec::Vec<BlacklistruneBlacklist>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlacklistruneBlacklist {
    #[prost(uint64, tag = "1")]
    pub start: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckruneRequest {
    #[prost(string, tag = "1")]
    pub rune: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub nodeid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub method: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckruneResponse {
    #[prost(bool, tag = "1")]
    pub valid: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateruneRequest {
    #[prost(string, optional, tag = "1")]
    pub rune: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub restrictions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateruneResponse {
    #[prost(string, tag = "1")]
    pub rune: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub warning_unrestricted_rune: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowrunesRequest {
    #[prost(string, optional, tag = "1")]
    pub rune: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowrunesResponse {
    #[prost(message, repeated, tag = "1")]
    pub runes: ::prost::alloc::vec::Vec<ShowrunesRunes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowrunesRunes {
    #[prost(string, tag = "1")]
    pub rune: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub restrictions: ::prost::alloc::vec::Vec<ShowrunesRunesRestrictions>,
    #[prost(string, tag = "4")]
    pub restrictions_as_english: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "5")]
    pub stored: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub blacklisted: ::core::option::Option<bool>,
    #[prost(double, optional, tag = "7")]
    pub last_used: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "8")]
    pub our_rune: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowrunesRunesRestrictions {
    #[prost(message, repeated, tag = "1")]
    pub alternatives: ::prost::alloc::vec::Vec<ShowrunesRunesRestrictionsAlternatives>,
    #[prost(string, tag = "2")]
    pub english: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowrunesRunesRestrictionsAlternatives {
    #[prost(string, tag = "1")]
    pub fieldname: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub condition: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub english: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBlockAddedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockAddedNotification {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamChannelOpenFailedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOpenFailedNotification {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamChannelOpenedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOpenedNotification {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub funding_msat: ::core::option::Option<Amount>,
    #[prost(bytes = "vec", tag = "3")]
    pub funding_txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub channel_ready: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamConnectRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerConnectNotification {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "peer_connect_notification::PeerConnectDirection", tag = "2")]
    pub direction: i32,
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<PeerConnectAddress>,
}
/// Nested message and enum types in `PeerConnectNotification`.
pub mod peer_connect_notification {
    /// connect.direction
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PeerConnectDirection {
        In = 0,
        Out = 1,
    }
    impl PeerConnectDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PeerConnectDirection::In => "IN",
                PeerConnectDirection::Out => "OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IN" => Some(Self::In),
                "OUT" => Some(Self::Out),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerConnectAddress {
    #[prost(enumeration = "peer_connect_address::PeerConnectAddressType", tag = "1")]
    pub item_type: i32,
    #[prost(string, optional, tag = "2")]
    pub socket: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub port: ::core::option::Option<u32>,
}
/// Nested message and enum types in `PeerConnectAddress`.
pub mod peer_connect_address {
    /// connect.address.type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PeerConnectAddressType {
        LocalSocket = 0,
        Ipv4 = 1,
        Ipv6 = 2,
        Torv2 = 3,
        Torv3 = 4,
    }
    impl PeerConnectAddressType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PeerConnectAddressType::LocalSocket => "LOCAL_SOCKET",
                PeerConnectAddressType::Ipv4 => "IPV4",
                PeerConnectAddressType::Ipv6 => "IPV6",
                PeerConnectAddressType::Torv2 => "TORV2",
                PeerConnectAddressType::Torv3 => "TORV3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCAL_SOCKET" => Some(Self::LocalSocket),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "TORV2" => Some(Self::Torv2),
                "TORV3" => Some(Self::Torv3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamCustomMsgRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMsgNotification {
    #[prost(bytes = "vec", tag = "1")]
    pub peer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod node_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NodeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NodeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NodeClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NodeClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NodeClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn getinfo(
            &mut self,
            request: impl tonic::IntoRequest<super::GetinfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetinfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Getinfo");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Getinfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_peers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListpeersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpeersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListPeers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListPeers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_funds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListfundsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListfundsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListFunds");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListFunds"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_pay(
            &mut self,
            request: impl tonic::IntoRequest<super::SendpayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendpayResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SendPay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SendPay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListchannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListChannels");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListChannels"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_gossip(
            &mut self,
            request: impl tonic::IntoRequest<super::AddgossipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddgossipResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/AddGossip");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "AddGossip"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_psbt_output(
            &mut self,
            request: impl tonic::IntoRequest<super::AddpsbtoutputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddpsbtoutputResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/AddPsbtOutput");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "AddPsbtOutput"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn auto_clean_once(
            &mut self,
            request: impl tonic::IntoRequest<super::AutocleanonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AutocleanonceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/AutoCleanOnce");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "AutoCleanOnce"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn auto_clean_status(
            &mut self,
            request: impl tonic::IntoRequest<super::AutocleanstatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AutocleanstatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/AutoCleanStatus");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "AutoCleanStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_message(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckmessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckmessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/CheckMessage");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "CheckMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseRequest>,
        ) -> std::result::Result<tonic::Response<super::CloseResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Close");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Close"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn connect_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ConnectPeer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ConnectPeer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/CreateInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "CreateInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn datastore(
            &mut self,
            request: impl tonic::IntoRequest<super::DatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatastoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Datastore");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Datastore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn datastore_usage(
            &mut self,
            request: impl tonic::IntoRequest<super::DatastoreusageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatastoreusageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DatastoreUsage");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DatastoreUsage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_onion(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateonionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateonionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/CreateOnion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "CreateOnion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn del_datastore(
            &mut self,
            request: impl tonic::IntoRequest<super::DeldatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeldatastoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DelDatastore");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DelDatastore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn del_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::DelinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DelinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DelInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DelInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dev_forget_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::DevforgetchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DevforgetchannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/DevForgetChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DevForgetChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn emergency_recover(
            &mut self,
            request: impl tonic::IntoRequest<super::EmergencyrecoverRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmergencyrecoverResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/EmergencyRecover",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "EmergencyRecover"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn recover(
            &mut self,
            request: impl tonic::IntoRequest<super::RecoverRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecoverResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Recover");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Recover"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn recover_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::RecoverchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecoverchannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/RecoverChannel");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "RecoverChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::InvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Invoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Invoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_invoice_request(
            &mut self,
            request: impl tonic::IntoRequest<super::InvoicerequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InvoicerequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/CreateInvoiceRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "CreateInvoiceRequest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disable_invoice_request(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableinvoicerequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisableinvoicerequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/DisableInvoiceRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "DisableInvoiceRequest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_invoice_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListinvoicerequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListinvoicerequestsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/ListInvoiceRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "ListInvoiceRequests"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_datastore(
            &mut self,
            request: impl tonic::IntoRequest<super::ListdatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListdatastoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListDatastore");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListDatastore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_invoices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListinvoicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListinvoicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListInvoices");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListInvoices"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_onion(
            &mut self,
            request: impl tonic::IntoRequest<super::SendonionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendonionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SendOnion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SendOnion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_send_pays(
            &mut self,
            request: impl tonic::IntoRequest<super::ListsendpaysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListsendpaysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListSendPays");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListSendPays"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListtransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListtransactionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/ListTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListTransactions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn make_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::MakesecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MakesecretResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/MakeSecret");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "MakeSecret"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pay(
            &mut self,
            request: impl tonic::IntoRequest<super::PayRequest>,
        ) -> std::result::Result<tonic::Response<super::PayResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Pay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Pay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListnodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListnodesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListNodes");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListNodes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait_any_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitanyinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitanyinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/WaitAnyInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "WaitAnyInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/WaitInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "WaitInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait_send_pay(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitsendpayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitsendpayResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/WaitSendPay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "WaitSendPay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::NewaddrRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewaddrResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/NewAddr");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "NewAddr"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WithdrawResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Withdraw");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Withdraw"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn key_send(
            &mut self,
            request: impl tonic::IntoRequest<super::KeysendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeysendResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/KeySend");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "KeySend"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fund_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::FundpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundpsbtResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/FundPsbt");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "FundPsbt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::SendpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendpsbtResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SendPsbt");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SendPsbt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::SignpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignpsbtResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SignPsbt");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SignPsbt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn utxo_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::UtxopsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UtxopsbtResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/UtxoPsbt");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "UtxoPsbt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_discard(
            &mut self,
            request: impl tonic::IntoRequest<super::TxdiscardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TxdiscardResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/TxDiscard");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "TxDiscard"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::TxprepareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TxprepareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/TxPrepare");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "TxPrepare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_send(
            &mut self,
            request: impl tonic::IntoRequest<super::TxsendRequest>,
        ) -> std::result::Result<tonic::Response<super::TxsendResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/TxSend");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "TxSend"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_peer_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListpeerchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpeerchannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/ListPeerChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListPeerChannels"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_closed_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListclosedchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListclosedchannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/ListClosedChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "ListClosedChannels"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn decode_pay(
            &mut self,
            request: impl tonic::IntoRequest<super::DecodepayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DecodepayResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DecodePay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DecodePay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn decode(
            &mut self,
            request: impl tonic::IntoRequest<super::DecodeRequest>,
        ) -> std::result::Result<tonic::Response<super::DecodeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Decode");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Decode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn del_pay(
            &mut self,
            request: impl tonic::IntoRequest<super::DelpayRequest>,
        ) -> std::result::Result<tonic::Response<super::DelpayResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DelPay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DelPay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn del_forward(
            &mut self,
            request: impl tonic::IntoRequest<super::DelforwardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DelforwardResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DelForward");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DelForward"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disable_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableofferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisableofferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/DisableOffer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "DisableOffer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisconnectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Disconnect");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Disconnect"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn feerates(
            &mut self,
            request: impl tonic::IntoRequest<super::FeeratesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FeeratesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Feerates");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Feerates"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fetch_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/FetchInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "FetchInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fund_channel_cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::FundchannelCancelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelCancelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/FundChannel_Cancel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "FundChannel_Cancel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fund_channel_complete(
            &mut self,
            request: impl tonic::IntoRequest<super::FundchannelCompleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelCompleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/FundChannel_Complete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "FundChannel_Complete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fund_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::FundchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/FundChannel");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "FundChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fund_channel_start(
            &mut self,
            request: impl tonic::IntoRequest<super::FundchannelStartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelStartResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/FundChannel_Start",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "FundChannel_Start"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_log(
            &mut self,
            request: impl tonic::IntoRequest<super::GetlogRequest>,
        ) -> std::result::Result<tonic::Response<super::GetlogResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/GetLog");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "GetLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn funder_update(
            &mut self,
            request: impl tonic::IntoRequest<super::FunderupdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FunderupdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/FunderUpdate");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "FunderUpdate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetrouteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetrouteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/GetRoute");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "GetRoute"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_forwards(
            &mut self,
            request: impl tonic::IntoRequest<super::ListforwardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListforwardsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListForwards");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListForwards"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListoffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListoffersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListOffers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListOffers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pays(
            &mut self,
            request: impl tonic::IntoRequest<super::ListpaysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpaysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListPays");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListPays"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_htlcs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListhtlcsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListhtlcsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListHtlcs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListHtlcs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn multi_fund_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::MultifundchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultifundchannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/MultiFundChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "MultiFundChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn multi_withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiwithdrawRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultiwithdrawResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/MultiWithdraw");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "MultiWithdraw"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn offer(
            &mut self,
            request: impl tonic::IntoRequest<super::OfferRequest>,
        ) -> std::result::Result<tonic::Response<super::OfferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Offer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Offer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel_abort(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenchannelAbortRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelAbortResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/OpenChannel_Abort",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "OpenChannel_Abort"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel_bump(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenchannelBumpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelBumpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/OpenChannel_Bump",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "OpenChannel_Bump"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel_init(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenchannelInitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelInitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/OpenChannel_Init",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "OpenChannel_Init"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel_signed(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenchannelSignedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelSignedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/OpenChannel_Signed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "OpenChannel_Signed"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel_update(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenchannelUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/OpenChannel_Update",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "OpenChannel_Update"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> std::result::Result<tonic::Response<super::PingResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Ping");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Ping"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginRequest>,
        ) -> std::result::Result<tonic::Response<super::PluginResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Plugin");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Plugin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn rene_pay_status(
            &mut self,
            request: impl tonic::IntoRequest<super::RenepaystatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RenepaystatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/RenePayStatus");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "RenePayStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn rene_pay(
            &mut self,
            request: impl tonic::IntoRequest<super::RenepayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RenepayResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/RenePay");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "RenePay"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reserve_inputs(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveinputsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReserveinputsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ReserveInputs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ReserveInputs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_custom_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SendcustommsgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendcustommsgResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SendCustomMsg");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SendCustomMsg"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SendinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SendInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SendInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::SetchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetchannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SetChannel");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SetChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetconfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetconfigResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SetConfig");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SetConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_psbt_version(
            &mut self,
            request: impl tonic::IntoRequest<super::SetpsbtversionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetpsbtversionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SetPsbtVersion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SetPsbtVersion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SigninvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SigninvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SignInvoice");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SignInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignmessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignmessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/SignMessage");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SignMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn splice_init(
            &mut self,
            request: impl tonic::IntoRequest<super::SpliceInitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceInitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Splice_Init");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Splice_Init"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn splice_signed(
            &mut self,
            request: impl tonic::IntoRequest<super::SpliceSignedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceSignedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Splice_Signed");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Splice_Signed"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn splice_update(
            &mut self,
            request: impl tonic::IntoRequest<super::SpliceUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Splice_Update");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Splice_Update"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unreserve_inputs(
            &mut self,
            request: impl tonic::IntoRequest<super::UnreserveinputsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnreserveinputsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/UnreserveInputs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "UnreserveInputs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn upgrade_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradewalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpgradewalletResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/UpgradeWallet");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "UpgradeWallet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait_block_height(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitblockheightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitblockheightResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/WaitBlockHeight");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "WaitBlockHeight"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitRequest>,
        ) -> std::result::Result<tonic::Response<super::WaitResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Wait");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Wait"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListconfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListconfigsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ListConfigs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ListConfigs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Stop");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Stop"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn help(
            &mut self,
            request: impl tonic::IntoRequest<super::HelpRequest>,
        ) -> std::result::Result<tonic::Response<super::HelpResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/Help");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "Help"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pre_approve_keysend(
            &mut self,
            request: impl tonic::IntoRequest<super::PreapprovekeysendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreapprovekeysendResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/PreApproveKeysend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "PreApproveKeysend"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pre_approve_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::PreapproveinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreapproveinvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/PreApproveInvoice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "PreApproveInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn static_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::StaticbackupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StaticbackupResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/StaticBackup");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "StaticBackup"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_channels_apy(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprchannelsapyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprchannelsapyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/BkprChannelsApy");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "BkprChannelsApy"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_dump_income_csv(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprdumpincomecsvRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprdumpincomecsvResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/BkprDumpIncomeCsv",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "BkprDumpIncomeCsv"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_inspect(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprinspectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprinspectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/BkprInspect");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "BkprInspect"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_list_account_events(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprlistaccounteventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistaccounteventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/BkprListAccountEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "BkprListAccountEvents"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_list_balances(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprlistbalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistbalancesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/BkprListBalances",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "BkprListBalances"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bkpr_list_income(
            &mut self,
            request: impl tonic::IntoRequest<super::BkprlistincomeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistincomeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/BkprListIncome");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "BkprListIncome"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklist_rune(
            &mut self,
            request: impl tonic::IntoRequest<super::BlacklistruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlacklistruneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/BlacklistRune");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "BlacklistRune"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_rune(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckruneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/CheckRune");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "CheckRune"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_rune(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateruneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/CreateRune");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "CreateRune"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn show_runes(
            &mut self,
            request: impl tonic::IntoRequest<super::ShowrunesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShowrunesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cln.Node/ShowRunes");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "ShowRunes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_block_added(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamBlockAddedRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BlockAddedNotification>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/SubscribeBlockAdded",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "SubscribeBlockAdded"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_channel_open_failed(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamChannelOpenFailedRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::ChannelOpenFailedNotification>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/SubscribeChannelOpenFailed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "SubscribeChannelOpenFailed"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_channel_opened(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamChannelOpenedRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ChannelOpenedNotification>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/SubscribeChannelOpened",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "SubscribeChannelOpened"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_connect(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamConnectRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PeerConnectNotification>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/SubscribeConnect",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cln.Node", "SubscribeConnect"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_custom_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamCustomMsgRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::CustomMsgNotification>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cln.Node/SubscribeCustomMsg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cln.Node", "SubscribeCustomMsg"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod node_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NodeServer.
    #[async_trait]
    pub trait Node: Send + Sync + 'static {
        async fn getinfo(
            &self,
            request: tonic::Request<super::GetinfoRequest>,
        ) -> std::result::Result<tonic::Response<super::GetinfoResponse>, tonic::Status>;
        async fn list_peers(
            &self,
            request: tonic::Request<super::ListpeersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpeersResponse>,
            tonic::Status,
        >;
        async fn list_funds(
            &self,
            request: tonic::Request<super::ListfundsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListfundsResponse>,
            tonic::Status,
        >;
        async fn send_pay(
            &self,
            request: tonic::Request<super::SendpayRequest>,
        ) -> std::result::Result<tonic::Response<super::SendpayResponse>, tonic::Status>;
        async fn list_channels(
            &self,
            request: tonic::Request<super::ListchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListchannelsResponse>,
            tonic::Status,
        >;
        async fn add_gossip(
            &self,
            request: tonic::Request<super::AddgossipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddgossipResponse>,
            tonic::Status,
        >;
        async fn add_psbt_output(
            &self,
            request: tonic::Request<super::AddpsbtoutputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddpsbtoutputResponse>,
            tonic::Status,
        >;
        async fn auto_clean_once(
            &self,
            request: tonic::Request<super::AutocleanonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AutocleanonceResponse>,
            tonic::Status,
        >;
        async fn auto_clean_status(
            &self,
            request: tonic::Request<super::AutocleanstatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AutocleanstatusResponse>,
            tonic::Status,
        >;
        async fn check_message(
            &self,
            request: tonic::Request<super::CheckmessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckmessageResponse>,
            tonic::Status,
        >;
        async fn close(
            &self,
            request: tonic::Request<super::CloseRequest>,
        ) -> std::result::Result<tonic::Response<super::CloseResponse>, tonic::Status>;
        async fn connect_peer(
            &self,
            request: tonic::Request<super::ConnectRequest>,
        ) -> std::result::Result<tonic::Response<super::ConnectResponse>, tonic::Status>;
        async fn create_invoice(
            &self,
            request: tonic::Request<super::CreateinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateinvoiceResponse>,
            tonic::Status,
        >;
        async fn datastore(
            &self,
            request: tonic::Request<super::DatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatastoreResponse>,
            tonic::Status,
        >;
        async fn datastore_usage(
            &self,
            request: tonic::Request<super::DatastoreusageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatastoreusageResponse>,
            tonic::Status,
        >;
        async fn create_onion(
            &self,
            request: tonic::Request<super::CreateonionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateonionResponse>,
            tonic::Status,
        >;
        async fn del_datastore(
            &self,
            request: tonic::Request<super::DeldatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeldatastoreResponse>,
            tonic::Status,
        >;
        async fn del_invoice(
            &self,
            request: tonic::Request<super::DelinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DelinvoiceResponse>,
            tonic::Status,
        >;
        async fn dev_forget_channel(
            &self,
            request: tonic::Request<super::DevforgetchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DevforgetchannelResponse>,
            tonic::Status,
        >;
        async fn emergency_recover(
            &self,
            request: tonic::Request<super::EmergencyrecoverRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmergencyrecoverResponse>,
            tonic::Status,
        >;
        async fn recover(
            &self,
            request: tonic::Request<super::RecoverRequest>,
        ) -> std::result::Result<tonic::Response<super::RecoverResponse>, tonic::Status>;
        async fn recover_channel(
            &self,
            request: tonic::Request<super::RecoverchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecoverchannelResponse>,
            tonic::Status,
        >;
        async fn invoice(
            &self,
            request: tonic::Request<super::InvoiceRequest>,
        ) -> std::result::Result<tonic::Response<super::InvoiceResponse>, tonic::Status>;
        async fn create_invoice_request(
            &self,
            request: tonic::Request<super::InvoicerequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InvoicerequestResponse>,
            tonic::Status,
        >;
        async fn disable_invoice_request(
            &self,
            request: tonic::Request<super::DisableinvoicerequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisableinvoicerequestResponse>,
            tonic::Status,
        >;
        async fn list_invoice_requests(
            &self,
            request: tonic::Request<super::ListinvoicerequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListinvoicerequestsResponse>,
            tonic::Status,
        >;
        async fn list_datastore(
            &self,
            request: tonic::Request<super::ListdatastoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListdatastoreResponse>,
            tonic::Status,
        >;
        async fn list_invoices(
            &self,
            request: tonic::Request<super::ListinvoicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListinvoicesResponse>,
            tonic::Status,
        >;
        async fn send_onion(
            &self,
            request: tonic::Request<super::SendonionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendonionResponse>,
            tonic::Status,
        >;
        async fn list_send_pays(
            &self,
            request: tonic::Request<super::ListsendpaysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListsendpaysResponse>,
            tonic::Status,
        >;
        async fn list_transactions(
            &self,
            request: tonic::Request<super::ListtransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListtransactionsResponse>,
            tonic::Status,
        >;
        async fn make_secret(
            &self,
            request: tonic::Request<super::MakesecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MakesecretResponse>,
            tonic::Status,
        >;
        async fn pay(
            &self,
            request: tonic::Request<super::PayRequest>,
        ) -> std::result::Result<tonic::Response<super::PayResponse>, tonic::Status>;
        async fn list_nodes(
            &self,
            request: tonic::Request<super::ListnodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListnodesResponse>,
            tonic::Status,
        >;
        async fn wait_any_invoice(
            &self,
            request: tonic::Request<super::WaitanyinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitanyinvoiceResponse>,
            tonic::Status,
        >;
        async fn wait_invoice(
            &self,
            request: tonic::Request<super::WaitinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitinvoiceResponse>,
            tonic::Status,
        >;
        async fn wait_send_pay(
            &self,
            request: tonic::Request<super::WaitsendpayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitsendpayResponse>,
            tonic::Status,
        >;
        async fn new_addr(
            &self,
            request: tonic::Request<super::NewaddrRequest>,
        ) -> std::result::Result<tonic::Response<super::NewaddrResponse>, tonic::Status>;
        async fn withdraw(
            &self,
            request: tonic::Request<super::WithdrawRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WithdrawResponse>,
            tonic::Status,
        >;
        async fn key_send(
            &self,
            request: tonic::Request<super::KeysendRequest>,
        ) -> std::result::Result<tonic::Response<super::KeysendResponse>, tonic::Status>;
        async fn fund_psbt(
            &self,
            request: tonic::Request<super::FundpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundpsbtResponse>,
            tonic::Status,
        >;
        async fn send_psbt(
            &self,
            request: tonic::Request<super::SendpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendpsbtResponse>,
            tonic::Status,
        >;
        async fn sign_psbt(
            &self,
            request: tonic::Request<super::SignpsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignpsbtResponse>,
            tonic::Status,
        >;
        async fn utxo_psbt(
            &self,
            request: tonic::Request<super::UtxopsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UtxopsbtResponse>,
            tonic::Status,
        >;
        async fn tx_discard(
            &self,
            request: tonic::Request<super::TxdiscardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TxdiscardResponse>,
            tonic::Status,
        >;
        async fn tx_prepare(
            &self,
            request: tonic::Request<super::TxprepareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TxprepareResponse>,
            tonic::Status,
        >;
        async fn tx_send(
            &self,
            request: tonic::Request<super::TxsendRequest>,
        ) -> std::result::Result<tonic::Response<super::TxsendResponse>, tonic::Status>;
        async fn list_peer_channels(
            &self,
            request: tonic::Request<super::ListpeerchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpeerchannelsResponse>,
            tonic::Status,
        >;
        async fn list_closed_channels(
            &self,
            request: tonic::Request<super::ListclosedchannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListclosedchannelsResponse>,
            tonic::Status,
        >;
        async fn decode_pay(
            &self,
            request: tonic::Request<super::DecodepayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DecodepayResponse>,
            tonic::Status,
        >;
        async fn decode(
            &self,
            request: tonic::Request<super::DecodeRequest>,
        ) -> std::result::Result<tonic::Response<super::DecodeResponse>, tonic::Status>;
        async fn del_pay(
            &self,
            request: tonic::Request<super::DelpayRequest>,
        ) -> std::result::Result<tonic::Response<super::DelpayResponse>, tonic::Status>;
        async fn del_forward(
            &self,
            request: tonic::Request<super::DelforwardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DelforwardResponse>,
            tonic::Status,
        >;
        async fn disable_offer(
            &self,
            request: tonic::Request<super::DisableofferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisableofferResponse>,
            tonic::Status,
        >;
        async fn disconnect(
            &self,
            request: tonic::Request<super::DisconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisconnectResponse>,
            tonic::Status,
        >;
        async fn feerates(
            &self,
            request: tonic::Request<super::FeeratesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FeeratesResponse>,
            tonic::Status,
        >;
        async fn fetch_invoice(
            &self,
            request: tonic::Request<super::FetchinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchinvoiceResponse>,
            tonic::Status,
        >;
        async fn fund_channel_cancel(
            &self,
            request: tonic::Request<super::FundchannelCancelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelCancelResponse>,
            tonic::Status,
        >;
        async fn fund_channel_complete(
            &self,
            request: tonic::Request<super::FundchannelCompleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelCompleteResponse>,
            tonic::Status,
        >;
        async fn fund_channel(
            &self,
            request: tonic::Request<super::FundchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelResponse>,
            tonic::Status,
        >;
        async fn fund_channel_start(
            &self,
            request: tonic::Request<super::FundchannelStartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FundchannelStartResponse>,
            tonic::Status,
        >;
        async fn get_log(
            &self,
            request: tonic::Request<super::GetlogRequest>,
        ) -> std::result::Result<tonic::Response<super::GetlogResponse>, tonic::Status>;
        async fn funder_update(
            &self,
            request: tonic::Request<super::FunderupdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FunderupdateResponse>,
            tonic::Status,
        >;
        async fn get_route(
            &self,
            request: tonic::Request<super::GetrouteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetrouteResponse>,
            tonic::Status,
        >;
        async fn list_forwards(
            &self,
            request: tonic::Request<super::ListforwardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListforwardsResponse>,
            tonic::Status,
        >;
        async fn list_offers(
            &self,
            request: tonic::Request<super::ListoffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListoffersResponse>,
            tonic::Status,
        >;
        async fn list_pays(
            &self,
            request: tonic::Request<super::ListpaysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListpaysResponse>,
            tonic::Status,
        >;
        async fn list_htlcs(
            &self,
            request: tonic::Request<super::ListhtlcsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListhtlcsResponse>,
            tonic::Status,
        >;
        async fn multi_fund_channel(
            &self,
            request: tonic::Request<super::MultifundchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultifundchannelResponse>,
            tonic::Status,
        >;
        async fn multi_withdraw(
            &self,
            request: tonic::Request<super::MultiwithdrawRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultiwithdrawResponse>,
            tonic::Status,
        >;
        async fn offer(
            &self,
            request: tonic::Request<super::OfferRequest>,
        ) -> std::result::Result<tonic::Response<super::OfferResponse>, tonic::Status>;
        async fn open_channel_abort(
            &self,
            request: tonic::Request<super::OpenchannelAbortRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelAbortResponse>,
            tonic::Status,
        >;
        async fn open_channel_bump(
            &self,
            request: tonic::Request<super::OpenchannelBumpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelBumpResponse>,
            tonic::Status,
        >;
        async fn open_channel_init(
            &self,
            request: tonic::Request<super::OpenchannelInitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelInitResponse>,
            tonic::Status,
        >;
        async fn open_channel_signed(
            &self,
            request: tonic::Request<super::OpenchannelSignedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelSignedResponse>,
            tonic::Status,
        >;
        async fn open_channel_update(
            &self,
            request: tonic::Request<super::OpenchannelUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenchannelUpdateResponse>,
            tonic::Status,
        >;
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> std::result::Result<tonic::Response<super::PingResponse>, tonic::Status>;
        async fn plugin(
            &self,
            request: tonic::Request<super::PluginRequest>,
        ) -> std::result::Result<tonic::Response<super::PluginResponse>, tonic::Status>;
        async fn rene_pay_status(
            &self,
            request: tonic::Request<super::RenepaystatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RenepaystatusResponse>,
            tonic::Status,
        >;
        async fn rene_pay(
            &self,
            request: tonic::Request<super::RenepayRequest>,
        ) -> std::result::Result<tonic::Response<super::RenepayResponse>, tonic::Status>;
        async fn reserve_inputs(
            &self,
            request: tonic::Request<super::ReserveinputsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReserveinputsResponse>,
            tonic::Status,
        >;
        async fn send_custom_msg(
            &self,
            request: tonic::Request<super::SendcustommsgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendcustommsgResponse>,
            tonic::Status,
        >;
        async fn send_invoice(
            &self,
            request: tonic::Request<super::SendinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendinvoiceResponse>,
            tonic::Status,
        >;
        async fn set_channel(
            &self,
            request: tonic::Request<super::SetchannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetchannelResponse>,
            tonic::Status,
        >;
        async fn set_config(
            &self,
            request: tonic::Request<super::SetconfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetconfigResponse>,
            tonic::Status,
        >;
        async fn set_psbt_version(
            &self,
            request: tonic::Request<super::SetpsbtversionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetpsbtversionResponse>,
            tonic::Status,
        >;
        async fn sign_invoice(
            &self,
            request: tonic::Request<super::SigninvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SigninvoiceResponse>,
            tonic::Status,
        >;
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignmessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignmessageResponse>,
            tonic::Status,
        >;
        async fn splice_init(
            &self,
            request: tonic::Request<super::SpliceInitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceInitResponse>,
            tonic::Status,
        >;
        async fn splice_signed(
            &self,
            request: tonic::Request<super::SpliceSignedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceSignedResponse>,
            tonic::Status,
        >;
        async fn splice_update(
            &self,
            request: tonic::Request<super::SpliceUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpliceUpdateResponse>,
            tonic::Status,
        >;
        async fn unreserve_inputs(
            &self,
            request: tonic::Request<super::UnreserveinputsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnreserveinputsResponse>,
            tonic::Status,
        >;
        async fn upgrade_wallet(
            &self,
            request: tonic::Request<super::UpgradewalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpgradewalletResponse>,
            tonic::Status,
        >;
        async fn wait_block_height(
            &self,
            request: tonic::Request<super::WaitblockheightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitblockheightResponse>,
            tonic::Status,
        >;
        async fn wait(
            &self,
            request: tonic::Request<super::WaitRequest>,
        ) -> std::result::Result<tonic::Response<super::WaitResponse>, tonic::Status>;
        async fn list_configs(
            &self,
            request: tonic::Request<super::ListconfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListconfigsResponse>,
            tonic::Status,
        >;
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status>;
        async fn help(
            &self,
            request: tonic::Request<super::HelpRequest>,
        ) -> std::result::Result<tonic::Response<super::HelpResponse>, tonic::Status>;
        async fn pre_approve_keysend(
            &self,
            request: tonic::Request<super::PreapprovekeysendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreapprovekeysendResponse>,
            tonic::Status,
        >;
        async fn pre_approve_invoice(
            &self,
            request: tonic::Request<super::PreapproveinvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreapproveinvoiceResponse>,
            tonic::Status,
        >;
        async fn static_backup(
            &self,
            request: tonic::Request<super::StaticbackupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StaticbackupResponse>,
            tonic::Status,
        >;
        async fn bkpr_channels_apy(
            &self,
            request: tonic::Request<super::BkprchannelsapyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprchannelsapyResponse>,
            tonic::Status,
        >;
        async fn bkpr_dump_income_csv(
            &self,
            request: tonic::Request<super::BkprdumpincomecsvRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprdumpincomecsvResponse>,
            tonic::Status,
        >;
        async fn bkpr_inspect(
            &self,
            request: tonic::Request<super::BkprinspectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprinspectResponse>,
            tonic::Status,
        >;
        async fn bkpr_list_account_events(
            &self,
            request: tonic::Request<super::BkprlistaccounteventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistaccounteventsResponse>,
            tonic::Status,
        >;
        async fn bkpr_list_balances(
            &self,
            request: tonic::Request<super::BkprlistbalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistbalancesResponse>,
            tonic::Status,
        >;
        async fn bkpr_list_income(
            &self,
            request: tonic::Request<super::BkprlistincomeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BkprlistincomeResponse>,
            tonic::Status,
        >;
        async fn blacklist_rune(
            &self,
            request: tonic::Request<super::BlacklistruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlacklistruneResponse>,
            tonic::Status,
        >;
        async fn check_rune(
            &self,
            request: tonic::Request<super::CheckruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckruneResponse>,
            tonic::Status,
        >;
        async fn create_rune(
            &self,
            request: tonic::Request<super::CreateruneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateruneResponse>,
            tonic::Status,
        >;
        async fn show_runes(
            &self,
            request: tonic::Request<super::ShowrunesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShowrunesResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeBlockAdded method.
        type SubscribeBlockAddedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BlockAddedNotification, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_block_added(
            &self,
            request: tonic::Request<super::StreamBlockAddedRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeBlockAddedStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeChannelOpenFailed method.
        type SubscribeChannelOpenFailedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ChannelOpenFailedNotification,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn subscribe_channel_open_failed(
            &self,
            request: tonic::Request<super::StreamChannelOpenFailedRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeChannelOpenFailedStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeChannelOpened method.
        type SubscribeChannelOpenedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ChannelOpenedNotification,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn subscribe_channel_opened(
            &self,
            request: tonic::Request<super::StreamChannelOpenedRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeChannelOpenedStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeConnect method.
        type SubscribeConnectStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PeerConnectNotification, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_connect(
            &self,
            request: tonic::Request<super::StreamConnectRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeConnectStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeCustomMsg method.
        type SubscribeCustomMsgStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::CustomMsgNotification, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_custom_msg(
            &self,
            request: tonic::Request<super::StreamCustomMsgRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeCustomMsgStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct NodeServer<T: Node> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Node> NodeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NodeServer<T>
    where
        T: Node,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cln.Node/Getinfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetinfoSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::GetinfoRequest>
                    for GetinfoSvc<T> {
                        type Response = super::GetinfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetinfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::getinfo(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetinfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListPeers" => {
                    #[allow(non_camel_case_types)]
                    struct ListPeersSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListpeersRequest>
                    for ListPeersSvc<T> {
                        type Response = super::ListpeersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListpeersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_peers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPeersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListFunds" => {
                    #[allow(non_camel_case_types)]
                    struct ListFundsSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListfundsRequest>
                    for ListFundsSvc<T> {
                        type Response = super::ListfundsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListfundsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_funds(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFundsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SendPay" => {
                    #[allow(non_camel_case_types)]
                    struct SendPaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SendpayRequest>
                    for SendPaySvc<T> {
                        type Response = super::SendpayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendpayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::send_pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListChannelsSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListchannelsRequest>
                    for ListChannelsSvc<T> {
                        type Response = super::ListchannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListchannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/AddGossip" => {
                    #[allow(non_camel_case_types)]
                    struct AddGossipSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::AddgossipRequest>
                    for AddGossipSvc<T> {
                        type Response = super::AddgossipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddgossipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::add_gossip(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddGossipSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/AddPsbtOutput" => {
                    #[allow(non_camel_case_types)]
                    struct AddPsbtOutputSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::AddpsbtoutputRequest>
                    for AddPsbtOutputSvc<T> {
                        type Response = super::AddpsbtoutputResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddpsbtoutputRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::add_psbt_output(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPsbtOutputSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/AutoCleanOnce" => {
                    #[allow(non_camel_case_types)]
                    struct AutoCleanOnceSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::AutocleanonceRequest>
                    for AutoCleanOnceSvc<T> {
                        type Response = super::AutocleanonceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AutocleanonceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::auto_clean_once(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AutoCleanOnceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/AutoCleanStatus" => {
                    #[allow(non_camel_case_types)]
                    struct AutoCleanStatusSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::AutocleanstatusRequest>
                    for AutoCleanStatusSvc<T> {
                        type Response = super::AutocleanstatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AutocleanstatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::auto_clean_status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AutoCleanStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CheckMessage" => {
                    #[allow(non_camel_case_types)]
                    struct CheckMessageSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::CheckmessageRequest>
                    for CheckMessageSvc<T> {
                        type Response = super::CheckmessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckmessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::check_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::CloseRequest>
                    for CloseSvc<T> {
                        type Response = super::CloseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::close(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ConnectPeer" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectPeerSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ConnectRequest>
                    for ConnectPeerSvc<T> {
                        type Response = super::ConnectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::connect_peer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CreateInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::CreateinvoiceRequest>
                    for CreateInvoiceSvc<T> {
                        type Response = super::CreateinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::create_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Datastore" => {
                    #[allow(non_camel_case_types)]
                    struct DatastoreSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DatastoreRequest>
                    for DatastoreSvc<T> {
                        type Response = super::DatastoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatastoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::datastore(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DatastoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DatastoreUsage" => {
                    #[allow(non_camel_case_types)]
                    struct DatastoreUsageSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::DatastoreusageRequest>
                    for DatastoreUsageSvc<T> {
                        type Response = super::DatastoreusageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatastoreusageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::datastore_usage(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DatastoreUsageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CreateOnion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOnionSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::CreateonionRequest>
                    for CreateOnionSvc<T> {
                        type Response = super::CreateonionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateonionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::create_onion(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOnionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DelDatastore" => {
                    #[allow(non_camel_case_types)]
                    struct DelDatastoreSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DeldatastoreRequest>
                    for DelDatastoreSvc<T> {
                        type Response = super::DeldatastoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeldatastoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::del_datastore(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelDatastoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DelInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct DelInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DelinvoiceRequest>
                    for DelInvoiceSvc<T> {
                        type Response = super::DelinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::del_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DevForgetChannel" => {
                    #[allow(non_camel_case_types)]
                    struct DevForgetChannelSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::DevforgetchannelRequest>
                    for DevForgetChannelSvc<T> {
                        type Response = super::DevforgetchannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DevforgetchannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::dev_forget_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DevForgetChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/EmergencyRecover" => {
                    #[allow(non_camel_case_types)]
                    struct EmergencyRecoverSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::EmergencyrecoverRequest>
                    for EmergencyRecoverSvc<T> {
                        type Response = super::EmergencyrecoverResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmergencyrecoverRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::emergency_recover(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EmergencyRecoverSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Recover" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::RecoverRequest>
                    for RecoverSvc<T> {
                        type Response = super::RecoverResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecoverRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::recover(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecoverSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/RecoverChannel" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverChannelSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::RecoverchannelRequest>
                    for RecoverChannelSvc<T> {
                        type Response = super::RecoverchannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecoverchannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::recover_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecoverChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Invoice" => {
                    #[allow(non_camel_case_types)]
                    struct InvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::InvoiceRequest>
                    for InvoiceSvc<T> {
                        type Response = super::InvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CreateInvoiceRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInvoiceRequestSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::InvoicerequestRequest>
                    for CreateInvoiceRequestSvc<T> {
                        type Response = super::InvoicerequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InvoicerequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::create_invoice_request(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateInvoiceRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DisableInvoiceRequest" => {
                    #[allow(non_camel_case_types)]
                    struct DisableInvoiceRequestSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::DisableinvoicerequestRequest>
                    for DisableInvoiceRequestSvc<T> {
                        type Response = super::DisableinvoicerequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisableinvoicerequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::disable_invoice_request(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisableInvoiceRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListInvoiceRequests" => {
                    #[allow(non_camel_case_types)]
                    struct ListInvoiceRequestsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ListinvoicerequestsRequest>
                    for ListInvoiceRequestsSvc<T> {
                        type Response = super::ListinvoicerequestsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListinvoicerequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_invoice_requests(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListInvoiceRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListDatastore" => {
                    #[allow(non_camel_case_types)]
                    struct ListDatastoreSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ListdatastoreRequest>
                    for ListDatastoreSvc<T> {
                        type Response = super::ListdatastoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListdatastoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_datastore(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDatastoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListInvoices" => {
                    #[allow(non_camel_case_types)]
                    struct ListInvoicesSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListinvoicesRequest>
                    for ListInvoicesSvc<T> {
                        type Response = super::ListinvoicesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListinvoicesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_invoices(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListInvoicesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SendOnion" => {
                    #[allow(non_camel_case_types)]
                    struct SendOnionSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SendonionRequest>
                    for SendOnionSvc<T> {
                        type Response = super::SendonionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendonionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::send_onion(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendOnionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListSendPays" => {
                    #[allow(non_camel_case_types)]
                    struct ListSendPaysSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListsendpaysRequest>
                    for ListSendPaysSvc<T> {
                        type Response = super::ListsendpaysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListsendpaysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_send_pays(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSendPaysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListTransactionsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ListtransactionsRequest>
                    for ListTransactionsSvc<T> {
                        type Response = super::ListtransactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListtransactionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_transactions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTransactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/MakeSecret" => {
                    #[allow(non_camel_case_types)]
                    struct MakeSecretSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::MakesecretRequest>
                    for MakeSecretSvc<T> {
                        type Response = super::MakesecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MakesecretRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::make_secret(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MakeSecretSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Pay" => {
                    #[allow(non_camel_case_types)]
                    struct PaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::PayRequest>
                    for PaySvc<T> {
                        type Response = super::PayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListNodes" => {
                    #[allow(non_camel_case_types)]
                    struct ListNodesSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListnodesRequest>
                    for ListNodesSvc<T> {
                        type Response = super::ListnodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListnodesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_nodes(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListNodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/WaitAnyInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct WaitAnyInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::WaitanyinvoiceRequest>
                    for WaitAnyInvoiceSvc<T> {
                        type Response = super::WaitanyinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitanyinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::wait_any_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitAnyInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/WaitInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct WaitInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::WaitinvoiceRequest>
                    for WaitInvoiceSvc<T> {
                        type Response = super::WaitinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::wait_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/WaitSendPay" => {
                    #[allow(non_camel_case_types)]
                    struct WaitSendPaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::WaitsendpayRequest>
                    for WaitSendPaySvc<T> {
                        type Response = super::WaitsendpayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitsendpayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::wait_send_pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitSendPaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/NewAddr" => {
                    #[allow(non_camel_case_types)]
                    struct NewAddrSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::NewaddrRequest>
                    for NewAddrSvc<T> {
                        type Response = super::NewaddrResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewaddrRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::new_addr(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAddrSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Withdraw" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::WithdrawRequest>
                    for WithdrawSvc<T> {
                        type Response = super::WithdrawResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WithdrawRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::withdraw(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/KeySend" => {
                    #[allow(non_camel_case_types)]
                    struct KeySendSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::KeysendRequest>
                    for KeySendSvc<T> {
                        type Response = super::KeysendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeysendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::key_send(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeySendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FundPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct FundPsbtSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::FundpsbtRequest>
                    for FundPsbtSvc<T> {
                        type Response = super::FundpsbtResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundpsbtRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fund_psbt(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundPsbtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SendPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct SendPsbtSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SendpsbtRequest>
                    for SendPsbtSvc<T> {
                        type Response = super::SendpsbtResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendpsbtRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::send_psbt(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPsbtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SignPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct SignPsbtSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SignpsbtRequest>
                    for SignPsbtSvc<T> {
                        type Response = super::SignpsbtResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignpsbtRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::sign_psbt(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignPsbtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/UtxoPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct UtxoPsbtSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::UtxopsbtRequest>
                    for UtxoPsbtSvc<T> {
                        type Response = super::UtxopsbtResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UtxopsbtRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::utxo_psbt(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UtxoPsbtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/TxDiscard" => {
                    #[allow(non_camel_case_types)]
                    struct TxDiscardSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::TxdiscardRequest>
                    for TxDiscardSvc<T> {
                        type Response = super::TxdiscardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxdiscardRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::tx_discard(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxDiscardSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/TxPrepare" => {
                    #[allow(non_camel_case_types)]
                    struct TxPrepareSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::TxprepareRequest>
                    for TxPrepareSvc<T> {
                        type Response = super::TxprepareResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxprepareRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::tx_prepare(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxPrepareSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/TxSend" => {
                    #[allow(non_camel_case_types)]
                    struct TxSendSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::TxsendRequest>
                    for TxSendSvc<T> {
                        type Response = super::TxsendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxsendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::tx_send(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxSendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListPeerChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListPeerChannelsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ListpeerchannelsRequest>
                    for ListPeerChannelsSvc<T> {
                        type Response = super::ListpeerchannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListpeerchannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_peer_channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPeerChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListClosedChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListClosedChannelsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ListclosedchannelsRequest>
                    for ListClosedChannelsSvc<T> {
                        type Response = super::ListclosedchannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListclosedchannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_closed_channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListClosedChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DecodePay" => {
                    #[allow(non_camel_case_types)]
                    struct DecodePaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DecodepayRequest>
                    for DecodePaySvc<T> {
                        type Response = super::DecodepayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DecodepayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::decode_pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DecodePaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Decode" => {
                    #[allow(non_camel_case_types)]
                    struct DecodeSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DecodeRequest>
                    for DecodeSvc<T> {
                        type Response = super::DecodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DecodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::decode(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DecodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DelPay" => {
                    #[allow(non_camel_case_types)]
                    struct DelPaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DelpayRequest>
                    for DelPaySvc<T> {
                        type Response = super::DelpayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelpayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::del_pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelPaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DelForward" => {
                    #[allow(non_camel_case_types)]
                    struct DelForwardSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DelforwardRequest>
                    for DelForwardSvc<T> {
                        type Response = super::DelforwardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelforwardRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::del_forward(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelForwardSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/DisableOffer" => {
                    #[allow(non_camel_case_types)]
                    struct DisableOfferSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DisableofferRequest>
                    for DisableOfferSvc<T> {
                        type Response = super::DisableofferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisableofferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::disable_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisableOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Disconnect" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::DisconnectRequest>
                    for DisconnectSvc<T> {
                        type Response = super::DisconnectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisconnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::disconnect(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisconnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Feerates" => {
                    #[allow(non_camel_case_types)]
                    struct FeeratesSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::FeeratesRequest>
                    for FeeratesSvc<T> {
                        type Response = super::FeeratesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FeeratesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::feerates(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FeeratesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FetchInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct FetchInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::FetchinvoiceRequest>
                    for FetchInvoiceSvc<T> {
                        type Response = super::FetchinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FetchinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fetch_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FundChannel_Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct FundChannel_CancelSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::FundchannelCancelRequest>
                    for FundChannel_CancelSvc<T> {
                        type Response = super::FundchannelCancelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundchannelCancelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fund_channel_cancel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundChannel_CancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FundChannel_Complete" => {
                    #[allow(non_camel_case_types)]
                    struct FundChannel_CompleteSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::FundchannelCompleteRequest>
                    for FundChannel_CompleteSvc<T> {
                        type Response = super::FundchannelCompleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundchannelCompleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fund_channel_complete(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundChannel_CompleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FundChannel" => {
                    #[allow(non_camel_case_types)]
                    struct FundChannelSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::FundchannelRequest>
                    for FundChannelSvc<T> {
                        type Response = super::FundchannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundchannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fund_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FundChannel_Start" => {
                    #[allow(non_camel_case_types)]
                    struct FundChannel_StartSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::FundchannelStartRequest>
                    for FundChannel_StartSvc<T> {
                        type Response = super::FundchannelStartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundchannelStartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::fund_channel_start(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundChannel_StartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/GetLog" => {
                    #[allow(non_camel_case_types)]
                    struct GetLogSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::GetlogRequest>
                    for GetLogSvc<T> {
                        type Response = super::GetlogResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetlogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::get_log(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/FunderUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct FunderUpdateSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::FunderupdateRequest>
                    for FunderUpdateSvc<T> {
                        type Response = super::FunderupdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FunderupdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::funder_update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FunderUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/GetRoute" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::GetrouteRequest>
                    for GetRouteSvc<T> {
                        type Response = super::GetrouteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetrouteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::get_route(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListForwards" => {
                    #[allow(non_camel_case_types)]
                    struct ListForwardsSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListforwardsRequest>
                    for ListForwardsSvc<T> {
                        type Response = super::ListforwardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListforwardsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_forwards(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListForwardsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListOffers" => {
                    #[allow(non_camel_case_types)]
                    struct ListOffersSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListoffersRequest>
                    for ListOffersSvc<T> {
                        type Response = super::ListoffersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListoffersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_offers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListOffersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListPays" => {
                    #[allow(non_camel_case_types)]
                    struct ListPaysSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListpaysRequest>
                    for ListPaysSvc<T> {
                        type Response = super::ListpaysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListpaysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_pays(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPaysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListHtlcs" => {
                    #[allow(non_camel_case_types)]
                    struct ListHtlcsSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListhtlcsRequest>
                    for ListHtlcsSvc<T> {
                        type Response = super::ListhtlcsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListhtlcsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_htlcs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListHtlcsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/MultiFundChannel" => {
                    #[allow(non_camel_case_types)]
                    struct MultiFundChannelSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::MultifundchannelRequest>
                    for MultiFundChannelSvc<T> {
                        type Response = super::MultifundchannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MultifundchannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::multi_fund_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiFundChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/MultiWithdraw" => {
                    #[allow(non_camel_case_types)]
                    struct MultiWithdrawSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::MultiwithdrawRequest>
                    for MultiWithdrawSvc<T> {
                        type Response = super::MultiwithdrawResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MultiwithdrawRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::multi_withdraw(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiWithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Offer" => {
                    #[allow(non_camel_case_types)]
                    struct OfferSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::OfferRequest>
                    for OfferSvc<T> {
                        type Response = super::OfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/OpenChannel_Abort" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannel_AbortSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::OpenchannelAbortRequest>
                    for OpenChannel_AbortSvc<T> {
                        type Response = super::OpenchannelAbortResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenchannelAbortRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::open_channel_abort(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannel_AbortSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/OpenChannel_Bump" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannel_BumpSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::OpenchannelBumpRequest>
                    for OpenChannel_BumpSvc<T> {
                        type Response = super::OpenchannelBumpResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenchannelBumpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::open_channel_bump(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannel_BumpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/OpenChannel_Init" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannel_InitSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::OpenchannelInitRequest>
                    for OpenChannel_InitSvc<T> {
                        type Response = super::OpenchannelInitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenchannelInitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::open_channel_init(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannel_InitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/OpenChannel_Signed" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannel_SignedSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::OpenchannelSignedRequest>
                    for OpenChannel_SignedSvc<T> {
                        type Response = super::OpenchannelSignedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenchannelSignedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::open_channel_signed(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannel_SignedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/OpenChannel_Update" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannel_UpdateSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::OpenchannelUpdateRequest>
                    for OpenChannel_UpdateSvc<T> {
                        type Response = super::OpenchannelUpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenchannelUpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::open_channel_update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannel_UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::PingRequest>
                    for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::ping(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Plugin" => {
                    #[allow(non_camel_case_types)]
                    struct PluginSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::PluginRequest>
                    for PluginSvc<T> {
                        type Response = super::PluginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PluginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::plugin(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PluginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/RenePayStatus" => {
                    #[allow(non_camel_case_types)]
                    struct RenePayStatusSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::RenepaystatusRequest>
                    for RenePayStatusSvc<T> {
                        type Response = super::RenepaystatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenepaystatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::rene_pay_status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenePayStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/RenePay" => {
                    #[allow(non_camel_case_types)]
                    struct RenePaySvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::RenepayRequest>
                    for RenePaySvc<T> {
                        type Response = super::RenepayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenepayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::rene_pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenePaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ReserveInputs" => {
                    #[allow(non_camel_case_types)]
                    struct ReserveInputsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::ReserveinputsRequest>
                    for ReserveInputsSvc<T> {
                        type Response = super::ReserveinputsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReserveinputsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::reserve_inputs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReserveInputsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SendCustomMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SendCustomMsgSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::SendcustommsgRequest>
                    for SendCustomMsgSvc<T> {
                        type Response = super::SendcustommsgResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendcustommsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::send_custom_msg(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendCustomMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SendInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct SendInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SendinvoiceRequest>
                    for SendInvoiceSvc<T> {
                        type Response = super::SendinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::send_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SetChannel" => {
                    #[allow(non_camel_case_types)]
                    struct SetChannelSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SetchannelRequest>
                    for SetChannelSvc<T> {
                        type Response = super::SetchannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetchannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::set_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SetconfigRequest>
                    for SetConfigSvc<T> {
                        type Response = super::SetconfigResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetconfigRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::set_config(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SetPsbtVersion" => {
                    #[allow(non_camel_case_types)]
                    struct SetPsbtVersionSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::SetpsbtversionRequest>
                    for SetPsbtVersionSvc<T> {
                        type Response = super::SetpsbtversionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetpsbtversionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::set_psbt_version(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPsbtVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SignInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct SignInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SigninvoiceRequest>
                    for SignInvoiceSvc<T> {
                        type Response = super::SigninvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SigninvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::sign_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SignmessageRequest>
                    for SignMessageSvc<T> {
                        type Response = super::SignmessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignmessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::sign_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Splice_Init" => {
                    #[allow(non_camel_case_types)]
                    struct Splice_InitSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SpliceInitRequest>
                    for Splice_InitSvc<T> {
                        type Response = super::SpliceInitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpliceInitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::splice_init(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Splice_InitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Splice_Signed" => {
                    #[allow(non_camel_case_types)]
                    struct Splice_SignedSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SpliceSignedRequest>
                    for Splice_SignedSvc<T> {
                        type Response = super::SpliceSignedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpliceSignedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::splice_signed(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Splice_SignedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Splice_Update" => {
                    #[allow(non_camel_case_types)]
                    struct Splice_UpdateSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::SpliceUpdateRequest>
                    for Splice_UpdateSvc<T> {
                        type Response = super::SpliceUpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpliceUpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::splice_update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = Splice_UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/UnreserveInputs" => {
                    #[allow(non_camel_case_types)]
                    struct UnreserveInputsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::UnreserveinputsRequest>
                    for UnreserveInputsSvc<T> {
                        type Response = super::UnreserveinputsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnreserveinputsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::unreserve_inputs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnreserveInputsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/UpgradeWallet" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeWalletSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::UpgradewalletRequest>
                    for UpgradeWalletSvc<T> {
                        type Response = super::UpgradewalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpgradewalletRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::upgrade_wallet(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpgradeWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/WaitBlockHeight" => {
                    #[allow(non_camel_case_types)]
                    struct WaitBlockHeightSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::WaitblockheightRequest>
                    for WaitBlockHeightSvc<T> {
                        type Response = super::WaitblockheightResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitblockheightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::wait_block_height(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitBlockHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Wait" => {
                    #[allow(non_camel_case_types)]
                    struct WaitSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::WaitRequest>
                    for WaitSvc<T> {
                        type Response = super::WaitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::wait(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ListConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct ListConfigsSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ListconfigsRequest>
                    for ListConfigsSvc<T> {
                        type Response = super::ListconfigsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListconfigsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::list_configs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConfigsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::StopRequest>
                    for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::stop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/Help" => {
                    #[allow(non_camel_case_types)]
                    struct HelpSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::HelpRequest>
                    for HelpSvc<T> {
                        type Response = super::HelpResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::help(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HelpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/PreApproveKeysend" => {
                    #[allow(non_camel_case_types)]
                    struct PreApproveKeysendSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::PreapprovekeysendRequest>
                    for PreApproveKeysendSvc<T> {
                        type Response = super::PreapprovekeysendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PreapprovekeysendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::pre_approve_keysend(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PreApproveKeysendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/PreApproveInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct PreApproveInvoiceSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::PreapproveinvoiceRequest>
                    for PreApproveInvoiceSvc<T> {
                        type Response = super::PreapproveinvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PreapproveinvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::pre_approve_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PreApproveInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/StaticBackup" => {
                    #[allow(non_camel_case_types)]
                    struct StaticBackupSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::StaticbackupRequest>
                    for StaticBackupSvc<T> {
                        type Response = super::StaticbackupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StaticbackupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::static_backup(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StaticBackupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprChannelsApy" => {
                    #[allow(non_camel_case_types)]
                    struct BkprChannelsApySvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BkprchannelsapyRequest>
                    for BkprChannelsApySvc<T> {
                        type Response = super::BkprchannelsapyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprchannelsapyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_channels_apy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprChannelsApySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprDumpIncomeCsv" => {
                    #[allow(non_camel_case_types)]
                    struct BkprDumpIncomeCsvSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BkprdumpincomecsvRequest>
                    for BkprDumpIncomeCsvSvc<T> {
                        type Response = super::BkprdumpincomecsvResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprdumpincomecsvRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_dump_income_csv(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprDumpIncomeCsvSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprInspect" => {
                    #[allow(non_camel_case_types)]
                    struct BkprInspectSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::BkprinspectRequest>
                    for BkprInspectSvc<T> {
                        type Response = super::BkprinspectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprinspectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_inspect(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprInspectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprListAccountEvents" => {
                    #[allow(non_camel_case_types)]
                    struct BkprListAccountEventsSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BkprlistaccounteventsRequest>
                    for BkprListAccountEventsSvc<T> {
                        type Response = super::BkprlistaccounteventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprlistaccounteventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_list_account_events(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprListAccountEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprListBalances" => {
                    #[allow(non_camel_case_types)]
                    struct BkprListBalancesSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BkprlistbalancesRequest>
                    for BkprListBalancesSvc<T> {
                        type Response = super::BkprlistbalancesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprlistbalancesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_list_balances(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprListBalancesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BkprListIncome" => {
                    #[allow(non_camel_case_types)]
                    struct BkprListIncomeSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BkprlistincomeRequest>
                    for BkprListIncomeSvc<T> {
                        type Response = super::BkprlistincomeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BkprlistincomeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::bkpr_list_income(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BkprListIncomeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/BlacklistRune" => {
                    #[allow(non_camel_case_types)]
                    struct BlacklistRuneSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::UnaryService<super::BlacklistruneRequest>
                    for BlacklistRuneSvc<T> {
                        type Response = super::BlacklistruneResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlacklistruneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::blacklist_rune(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlacklistRuneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CheckRune" => {
                    #[allow(non_camel_case_types)]
                    struct CheckRuneSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::CheckruneRequest>
                    for CheckRuneSvc<T> {
                        type Response = super::CheckruneResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckruneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::check_rune(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckRuneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/CreateRune" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRuneSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::CreateruneRequest>
                    for CreateRuneSvc<T> {
                        type Response = super::CreateruneResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateruneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::create_rune(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRuneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/ShowRunes" => {
                    #[allow(non_camel_case_types)]
                    struct ShowRunesSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::ShowrunesRequest>
                    for ShowRunesSvc<T> {
                        type Response = super::ShowrunesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShowrunesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::show_runes(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShowRunesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SubscribeBlockAdded" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeBlockAddedSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::ServerStreamingService<
                        super::StreamBlockAddedRequest,
                    > for SubscribeBlockAddedSvc<T> {
                        type Response = super::BlockAddedNotification;
                        type ResponseStream = T::SubscribeBlockAddedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamBlockAddedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::subscribe_block_added(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeBlockAddedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SubscribeChannelOpenFailed" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeChannelOpenFailedSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::ServerStreamingService<
                        super::StreamChannelOpenFailedRequest,
                    > for SubscribeChannelOpenFailedSvc<T> {
                        type Response = super::ChannelOpenFailedNotification;
                        type ResponseStream = T::SubscribeChannelOpenFailedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::StreamChannelOpenFailedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::subscribe_channel_open_failed(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeChannelOpenFailedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SubscribeChannelOpened" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeChannelOpenedSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::ServerStreamingService<
                        super::StreamChannelOpenedRequest,
                    > for SubscribeChannelOpenedSvc<T> {
                        type Response = super::ChannelOpenedNotification;
                        type ResponseStream = T::SubscribeChannelOpenedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamChannelOpenedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::subscribe_channel_opened(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeChannelOpenedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SubscribeConnect" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeConnectSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::ServerStreamingService<super::StreamConnectRequest>
                    for SubscribeConnectSvc<T> {
                        type Response = super::PeerConnectNotification;
                        type ResponseStream = T::SubscribeConnectStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamConnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::subscribe_connect(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeConnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cln.Node/SubscribeCustomMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCustomMsgSvc<T: Node>(pub Arc<T>);
                    impl<
                        T: Node,
                    > tonic::server::ServerStreamingService<
                        super::StreamCustomMsgRequest,
                    > for SubscribeCustomMsgSvc<T> {
                        type Response = super::CustomMsgNotification;
                        type ResponseStream = T::SubscribeCustomMsgStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamCustomMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::subscribe_custom_msg(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCustomMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Node> Clone for NodeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Node> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Node> tonic::server::NamedService for NodeServer<T> {
        const NAME: &'static str = "cln.Node";
    }
}
