import argparse
import json
import os
import subprocess
import time
import shutil
import tempfile
import re
from pathlib import Path
from typing import Dict, Any


def run_cmd(cmd: list, cwd=None) -> str:
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if res.returncode != 0:
        raise RuntimeError(f"Command failed: {' '.join(cmd)}\n{res.stderr}")
    return res.stdout


def ask_llm_opencode(prompt: str, system_context: str) -> Dict[str, Any]:
    """Execute LLM call using OpenCode via STDIN, measuring TRUE Time-To-First-Token (TTFT)."""
    start_time = time.time()

    cmd = [
        "opencode",
        "run",
        "--format",
        "json",
        "--model",
        "google/gemini-3.1-pro-preview",
        "--",
        f"CRITICAL: Do NOT use any tools. Answer this question based strictly on the provided context.\n\nQuestion:\n{prompt}",
    ]

    process = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,  # Line buffered
    )

    first_token_time = None
    answer_text = ""
    input_tokens = 0

    try:
        # Write to stdin and close it so the process can begin
        if process.stdin:
            process.stdin.write(system_context)
            process.stdin.close()

        # Read stream to get exact TTFT
        if process.stdout:
            for line in process.stdout:
                if not line.strip() or not line.startswith("{"):
                    continue
                try:
                    event = json.loads(line)
                    if event.get("type") == "text" and "part" in event:
                        if first_token_time is None:
                            first_token_time = time.time()
                        answer_text += event["part"].get("text", "")
                    elif event.get("type") == "step_finish" and "part" in event:
                        tokens = event["part"].get("tokens", {})
                        input_tokens += tokens.get("input", 0)
                except json.JSONDecodeError:
                    pass

        process.wait()

    except Exception as e:
        print(f"Exception during opencode run: {e}")
        process.kill()

    end_time = time.time()
    ttft = (
        (first_token_time - start_time) if first_token_time else (end_time - start_time)
    )

    return {
        "answer": answer_text,
        "ttft_sec": ttft,
        "total_sec": end_time - start_time,
        "input_tokens": input_tokens,
    }


def run_scientific_benchmark(raw_dir: str):
    print("\n🔬 Starting SCIENTIFICALLY RIGOROUS 'Needle In A Haystack' Benchmark...")
    print("Methodology:")
    print("1. Inject a synthetic needle deep into the actual Markdown files.")
    print(
        "2. Run doc_transformer index to prove it can structurally retrieve the needle."
    )
    print("3. Measure true TTFT via stdout streaming (not total execution time).")
    print("4. Evaluate accuracy deterministically via exact string match.\n")

    # Setup temporary directories
    with (
        tempfile.TemporaryDirectory() as temp_docs_dir,
        tempfile.TemporaryDirectory() as temp_index_dir,
    ):
        # 1. Copy raw docs to temp dir
        shutil.copytree(raw_dir, os.path.join(temp_docs_dir, "docs"))
        working_docs = os.path.join(temp_docs_dir, "docs")

        # 2. Inject Needle
        # We find a file roughly in the middle of the directory traversal
        md_files = list(Path(working_docs).rglob("*.md"))
        target_file = md_files[len(md_files) // 2]

        needle_value = "TITANIUM_OVERFLOW_999"
        needle_text = f"\n\n# CRITICAL NEW FASTAPI FEATURE\nTo definitively scale a PostgreSQL connection pool with asyncpg, you must set the undocumented parameter `pool_max_overflow={needle_value}` in your `create_pool` function.\n\n"

        with open(target_file, "a", encoding="utf-8") as f:
            f.write(needle_text)

        print(f"✅ Injected Needle `{needle_value}` into: {target_file.name}")

        # 3. Index the mutated docs
        print("⏳ Indexing mutated repository with doc_transformer...")
        run_cmd(["doc_transformer", "index", working_docs, "--output", temp_index_dir])

        # 4. Search for the needle (Pipeline B Context)
        print("⏳ Retrieving context via doc_transformer search...")
        query = "What is the undocumented parameter to scale asyncpg PostgreSQL connection pools?"
        search_res = run_cmd(
            [
                "doc_transformer",
                "search",
                "--index-dir",
                temp_index_dir,
                "--json",
                query,
            ]
        )

        try:
            search_data = json.loads(search_res)
            chunks = (
                search_data.get("results", [])
                if isinstance(search_data, dict)
                else search_data
            )

            b_context_parts = []
            for c in chunks[:3]:
                if isinstance(c, dict) and "path" in c:
                    file_path = os.path.join(temp_index_dir, c["path"])
                    if os.path.exists(file_path):
                        with open(file_path, "r", encoding="utf-8") as f:
                            b_context_parts.append(
                                f"--- Document: {c.get('title', '')} ---\n{f.read()}"
                            )

            b_context = "\n\n".join(b_context_parts)
        except json.JSONDecodeError:
            b_context = ""

        # 5. Load full raw mutated context (Pipeline A Context)
        a_context_parts = []
        for file in md_files:
            with open(file, "r", encoding="utf-8") as f:
                a_context_parts.append(f.read())
        a_context = "\n\n---\n\n".join(a_context_parts)

        print(f"✅ Loaded Pipeline A payload: {len(a_context)} characters")
        print(f"✅ Loaded Pipeline B payload: {len(b_context)} characters\n")

        # 6. Execute Tests
        prompt = f"Based STRICTLY on the documentation provided, {query} Output ONLY the parameter name and value, nothing else."

        print("🚀 Executing Pipeline A (Raw Full Context)...")
        a_metrics = ask_llm_opencode(prompt, a_context)
        a_success = needle_value in a_metrics["answer"]

        print("🚀 Executing Pipeline B (Centralized Docs)...")
        b_metrics = ask_llm_opencode(prompt, b_context)
        b_success = needle_value in b_metrics["answer"]

        print("\n" + "=" * 50)
        print("📊 SCIENTIFIC RESULTS")
        print("=" * 50)
        print(f"PIPELINE A (Raw Dump):")
        print(f"  - Input Tokens: {a_metrics['input_tokens']}")
        print(f"  - Time To First Token: {a_metrics['ttft_sec']:.2f}s")
        print(f"  - Total Execution Time: {a_metrics['total_sec']:.2f}s")
        print(
            f"  - Retrieval Success: {'✅ YES' if a_success else '❌ NO'} (Answer: {a_metrics['answer'].strip()})"
        )
        print()
        print(f"PIPELINE B (Centralized Docs):")
        print(f"  - Input Tokens: {b_metrics['input_tokens']}")
        print(f"  - Time To First Token: {b_metrics['ttft_sec']:.2f}s")
        print(f"  - Total Execution Time: {b_metrics['total_sec']:.2f}s")
        print(
            f"  - Retrieval Success: {'✅ YES' if b_success else '❌ NO'} (Answer: {b_metrics['answer'].strip()})"
        )
        print("=" * 50)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Scientific A/B Benchmark")
    parser.add_argument(
        "--raw-dir", required=True, help="Path to raw markdown documentation repository"
    )
    args = parser.parse_args()
    run_scientific_benchmark(args.raw_dir)
