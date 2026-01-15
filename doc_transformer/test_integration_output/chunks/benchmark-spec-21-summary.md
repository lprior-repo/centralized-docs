---
doc_id: benchmark-spec
chunk_id: benchmark-spec#21
chunk_level: summary
chunk_type: prose
heading: 6. Validation Checklist
token_count: 129
summary: - [ ] `cargo bench` runs without panic. - [ ] All 4 benchmark groups execute
---




- [ ] `cargo bench` runs without panic
- [ ] All 4 benchmark groups execute
- [ ] Results in `target/criterion/`
- [ ] HTML report generated and viewable

### Scaling Validation
- [ ] Time ratio (1000/100) is 5-10x (not 100x)
- [ ] Time ratio (5000/1000) is 4-7x (not 25x)
- [ ] Time ratio (10000/5000) is 1.8-2.5x (sub-quadratic)
- [ ] Edge count grows linearly with N

### Performance Targets
- [ ] N=1,000 completes in < 1 second
- [ ] N=5,000 completes in < 5 seconds
- [ ] N=10,000 completes in < 20 seconds
