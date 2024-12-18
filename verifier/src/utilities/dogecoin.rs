use bitcoin::dogecoin::DogecoinTarget;
use bitcoin::pow::Target;
use primitive_types::U256;

/// How much time on average should occur between diffchanges.
/// The value is applicable for blocks with height >= 145000.
/// For Dogecoin, the target is set to 1 minute (60 seconds).
pub const DIFFCHANGE_TIMESPAN: u32 = 60;

/// Calculates the next target.
///
/// Ref:
/// - [`CalculateDogecoinNextWorkRequired(..)` in Dogecoin source code](https://github.com/dogecoin/dogecoin/blob/v1.14.8/src/dogecoin.cpp#L41)
pub fn calculate_dogecoin_next_work_required(
    prev_target: Target,
    start_time: u32,
    end_time: u32,
) -> Target {
    let expected = DIFFCHANGE_TIMESPAN as i64;
    let actual = {
        let actual = (end_time as i64) - (start_time as i64);
        let mut modulated = expected + (actual - expected) / 8;
        if modulated < expected - expected / 4 {
            modulated = expected - expected / 4;
        }
        if modulated > expected + expected / 2 {
            modulated = expected + expected / 2;
        }
        modulated as u32
    };

    let le_bytes = {
        let prev_target_le_bytes = prev_target.to_le_bytes();
        let x = U256::from_little_endian(&prev_target_le_bytes);
        trace!("prev-target = {x}");
        let y = x * U256::from(actual);
        trace!("prev-target * {actual} = {y}");
        let z = y / U256::from(expected);
        trace!("{y} / {expected} = {z}");

        let mut le_bytes = [0u8; 32];
        z.to_little_endian(&mut le_bytes);
        le_bytes
    };

    let target = DogecoinTarget::from_le_bytes(le_bytes);
    if target > DogecoinTarget::MAX {
        trace!("fallback to the max target");
        DogecoinTarget::MAX.into()
    } else {
        trace!("use the calculated target");
        target.into()
    }
}
