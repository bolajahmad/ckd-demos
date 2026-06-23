use super::error::Error;
use ckb_std::ckb_constants::Source;
use ckb_std::error::SysError;
use ckb_std::high_level::{
    load_cell_data, load_cell_lock_hash, load_script,
};
use core::result::Result;

const HASH_LEN: usize = 32;

enum Status {
    Active,
    Finalized,
    Cancelled,
}

impl Status {
    fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            1 => Ok(Self::Active),
            2 => Ok(Self::Finalized),
            3 => Ok(Self::Cancelled),
            _ => Err(Error::InvalidStatus),
        }
    }
}

// Binary layout (little-endian):
// version(1) | status(1) | winners(1) | reserved(1)
// | start_time(8) | prize(8) | metadata_hash(32)
// | host_lock_hash(32) | verifier_lock_hash(32)
struct Giveaway {
    version: u8,
    status: Status,
    winners: u8,
    start_time: u64,
    prize: u64,
    metadata_hash: [u8; HASH_LEN],
    host_lock_hash: [u8; HASH_LEN],
    verifier_lock_hash: [u8; HASH_LEN],
}

impl Giveaway {
    const DATA_LEN: usize = 116;

    fn from_slice(data: &[u8]) -> Result<Self, Error> {
        if data.len() != Self::DATA_LEN {
            return Err(Error::InvalidDataLength);
        }

        let version = data[0];
        let status = Status::from_u8(data[1])?;
        let winners = data[2];
        let start_time = read_u64_le(&data[4..12])?;
        let prize = read_u64_le(&data[12..20])?;

        let mut metadata_hash = [0u8; HASH_LEN];
        metadata_hash.copy_from_slice(&data[20..52]);

        let mut host_lock_hash = [0u8; HASH_LEN];
        host_lock_hash.copy_from_slice(&data[52..84]);

        let mut verifier_lock_hash = [0u8; HASH_LEN];
        verifier_lock_hash.copy_from_slice(&data[84..116]);

        Ok(Self {
            version,
            status,
            winners,
            start_time,
            prize,
            metadata_hash,
            host_lock_hash,
            verifier_lock_hash,
        })
    }

    fn validate_for_create(&self) -> Result<(), Error> {
        if self.version == 0 {
            return Err(Error::Encoding);
        }

        if !matches!(self.status, Status::Active) {
            return Err(Error::InvalidStatus);
        }

        if self.winners == 0 {
            return Err(Error::WrongWinnerCount);
        }

        if self.prize == 0 {
            return Err(Error::InvalidPrize);
        }

        if self.start_time == 0 {
            return Err(Error::GiveawayStarted);
        }

        if is_zero_hash(&self.host_lock_hash) {
            return Err(Error::InvalidHost);
        }

        if is_zero_hash(&self.verifier_lock_hash) {
            return Err(Error::InvalidAdmin);
        }

        if is_zero_hash(&self.metadata_hash) {
            return Err(Error::Encoding);
        }

        Ok(())
    }
}

fn read_u64_le(bytes: &[u8]) -> Result<u64, Error> {
    if bytes.len() != 8 {
        return Err(Error::Encoding);
    }

    let mut arr = [0u8; 8];
    arr.copy_from_slice(bytes);
    Ok(u64::from_le_bytes(arr))
}

fn is_zero_hash(hash: &[u8; HASH_LEN]) -> bool {
    *hash == [0u8; HASH_LEN]
}

enum Transition {
    Create,
    Update,
    Cancel,
}

fn has_group_input(index: usize) -> Result<bool, Error> {
    match load_cell_data(index, Source::GroupInput) {
        Ok(_) => Ok(true),
        Err(SysError::IndexOutOfBound) => Ok(false),
        Err(_) => Err(Error::InvalidTransition),
    }
}

fn has_group_output(index: usize) -> Result<bool, Error> {
    match load_cell_data(index, Source::GroupOutput) {
        Ok(_) => Ok(true),
        Err(SysError::IndexOutOfBound) => Ok(false),
        Err(_) => Err(Error::InvalidTransition),
    }
}

