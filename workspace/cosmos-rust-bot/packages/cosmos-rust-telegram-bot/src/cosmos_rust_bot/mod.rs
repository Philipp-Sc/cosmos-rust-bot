use cosmos_rust_interface::utils::entry::UserMetaData;

mod static_commands;
mod commands;

use static_commands::*;
use commands::*;


pub async fn handle_message(user_id: u64, message: String, db: &sled::Db) {

    let mut msg = String::with_capacity(message.len());
    message.trim().to_lowercase().replace("/","").replace("_"," ").replace("\n","").split_whitespace().for_each(|w| {
        if !msg.is_empty() {
            msg.push(' ');
        }
        msg.push_str(w);
    });

    let msg_for_query = msg.replace(" subscribe","").replace(" unsubscribe","");

    let user_hash = UserMetaData::user_hash(user_id);

    handle_start(user_hash,&msg,db)
        .map_err(|_|handle_about(user_hash,&msg,db)
        .map_err(|_|handle_help(user_hash,&msg,db)
        .map_err(|_|handle_help_tasks(user_hash,&msg,db)
        .map_err(|_|handle_help_governance_proposals(user_hash,&msg,db)
        .map_err(|_|handle_help_subscriptions(user_hash,&msg,db)
        .map_err(|_|handle_common_subs(user_hash,&msg,db)
        .map_err(|_|handle_tasks(user_hash,&msg,db)
        .map_err(|_|handle_governance_proposals(user_hash,&msg,db)
        .map_err(|_|handle_proposal_by_id(user_hash,&msg,db)
        .map_err(|_|handle_latest_proposals(user_hash,&msg,db)
        .map_err(|_|handle_proposals_by_status(user_hash,&msg,db)
        .map_err(|_|handle_tasks_logs_errors_debug(user_hash,&msg, &msg_for_query,db)
        .map_err(|_|handle_tasks_count_list_history(user_hash,&msg, &msg_for_query,db)
        .map_err(|_|handle_subscribe_unsubscribe(user_hash,&msg, &msg_for_query,db)
        .map_err(|_|handle_gov_prpsl(user_hash,&msg, &msg_for_query,db)
        .map_err(|_|handle_unknown_command(user_hash,db).ok())))))))))))))))).ok();

}
