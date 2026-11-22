use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::shared::loot_item_vanilla_tbc_wrath::LootItem;
use wow_world_base::shared::loot_method_error_vanilla_tbc_wrath::LootMethodError;
use wow_world_base::shared::loot_method_vanilla_tbc_wrath::LootMethod;
use wow_world_base::shared::loot_slot_type_vanilla_tbc_wrath::LootSlotType;

/// LyraCore fork: the `ErrorX` form is the guid, the loot method and the error: 10 bytes, with no
/// `gold` and no `amount_of_items`, as 1.12 sends it. `gold` is `0` and `items` is empty when
/// read, and neither is written.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:73`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L73):
/// ```text
/// smsg SMSG_LOOT_RESPONSE = 0x0160 {
///     Guid guid;
///     LootMethod loot_method;
///     if (loot_method == ERROR) {
///         LootMethodError loot_error;
///     }
///     else {
///         Gold gold;
///         u8 amount_of_items;
///         LootItem[amount_of_items] items;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: SMSG_LOOT_RESPONSE_LootMethod,
    pub gold: Gold,
    pub items: Vec<LootItem>,
}

impl crate::private::Sealed for SMSG_LOOT_RESPONSE {}
impl SMSG_LOOT_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=1551).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // loot_method: LootMethod
        let loot_method = crate::util::read_u8_le(&mut r)?.try_into()?;

        let loot_method_if = match loot_method {
            LootMethod::ErrorX => {
                // loot_error: LootMethodError
                let loot_error = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                    loot_error,
                }
            }
            LootMethod::Corpse => SMSG_LOOT_RESPONSE_LootMethod::Corpse,
            LootMethod::Pickpocketing => SMSG_LOOT_RESPONSE_LootMethod::Pickpocketing,
            LootMethod::Fishing => SMSG_LOOT_RESPONSE_LootMethod::Fishing,
            LootMethod::Disenchanting => SMSG_LOOT_RESPONSE_LootMethod::Disenchanting,
            LootMethod::Skinning => SMSG_LOOT_RESPONSE_LootMethod::Skinning,
            LootMethod::Fishinghole => SMSG_LOOT_RESPONSE_LootMethod::Fishinghole,
            LootMethod::FishingFail => SMSG_LOOT_RESPONSE_LootMethod::FishingFail,
            LootMethod::Insignia => SMSG_LOOT_RESPONSE_LootMethod::Insignia,
        };

        if matches!(loot_method_if, SMSG_LOOT_RESPONSE_LootMethod::ErrorX { .. }) {
            return Ok(Self {
                guid,
                loot_method: loot_method_if,
                gold: Gold::new(0),
                items: Vec::new(),
            });
        }

        // gold: Gold
        let gold = Gold::new(crate::util::read_u32_le(&mut r)?);

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // items: LootItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for _ in 0..amount_of_items {
                items.push(LootItem::read(&mut r)?);
            }
            items
        };

        Ok(Self {
            guid,
            loot_method: loot_method_if,
            gold,
            items,
        })
    }

}

impl crate::Message for SMSG_LOOT_RESPONSE {
    const OPCODE: u32 = 0x0160;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOOT_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    loot_method = {};", LootMethod::try_from(self.loot_method.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.loot_method {
            crate::shared::smsg_loot_response_vanilla_tbc_wrath::SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                loot_error,
            } => {
                writeln!(s, "    loot_error = {};", loot_error.as_test_case_value()).unwrap();
            }
            _ => {}
        }

        if !matches!(self.loot_method, SMSG_LOOT_RESPONSE_LootMethod::ErrorX { .. }) {
            writeln!(s, "    gold = {};", self.gold.as_int()).unwrap();
            writeln!(s, "    amount_of_items = {};", self.items.len()).unwrap();
            writeln!(s, "    items = [").unwrap();
            for v in self.items.as_slice() {
                writeln!(s, "        {{").unwrap();
                // Members
                writeln!(s, "            index = {};", v.index).unwrap();
                writeln!(s, "            item = {};", v.item).unwrap();
                writeln!(s, "            ty = {};", v.ty.as_test_case_value()).unwrap();

                writeln!(s, "        }},").unwrap();
            }
            writeln!(s, "    ];").unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 352_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "loot_method", "    ");
        match &self.loot_method {
            crate::shared::smsg_loot_response_vanilla_tbc_wrath::SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                loot_error,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "loot_error", "    ");
            }
            _ => {}
        }

        if !matches!(self.loot_method, SMSG_LOOT_RESPONSE_LootMethod::ErrorX { .. }) {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "gold", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_items", "    ");
            if !self.items.is_empty() {
                writeln!(s, "    /* items: LootItem[amount_of_items] start */").unwrap();
                for (i, v) in self.items.iter().enumerate() {
                    writeln!(s, "    /* items: LootItem[amount_of_items] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "index", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "ty", "        ");
                    writeln!(s, "    /* items: LootItem[amount_of_items] {i} end */").unwrap();
                }
                writeln!(s, "    /* items: LootItem[amount_of_items] end */").unwrap();
            }
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int().to_le_bytes()))?;

        if let SMSG_LOOT_RESPONSE_LootMethod::ErrorX { loot_error } = &self.loot_method {
            // loot_error: LootMethodError
            w.write_all(&(loot_error.as_int().to_le_bytes()))?;
            return Ok(());
        }

        // gold: Gold
        w.write_all((self.gold.as_int()).to_le_bytes().as_slice())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: LootItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(352, "SMSG_LOOT_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_RESPONSE {}

impl SMSG_LOOT_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        let loot_method = self.loot_method.size(); // loot_method: SMSG_LOOT_RESPONSE_LootMethod
        if matches!(self.loot_method, SMSG_LOOT_RESPONSE_LootMethod::ErrorX { .. }) {
            return 8 + loot_method; // guid: Guid
        }
        8 // guid: Guid
        + loot_method
        + 4 // gold: Gold
        + 1 // amount_of_items: u8
        + self.items.len() * 6 // items: LootItem[amount_of_items]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_LOOT_RESPONSE_LootMethod {
    ErrorX {
        loot_error: LootMethodError,
    },
    Corpse,
    Pickpocketing,
    Fishing,
    Disenchanting,
    Skinning,
    Fishinghole,
    FishingFail,
    Insignia,
}

