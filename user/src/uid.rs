use ipc_userd::Userd;
use std::collections::HashSet;

pub fn get_new_uid(userd: &mut Userd) -> Result<u32, String> {
    let users = userd.get_users().map_err(|e| format!("Failed to fetch users: {:?}", e))?;

    let uids: HashSet<u32> = users.iter().map(|user| user.uid).collect();
    let mut uid: u32 = 1000;

    while uids.contains(&uid) {
        uid += 1;
    }

    Ok(uid)
}
