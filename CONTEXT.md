# YunXiao CLI Context

## Glossary

### Project Space

A YunXiao Projex project identified by its space ID. CLI commands use a Project
Space to scope work items, work-item types, and labels.

### Organization Member

A person's membership record in an organization. A membership has its own
membership ID and is distinct from the Account User represented by that member.

### Account User

The YunXiao account identified by `userId`. Work-item responsibility is assigned
to an Account User, not to an Organization Member's membership ID.

### Work-item Label

A named tag configured within a Project Space. A label is associated with a
work item by its label ID. A label reference is either that fixed-format
hexadecimal ID or the label's exact name.

### Specification Publication

The act of recording an implementation specification as a YunXiao work item
with its required ownership and metadata, then confirming the stored work item
matches those requested values.

### Stable Work-item Detail

The authoritative work-item detail retrieved after a write. It contains the
requested fields as persisted by YunXiao and is distinct from a create
operation's initial response.
