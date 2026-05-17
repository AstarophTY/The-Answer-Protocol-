use std::collections::HashSet;

pub type GroupId = u64;

pub struct Group {
    pub id: GroupId,
    pub leader: String,
    pub members: Vec<String>,
    pub invited: HashSet<String>,
}

impl Group {
    pub fn new(id: GroupId, leader: String) -> Self {
        Group {
            id,
            members: vec![leader.clone()],
            leader,
            invited: HashSet::new(),
        }
    }

    pub fn remove_member(&mut self, name: &str) {
        self.members.retain(|m| m != name);
    }
}
