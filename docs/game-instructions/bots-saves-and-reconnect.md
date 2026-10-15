# Bots, saves, and reconnect

**Planned:** offline bots/save-resume at M06; online recovery at M09; configurable policies at M10.

Hosts choose total seats and bots. Four seats with two people means two bots if remaining seats are filled. Bots use the same rules and permitted information as people.

Offline autosaves follow accepted actions and load validated versioned state. Online accepted actions persist before acknowledgment; reconnect restores the same identity, committed state, and pending choices.

Guest recovery across devices is not promised. Do not reset a guest identity to troubleshoot a live game without understanding the consequence. Exact locations, recovery steps, and settings will follow implemented behavior.
