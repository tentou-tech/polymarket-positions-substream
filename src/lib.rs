mod abi;
#[allow(unused)]
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)] // Might not be needed depending on actual ABI, hence the allow
use {num_traits::cast::ToPrimitive, std::str::FromStr, substreams::scalar::BigDecimal};

substreams_ethereum::init!();

const CONDITIONAL_TOKENS_TRACKED_CONTRACT: [u8; 20] = hex!("4d97dcd97ec945f40cf65f87097ace5ea0476045");

fn map_conditional_tokens_events(blk: &eth::Block, events: &mut contract::Events) {
    events.conditional_tokens_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::ConditionalTokensApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_condition_preparations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::ConditionPreparation::match_and_decode(log) {
                        return Some(contract::ConditionalTokensConditionPreparation {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            condition_id: Vec::from(event.condition_id),
                            oracle: event.oracle,
                            outcome_slot_count: event.outcome_slot_count.to_string(),
                            question_id: Vec::from(event.question_id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_condition_resolutions.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::ConditionResolution::match_and_decode(log) {
                        return Some(contract::ConditionalTokensConditionResolution {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            condition_id: Vec::from(event.condition_id),
                            oracle: event.oracle,
                            outcome_slot_count: event.outcome_slot_count.to_string(),
                            payout_numerators: event.payout_numerators.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            question_id: Vec::from(event.question_id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_payout_redemptions.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::PayoutRedemption::match_and_decode(log) {
                        return Some(contract::ConditionalTokensPayoutRedemption {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collateral_token: event.collateral_token,
                            condition_id: Vec::from(event.condition_id),
                            index_sets: event.index_sets.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            parent_collection_id: Vec::from(event.parent_collection_id),
                            payout: event.payout.to_string(),
                            redeemer: event.redeemer,
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_position_splits.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::PositionSplit::match_and_decode(log) {
                        return Some(contract::ConditionalTokensPositionSplit {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            collateral_token: event.collateral_token,
                            condition_id: Vec::from(event.condition_id),
                            parent_collection_id: Vec::from(event.parent_collection_id),
                            partition: event.partition.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            stakeholder: event.stakeholder,
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_positions_merges.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::PositionsMerge::match_and_decode(log) {
                        return Some(contract::ConditionalTokensPositionsMerge {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            collateral_token: event.collateral_token,
                            condition_id: Vec::from(event.condition_id),
                            parent_collection_id: Vec::from(event.parent_collection_id),
                            partition: event.partition.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            stakeholder: event.stakeholder,
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_transfer_batches.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::TransferBatch::match_and_decode(log) {
                        return Some(contract::ConditionalTokensTransferBatch {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            ids: event.ids.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            operator: event.operator,
                            to: event.to,
                            values: event.values.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_transfer_singles.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::TransferSingle::match_and_decode(log) {
                        return Some(contract::ConditionalTokensTransferSingle {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            id: event.id.to_string(),
                            operator: event.operator,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.conditional_tokens_uris.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CONDITIONAL_TOKENS_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::conditional_tokens_contract::events::Uri::match_and_decode(log) {
                        return Some(contract::ConditionalTokensUri {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: event.id.to_string(),
                            value: event.value,
                        });
                    }

                    None
                })
        })
        .collect());
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_conditional_tokens_events(&blk, &mut events);
    Ok(events)
}

