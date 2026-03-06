#!/usr/bin/env bash
set -e

# Benchmark script to validate doc_transformer against 25 popular open-source repositories.
# This proves the token reduction mechanism and resilience of the AST parser.
# Run this from the root of the centralized-docs repository.

REPOS=(
	"https://github.com/tokio-rs/tokio"
	"https://github.com/rust-lang/mdBook"
	"https://github.com/psf/requests"
	"https://github.com/pallets/flask"
	"https://github.com/tiangolo/fastapi"
	"https://github.com/golang/go"
	"https://github.com/kubernetes/kubernetes"
	"https://github.com/prometheus/prometheus"
	"https://github.com/docker/cli"
	"https://github.com/hashicorp/terraform"
	"https://github.com/redis/redis"
	"https://github.com/ansible/ansible"
	"https://github.com/BurntSushi/ripgrep"
	"https://github.com/clap-rs/clap"
	"https://github.com/serde-rs/serde"
	"https://github.com/huggingface/transformers"
	"https://github.com/django/django"
	"https://github.com/celery/celery"
	"https://github.com/rust-lang/cargo"
	"https://github.com/apache/kafka"
	"https://github.com/elastic/elasticsearch"
	"https://github.com/postgres/postgres"
	"https://github.com/nginx/nginx"
	"https://github.com/git/git"
)

mkdir -p bench_output

echo "# Repository Ingestion Benchmarks" >BENCHMARKS.md
echo "Date: $(date)" >>BENCHMARKS.md
echo "Testing against popular repositories to prove ingest-git resilience and token reduction." >>BENCHMARKS.md
echo "" >>BENCHMARKS.md
echo "| Repository | Docs Found | Chunks | Raw Words | llms.txt Words | Token Reduction % | Index Time (s) |" >>BENCHMARKS.md
echo "|------------|------------|--------|-----------|----------------|-------------------|----------------|" >>BENCHMARKS.md

echo "Building release binary..."
cargo build --release

for repo in "${REPOS[@]}"; do
	echo "Benchmarking $repo..."
	slug=$(echo "$repo" | awk -F/ '{print $5}')
	out_dir="bench_output/$slug"

	mkdir -p "$out_dir"

	start_time=$(date +%s)
	# Using depth 1 to only clone the latest commit, speeding up network IO
	# Adding filter to capture only English docs if multiple languages exist (like FastAPI)
	if ./target/release/doc_transformer ingest-git "$repo" -o "$out_dir" --depth 1 --filter "^docs/en/" >"$out_dir/ingest.log" 2>&1 || ./target/release/doc_transformer ingest-git "$repo" -o "$out_dir" --depth 1 >"$out_dir/ingest.log" 2>&1; then
		end_time=$(date +%s)
		index_time=$((end_time - start_time))

		docs_found=$(jq '.documents | length' "$out_dir/INDEX.json" 2>/dev/null || echo "0")
		chunks=$(jq '.chunks | length' "$out_dir/INDEX.json" 2>/dev/null || echo "0")

		# Calculate words
		raw_words=$(find "$out_dir/.git-clone" -name "*.md" -exec cat {} + 2>/dev/null | wc -w || echo "0")
		llms_words=$(wc -w <"$out_dir/llms.txt" 2>/dev/null || echo "0")

		# Prevent division by zero
		if [ "$raw_words" -gt 0 ]; then
			reduction=$(awk "BEGIN {printf \"%.1f\", 100 - ($llms_words / $raw_words * 100)}")
		else
			reduction="0.0"
		fi

		echo "| $slug | $docs_found | $chunks | $raw_words | $llms_words | ${reduction}% | ${index_time}s |" >>BENCHMARKS.md
		echo "  -> Success: $docs_found docs, $chunks chunks, ${reduction}% token reduction"
	else
		echo "| $slug | FAILED | N/A | N/A | N/A | N/A | N/A |" >>BENCHMARKS.md
		echo "  -> Failed (Check $out_dir/ingest.log)"
	fi

	# Clean up massive git clones to save disk space
	rm -rf "$out_dir/.git-clone"
done

echo ""
echo "Benchmarks complete. Results written to BENCHMARKS.md"
