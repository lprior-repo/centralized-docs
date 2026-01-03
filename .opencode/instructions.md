# OpenCode AI Instructions - TCR JAIL

## CRITICAL: You are in a TCR (Test-Commit-Revert) Jail

### The ONLY way to commit code:
```bash
./scripts/tcr.sh "Your commit message"
```

### What happens:
1. Tests are run automatically
2. If tests PASS → Commit is created
3. If tests FAIL → ALL your changes are REVERTED (deleted)

### Rules - NON-NEGOTIABLE:
1. NEVER use `git commit` directly
2. NEVER use `--no-verify` or `-n` flags
3. NEVER modify `.git/hooks/` files
4. NEVER delete SAFE_POINT tag
5. ALWAYS write tests FIRST

### If you try to escape:
- Git hooks will catch you
- Your changes will be reverted
- The jail is inescapable

### Safe rollback (for the user):
```bash
./scripts/revert-to-safe.sh
```

### Verify jail integrity:
```bash
./scripts/verify-jail.sh
```