impl Default for SMSG_LOOT_RESPONSE_LootMethod {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Corpse
    }
}

impl SMSG_LOOT_RESPONSE_LootMethod {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ErrorX { .. } => 0,
            Self::Corpse => 1,
            Self::Pickpocketing => 2,
            Self::Fishing => 3,
            Self::Disenchanting => 4,
            Self::Skinning => 6,
            Self::Fishinghole => 20,
            Self::FishingFail => 21,
            Self::Insignia => 22,
        }
    }

}

impl std::fmt::Display for SMSG_LOOT_RESPONSE_LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrorX{ .. } => f.write_str("ErrorX"),
            Self::Corpse => f.write_str("Corpse"),
            Self::Pickpocketing => f.write_str("Pickpocketing"),
            Self::Fishing => f.write_str("Fishing"),
            Self::Disenchanting => f.write_str("Disenchanting"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Fishinghole => f.write_str("Fishinghole"),
            Self::FishingFail => f.write_str("FishingFail"),
            Self::Insignia => f.write_str("Insignia"),
        }
    }
}

impl SMSG_LOOT_RESPONSE_LootMethod {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::ErrorX {
                ..
            } => {
                1
                + 1 // loot_error: LootMethodError
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::ServerMessage;

    const ERROR_HEADER_SIZE: usize = 2 + 2;
    const ERROR_RAW: [u8; 14] = [
        0x00, 0x0C, 0x60, 0x01, // size 12 (big endian), opcode 0x0160
        0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0xF1, // guid
        0x00, // loot method ErrorX
        0x04, // loot error TooFar
    ];

    fn error() -> SMSG_LOOT_RESPONSE {
        SMSG_LOOT_RESPONSE {
            guid: Guid::new(0xF130_0000_0000_0007),
            loot_method: SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                loot_error: LootMethodError::TooFar,
            },
            gold: Gold::new(0),
            items: Vec::new(),
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn a_loot_error_reads_as_ten_bytes_without_gold_or_an_item_count() {
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&ERROR_RAW)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOOT_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &error());
        assert_eq!(t.size() + ERROR_HEADER_SIZE, ERROR_RAW.len());
    }

    #[cfg(feature = "sync")]
    #[test]
    fn a_loot_error_writes_as_ten_bytes_without_gold_or_an_item_count() {
        let mut dest = Vec::with_capacity(ERROR_RAW.len());
        error()
            .write_unencrypted_server(&mut std::io::Cursor::new(&mut dest))
            .unwrap();

        assert_eq!(dest, ERROR_RAW);
    }

    #[cfg(feature = "sync")]
    #[test]
    fn a_corpse_loot_still_reads_gold_and_an_item_count() {
        let raw = [
            0x00, 0x10, 0x60, 0x01, // size 16, opcode 0x0160
            0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0xF1, // guid
            0x01, // loot method Corpse
            0x0D, 0x00, 0x00, 0x00, // gold 13
            0x00, // no items
        ];
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let ServerOpcodeMessage::SMSG_LOOT_RESPONSE(t) = t else {
            panic!("incorrect opcode");
        };
        assert_eq!(t.loot_method, SMSG_LOOT_RESPONSE_LootMethod::Corpse);
        assert_eq!(t.gold, Gold::new(13));
        assert!(t.items.is_empty());
    }
}
