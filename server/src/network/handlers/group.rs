use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::protocol::command::GroupAction;
use crate::protocol::response::Response;
use crate::state::game::{GameState, GroupLeave};

pub async fn group(
    action: GroupAction,
    addr: &str,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    match action {
        GroupAction::Create => {
            if state.players[&name].group.is_some() {
                return Response::error(402, "ALREADY_IN_GROUP");
            }
            let gid = state.create_group(&name);
            info!(player = %name, group = gid, "Group created");
            Response::ok("group", json!({ "action": "create", "group": gid }))
        }

        GroupAction::Invite { target } => {
            let gid = match state.players[&name].group {
                Some(gid) => gid,
                None => return Response::error(401, "NOT_IN_GROUP"),
            };
            if target == name {
                return Response::error(400, "Cannot invite yourself");
            }
            if !state.players.contains_key(&target) {
                return Response::error(404, "Player not found");
            }
            if state.players[&target].group.is_some() {
                return Response::error(402, "ALREADY_IN_GROUP");
            }

            state.invite_to_group(gid, &target);
            let leader = state.groups[&gid].leader.clone();

            state.send_to(
                &target,
                Response::ok(
                    "event",
                    json!({
                        "event": "group_invite",
                        "leader": leader,
                        "group": gid,
                        "from": name,
                    }),
                ),
            );

            info!(player = %name, target = %target, group = gid, "Group invite");
            Response::ok("group", json!({ "action": "invite", "invited": target }))
        }

        GroupAction::Join { leader } => {
            if state.players[&name].group.is_some() {
                return Response::error(402, "ALREADY_IN_GROUP");
            }
            let gid = match state.group_by_leader(&leader) {
                Some(gid) => gid,
                None => return Response::error(404, "No such group"),
            };
            if !state.is_invited(gid, &name) {
                return Response::error(403, "Not invited to this group");
            }

            state.join_group(gid, &name);

            state.broadcast_group(
                gid,
                Some(&name),
                Response::ok(
                    "event",
                    json!({ "event": "group_join", "name": name }),
                ),
            );

            info!(player = %name, group = gid, "Group join");
            Response::ok("group", json!({ "action": "join", "group": gid }))
        }

        GroupAction::Leave => match state.leave_group(&name) {
            GroupLeave::NotInGroup => Response::error(401, "NOT_IN_GROUP"),

            GroupLeave::Left { gid, remaining } => {
                let msg = Response::ok(
                    "event",
                    json!({ "event": "group_leave", "name": name }),
                );
                for m in &remaining {
                    state.send_to(m, msg.clone());
                }
                info!(player = %name, group = gid, "Group leave");
                Response::ok("group", json!({ "action": "leave" }))
            }

            GroupLeave::Disbanded { gid, members } => {
                let msg = Response::ok(
                    "event",
                    json!({ "event": "group_disband", "by": name }),
                );
                for m in &members {
                    if m != &name {
                        state.send_to(m, msg.clone());
                    }
                }
                info!(player = %name, group = gid, "Group disbanded");
                Response::ok("group", json!({ "action": "leave", "disbanded": true }))
            }
        },
    }
}
