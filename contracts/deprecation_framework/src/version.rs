use crate::errors::Error;
use crate::types::DataKey;
use crate::events;
use soroban_sdk::Env;

pub fn pin_version(env: Env, version: u32) -> Result<(), Error> {
    let min_pinnable = env
        .storage()
        .instance()
        .get(&DataKey::MinPinnableVersion)
        .unwrap_or(1u32);

    if version < min_pinnable {
        events::publish_version_pin_rejected(&env, version, min_pinnable);
        return Err(Error::DowngradeNotAllowed);
    }

    env.storage().instance().set(
        &DataKey::CurrentPinnedVersion,
        &(version, env.ledger().timestamp()),
    );

    events::publish_version_pinned(&env, version, env.ledger().timestamp());
    Ok(())
}

pub fn propose_raise_min_version(
    env: Env,
    new_min_version: u32,
) -> Result<(), Error> {
    let min_pinnable = env
        .storage()
        .instance()
        .get(&DataKey::MinPinnableVersion)
        .unwrap_or(1u32);

    if new_min_version <= min_pinnable {
        return Err(Error::InvalidMinVersion);
    }

    let unlock_time = env.ledger().timestamp() + 86400; // 24 hours timelock

    env.storage()
        .instance()
        .set(&DataKey::ProposedMinVersion, &new_min_version);
    env.storage()
        .instance()
        .set(&DataKey::MinVersionUnlockTime, &unlock_time);

    events::publish_min_version_proposed(&env, new_min_version, unlock_time);
    Ok(())
}

pub fn raise_min_version(env: Env) -> Result<(), Error> {
    let proposed: u32 = env
        .storage()
        .instance()
        .get(&DataKey::ProposedMinVersion)
        .ok_or(Error::NoProposalPending)?;

    let unlock_time: u64 = env
        .storage()
        .instance()
        .get(&DataKey::MinVersionUnlockTime)
        .ok_or(Error::NoProposalPending)?;

    if env.ledger().timestamp() < unlock_time {
        return Err(Error::TimeLockNotExpired);
    }

    env.storage()
        .instance()
        .set(&DataKey::MinPinnableVersion, &proposed);

    env.storage().instance().remove(&DataKey::ProposedMinVersion);
    env.storage().instance().remove(&DataKey::MinVersionUnlockTime);

    events::publish_min_version_raised(&env, proposed);
    Ok(())
}

pub fn get_min_pinnable_version(env: Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::MinPinnableVersion)
        .unwrap_or(1u32)
}

pub fn get_current_pinned_version(env: Env) -> Option<(u32, u64)> {
    env.storage()
        .instance()
        .get(&DataKey::CurrentPinnedVersion)
}

pub fn get_proposed_min_version(env: Env) -> Option<(u32, u64)> {
    let proposed = env.storage().instance().get(&DataKey::ProposedMinVersion);
    let unlock_time = env.storage().instance().get(&DataKey::MinVersionUnlockTime);
    match (proposed, unlock_time) {
        (Some(p), Some(u)) => Some((p, u)),
        _ => None,
    }
}
