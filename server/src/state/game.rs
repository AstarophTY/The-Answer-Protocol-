use std::collections::HashMap;

use crate::protocol::response::Response;
use crate::state::group::{Group, GroupId};
use crate::state::player::Player;
use crate::state::world::WorldState;

pub enum GroupLeave {
    NotInGroup,
    Left {
        gid: GroupId,
        remaining: Vec<String>,
    },
    Disbanded {
        gid: GroupId,
        members: Vec<String>,
    },
}

pub struct GameState {
    pub players: HashMap<String, Player>,
    pub groups: HashMap<GroupId, Group>,
    pub world: WorldState,
    next_group_id: GroupId,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
            groups: HashMap::new(),
            world: WorldState::from_config(),
            next_group_id: 1,
        }
    }

    pub fn send_to(&self, name: &str, msg: Response) {
        if let Some(p) = self.players.get(name) {
            let _ = p.tx.send(msg);
        }
    }

    pub fn broadcast_room(&self, room: &str, except: Option<&str>, msg: Response) {
        for p in self.players.values() {
            if p.room == room && Some(p.name.as_str()) != except {
                let _ = p.tx.send(msg.clone());
            }
        }
    }

    pub fn broadcast_all(&self, except: Option<&str>, msg: Response) {
        for p in self.players.values() {
            if Some(p.name.as_str()) != except {
                let _ = p.tx.send(msg.clone());
            }
        }
    }

    pub fn broadcast_group(&self, gid: GroupId, except: Option<&str>, msg: Response) {
        if let Some(g) = self.groups.get(&gid) {
            for member in &g.members {
                if Some(member.as_str()) != except {
                    self.send_to(member, msg.clone());
                }
            }
        }
    }

    pub fn name_of(&self, addr: &str) -> Option<String> {
        self.players
            .values()
            .find(|p| p.addr == addr)
            .map(|p| p.name.clone())
    }

    pub fn group_by_leader(&self, leader: &str) -> Option<GroupId> {
        self.groups
            .values()
            .find(|g| g.leader == leader)
            .map(|g| g.id)
    }

    pub fn create_group(&mut self, leader: &str) -> GroupId {
        let gid = self.next_group_id;
        self.next_group_id += 1;
        self.groups.insert(gid, Group::new(gid, leader.to_string()));
        if let Some(p) = self.players.get_mut(leader) {
            p.group = Some(gid);
        }
        gid
    }

    pub fn invite_to_group(&mut self, gid: GroupId, target: &str) {
        if let Some(g) = self.groups.get_mut(&gid) {
            g.invited.insert(target.to_string());
        }
    }

    pub fn is_invited(&self, gid: GroupId, name: &str) -> bool {
        self.groups
            .get(&gid)
            .map(|g| g.invited.contains(name))
            .unwrap_or(false)
    }

    pub fn join_group(&mut self, gid: GroupId, name: &str) {
        if let Some(g) = self.groups.get_mut(&gid) {
            g.invited.remove(name);
            if !g.members.iter().any(|m| m == name) {
                g.members.push(name.to_string());
            }
        }
        if let Some(p) = self.players.get_mut(name) {
            p.group = Some(gid);
        }
    }

    pub fn leave_group(&mut self, name: &str) -> GroupLeave {
        let gid = match self.players.get(name).and_then(|p| p.group) {
            Some(gid) => gid,
            None => return GroupLeave::NotInGroup,
        };

        let is_leader = self
            .groups
            .get(&gid)
            .map(|g| g.leader == name)
            .unwrap_or(false);

        if is_leader {
            let members = self
                .groups
                .remove(&gid)
                .map(|g| g.members)
                .unwrap_or_default();
            for m in &members {
                if let Some(p) = self.players.get_mut(m) {
                    p.group = None;
                }
            }
            GroupLeave::Disbanded { gid, members }
        } else {
            if let Some(g) = self.groups.get_mut(&gid) {
                g.remove_member(name);
            }
            if let Some(p) = self.players.get_mut(name) {
                p.group = None;
            }
            let remaining = self
                .groups
                .get(&gid)
                .map(|g| g.members.clone())
                .unwrap_or_default();
            GroupLeave::Left { gid, remaining }
        }
    }
}
