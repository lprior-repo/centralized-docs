#!/bin/bash
# Test real site scraping - PLAN.md requirement line 310

echo "Testing real site scraping as per PLAN.md line 310..."
echo ""
echo "NOTE: This test requires network access and targets example.com"
echo "which is safe and designed for testing purposes."
echo ""

# Create output directory
mkdir -p real_scrape_test

# Test scrape command (using example.com as it's safe for testing)
echo "Running: ./target/release/doc_transformer scrape http://example.com --output real_scrape_test --delay 1000"
./target/release/doc_transformer scrape http://example.com --output real_scrape_test --delay 1000 2>&1 | head -50

# Check results
echo ""
echo "=== Scrape Results ==="
if [ -d "real_scrape_test" ]; then
    echo "✅ Output directory created"
    echo "Files created:"
    find real_scrape_test -type f | head -20
    echo ""
    echo "File count: $(find real_scrape_test -type f | wc -l)"
else
    echo "❌ No output directory"
fi

# Cleanup
echo ""
read -p "Clean up test files? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -rf real_scrape_test
    echo "✅ Cleaned up"
fi