fn detect_transition() -> Result<Transition, Error> {
    let in0 = has_group_input(0)?;
    let in1 = has_group_input(1)?;
    let out0 = has_group_output(0)?;
    let out1 = has_group_output(1)?;

    match (in0, in1, out0, out1) {
        (false, false, true, false) => Ok(Transition::Create),
        (true, false, true, false) => Ok(Transition::Update),
        (true, false, false, false) => Ok(Transition::Cancel),
        _ => Err(Error::InvalidTransition),
    }
}

fn validate_common_admin_policy(giveaway: &Giveaway) -> Result<(), Error> {
    let script = load_script().map_err(|_| Error::InvalidScriptArgs)?;
    let args = script.args().raw_data();
    if !args.is_empty() {
        if args.len() != HASH_LEN {
            return Err(Error::InvalidScriptArgs);
        }

        let mut verifier_from_args = [0u8; HASH_LEN];
        verifier_from_args.copy_from_slice(args.as_ref());
        if giveaway.verifier_lock_hash != verifier_from_args {
            return Err(Error::InvalidAdmin);
        }
    }

    Ok(())
}

fn validate_create_transition() -> Result<(), Error> {
    let output_data = load_cell_data(0, Source::GroupOutput).map_err(|_| Error::InvalidTransition)?;
    let giveaway = Giveaway::from_slice(output_data.as_ref())?;
    giveaway.validate_for_create()?;

    let output_lock_hash = load_cell_lock_hash(0, Source::GroupOutput)
        .map_err(|_| Error::InvalidTransition)?;
    if output_lock_hash != giveaway.host_lock_hash {
        return Err(Error::InvalidHost);
    }

    validate_common_admin_policy(&giveaway)
}

fn validate_update_transition() -> Result<(), Error> {
    let input_data = load_cell_data(0, Source::GroupInput).map_err(|_| Error::InvalidTransition)?;
    let output_data = load_cell_data(0, Source::GroupOutput).map_err(|_| Error::InvalidTransition)?;

    let input_giveaway = Giveaway::from_slice(input_data.as_ref())?;
    let output_giveaway = Giveaway::from_slice(output_data.as_ref())?;

    if !matches!(input_giveaway.status, Status::Active)
        || !matches!(output_giveaway.status, Status::Active)
    {
        return Err(Error::InvalidStatus);
    }

    if output_giveaway.prize < input_giveaway.prize {
        return Err(Error::InvalidPrize);
    }

    if output_giveaway.host_lock_hash != input_giveaway.host_lock_hash {
        return Err(Error::InvalidHost);
    }

    if output_giveaway.verifier_lock_hash != input_giveaway.verifier_lock_hash {
        return Err(Error::InvalidAdmin);
    }

    let input_lock_hash =
        load_cell_lock_hash(0, Source::GroupInput).map_err(|_| Error::InvalidTransition)?;
    let output_lock_hash =
        load_cell_lock_hash(0, Source::GroupOutput).map_err(|_| Error::InvalidTransition)?;
    if input_lock_hash != input_giveaway.host_lock_hash || output_lock_hash != input_giveaway.host_lock_hash {
        return Err(Error::InvalidHost);
    }

    validate_common_admin_policy(&input_giveaway)
}

fn validate_cancel_transition() -> Result<(), Error> {
    let input_data = load_cell_data(0, Source::GroupInput).map_err(|_| Error::InvalidTransition)?;
    let input_giveaway = Giveaway::from_slice(input_data.as_ref())?;

    if !matches!(input_giveaway.status, Status::Active) {
        return Err(Error::InvalidStatus);
    }

    let input_lock_hash =
        load_cell_lock_hash(0, Source::GroupInput).map_err(|_| Error::InvalidTransition)?;
    if input_lock_hash != input_giveaway.host_lock_hash {
        return Err(Error::InvalidHost);
    }

    validate_common_admin_policy(&input_giveaway)
}

pub fn main() -> Result<(), Error> {
    match detect_transition()? {
        Transition::Create => validate_create_transition(),
        Transition::Update => validate_update_transition(),
        Transition::Cancel => validate_cancel_transition(),
    }
}
