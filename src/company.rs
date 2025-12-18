use core::ops::BitOr;

pub struct CompanyID(u8);

impl CompanyID {
    pub const END: CompanyID = CompanyID(0xF);
    pub const INVALID: CompanyID = CompanyID(0xFF);

    /* 'Fake' companies used for networks */
    ///< The client is joining
    pub const INACTIVE_CLIENT: CompanyID = CompanyID(253);
    ///< The client wants a new company
    pub const NEW_COMPANY: CompanyID = CompanyID(254);
    ///< The client is spectating
    pub const SPECTATOR: CompanyID = CompanyID(255);
}

pub type Owner = CompanyID;
///< A town owns the tile, or a town is expanding
pub const OWNER_TOWN: CompanyID = CompanyID(0x0F);
///< The tile has no ownership
pub const OWNER_NONE: CompanyID = CompanyID(0x10);
///< The tile/execution is done by "water"
pub const OWNER_WATER: CompanyID = CompanyID(0x11);
///< The object is owned by a superuser / goal script
pub const OWNER_DEITY: CompanyID = CompanyID(0x0F);
///< Last + 1 owner
pub const OWNER_END: CompanyID = CompanyID(0x0F);
///< An invalid owner
pub const INVALID_OWNER: CompanyID = CompanyID::INVALID;

pub const MAX_COMPANIES: u8 = CompanyID::END.0;
///< The maximum length of a president name in characters including '\0'
pub const MAX_LENGTH_PRESIDENT_NAME_CHARS: u32 = 32;
///< The maximum length of a company name in characters including '\0'
pub const MAX_LENGTH_COMPANY_NAME_CHARS: u32 = 32;

///< The maximum number of quarters kept as performance's history
pub const MAX_HISTORY_QUARTERS: u32 = 24;

///< The minimum interval (in minutes) between competitors.
pub const MIN_COMPETITORS_INTERVAL: u32 = 0;
///< The maximum interval (in minutes) between competitors.
pub const MAX_COMPETITORS_INTERVAL: u32 = 500;

pub struct CompanyMask(pub u16);

impl From<CompanyID> for CompanyMask {
    fn from(value: CompanyID) -> Self {
        CompanyMask((value.0 as u16) << 8)
    }
}

impl BitOr<CompanyMask> for CompanyMask {
    type Output = CompanyMask;

    fn bitor(self, rhs: CompanyMask) -> Self::Output {
        CompanyMask(self.0 | rhs.0)
    }
}

impl BitOr<CompanyID> for CompanyMask {
    type Output = CompanyMask;

    fn bitor(self, rhs: CompanyID) -> Self::Output {
        self | CompanyMask::from(rhs)
    }
}

#[derive(Default)]
pub struct CompanyManagerFace {
    ///< Company manager face style.
    pub style: u32,
    ///< Company manager face bits, meaning is dependent on style.
    pub bits: u32,
    ///< Face style label.
    pub style_label: String,
}

/** The reason why the company was removed. */
pub enum CompanyRemoveReason {
    ///< The company is manually removed.
    Manual,
    ///< The company is removed due to autoclean.
    Autoclean,
    ///< The company went belly-up.
    Bankrupt,
}

impl CompanyRemoveReason {
    ///< Dummy reason for actions that don't need one.
    pub const NONE: CompanyRemoveReason = CompanyRemoveReason::Manual;
}

/** The action to do with CMD_COMPANY_CTRL. */
pub enum CompanyCtrlAction {
    ///< Create a new company.
    New,
    ///< Create a new AI company.
    NewAI,
    ///< Delete a company.
    Delete,
}

/** The action to do with CMD_COMPANY_ALLOW_LIST_CTRL. */
pub enum CompanyAllowListCtrlAction {
    ///< Create a public key.
    Add,
    ///< Remove a public key.
    Remove,
}
