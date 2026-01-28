#!/usr/bin/env bash
# TDD "Tests" for ROADMAP.md documentation updates
# These tests validate that the documentation changes are correct

set -e

ROADMAP="ROADMAP.md"

echo "=== ROADMAP.md Validation Tests ==="
echo

# Test 1: v8.0 vector embeddings should be in Future/Exploration, not as current feature
echo "Test 1: v8.0 vector embeddings in Future section..."
if grep -q "## Phase 4: Advanced Features (v8.0+)" "$ROADMAP"; then
	echo "  ✓ Phase 4 section exists (correct - marked as exploration)"
	# Check it's marked as "EXPLORATION" or "FUTURE"
	if grep -A2 "## Phase 4: Advanced Features (v8.0+)" "$ROADMAP" | grep -q "EXPLORATION\|FUTURE"; then
		echo "  ✓ Properly marked as future/exploration"
	else
		echo "  ✗ FAIL: v8.0 not clearly marked as future/exploration"
		exit 1
	fi
else
	echo "  ✗ FAIL: Phase 4 section missing"
	exit 1
fi
echo

# Test 2: v6.0 "10 MCP tools" should be in Planned, not Targets
echo "Test 2: v6.0 MCP tools claims in Planned section..."
if grep -q "10 MCP" "$ROADMAP"; then
	# Should NOT be in v6.0 Targets as "available"
	if grep "### v6.0 Targets" -A10 "$ROADMAP" | grep -q "10 MCP.*available"; then
		echo "  ✗ FAIL: '10 MCP tools available' incorrectly in current targets"
		exit 1
	else
		echo "  ✓ '10 MCP tools' not in current targets (correct)"
	fi
else
	echo "  ℹ  Note: No mention of MCP tools in roadmap"
fi
echo

# Test 3: Chunk size should be clarified
echo "Test 3: Chunk size issue clarified..."
if grep -i "chunk.*size\|512.*token" "$ROADMAP"; then
	echo "  ✓ Chunk size mentioned in roadmap"
	# Check if there's clarification about 512 tokens
	if grep -i "512.*token" "$ROADMAP" | grep -q "standard\|accepted\|known limitation\|current"; then
		echo "  ✓ Chunk size clarified as standard/current"
	else
		echo "  ℹ  Note: Chunk size mentioned but may need clarification"
	fi
else
	echo "  ✗ FAIL: Chunk size not mentioned"
	exit 1
fi
echo

# Test 4: "What Actually Works" vs "What's Planned" section should exist
echo "Test 4: 'What Actually Works' vs 'What's Planned' section..."
if grep -q "What Actually Works\|What's Planned" "$ROADMAP"; then
	echo "  ✓ Section exists in roadmap"
else
	echo "  ✗ FAIL: 'What Actually Works' / 'What's Planned' section missing"
	exit 1
fi
echo

echo "=== All Tests PASS ==="
exit 0
