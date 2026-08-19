# Reliable Work-item Publication

Work-item assignment uses an Account User's `userId`, never an Organization
Member's membership ID. Create operations resolve and validate label references,
then return a Stable Work-item Detail after bounded verification; a label repair
may update that known work item once, but creation is never retried because a
failed response can represent an already-created work item.

## Considered Options

- Expose the YunXiao API response directly and leave validation and recovery to
  each skill or caller.
- Retry a failed create request to repair missing labels.

The first option repeatedly leaks provider-specific identity and consistency
semantics to callers. The second can create duplicate work items, so recovery is
limited to the already-known work-item ID.
