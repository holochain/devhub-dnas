mod create_entry;
mod update_entry;
mod delete_entry;
// mod create_link;
// mod delete_link;

use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,
    LinkTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};


#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let result = match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreRecord(op_record) => match op_record {
            OpRecord::CreateEntry { app_entry, action } =>
                create_entry::validation( app_entry, action ),
            OpRecord::UpdateEntry { app_entry, action, original_action_hash, original_entry_hash } =>
                update_entry::validation( app_entry, action, original_action_hash, original_entry_hash ),
            OpRecord::DeleteEntry { original_action_hash, original_entry_hash, action } =>
                delete_entry::validation( original_action_hash, original_entry_hash, action ),
            // OpRecord::CreateLink { base_address, target_address, tag, link_type, action } =>
            //     create_link::validation( base_address, target_address, link_type, tag, action ),
            // OpRecord::DeleteLink { original_action_hash, base_address, action } =>
            //     delete_link::validation( original_action_hash, base_address, action ),
            // OpRecord::CreateAgent { agent, action: create },
            // OpRecord::UpdateAgent { original_key, new_key, original_action_hash, action: update },
            // OpRecord::CreateCapClaim { action: create },
            // OpRecord::CreateCapGrant { action: create },
            // OpRecord::CreatePrivateEntry { app_entry_type, action: create },
            // OpRecord::UpdatePrivateEntry { original_action_hash, original_entry_hash, app_entry_type, action: update },
            // OpRecord::UpdateCapClaim { original_action_hash, original_entry_hash, action: update },
            // OpRecord::UpdateCapGrant { original_action_hash, original_entry_hash, action: update },
            // OpRecord::Dna { dna_hash, action: dna },
            // OpRecord::OpenChain { previous_dna_hash, action: open_chain },
            // OpRecord::CloseChain { new_dna_hash, action: close_chain },
            // OpRecord::AgentValidationPkg { membrane_proof, action: agent_validation_pkg },
            // OpRecord::InitZomesComplete { action: init_zomes_complete },
            _ => valid!(),
        },
        // FlatOp::StoreEntry(op_entry),
        // FlatOp::RegisterAgentActivity(op_activity),
        // FlatOp::RegisterCreateLink { base_address, target_address, tag, link_type, action: create_link },
        // FlatOp::RegisterDeleteLink { original_action, base_address, target_address, tag, link_type, action: delete_link },
        // FlatOp::RegisterUpdate(op_update),
        // FlatOp::RegisterDelete(op_delete),
        _ => valid!(),
    };

    if let Err(WasmError{ error: WasmErrorInner::Guest(msg), .. }) = result {
        invalid!(msg)
    }

    result
}
