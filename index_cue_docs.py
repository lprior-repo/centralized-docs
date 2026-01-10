#!/usr/bin/env python3
"""
DOC_TRANSFORMER v4.2 Implementation
Transform raw docs into AI-optimized, navigable knowledge structures
"""

import os
import json
import re
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Tuple, Optional, Set
from dataclasses import dataclass, asdict
from collections import defaultdict

# ============================================================================
# STEP 1: DISCOVER
# ============================================================================

@dataclass
class DiscoveryFile:
    source_path: str
    size_bytes: int

def discover_files(source_dir: str) -> Tuple[List[DiscoveryFile], Dict]:
    """Discover all documentation files in source directory"""
    source_path = Path(source_dir)

    if not source_path.exists():
        raise FileNotFoundError(f"Source directory not found: {source_dir}")

    files = []
    extensions = {'.md', '.mdx', '.rst', '.txt'}
    exclude_dirs = {'node_modules', '.git', '_build', 'dist', 'vendor'}

    for file_path in source_path.rglob('*'):
        # Skip excluded directories
        if any(excl in file_path.parts for excl in exclude_dirs):
            continue

        if file_path.is_file() and file_path.suffix in extensions:
            rel_path = str(file_path.relative_to(source_path))
            size = file_path.stat().st_size
            files.append(DiscoveryFile(source_path=rel_path, size_bytes=size))

    manifest = {
        "source_dir": source_dir,
        "discovered_at": datetime.now().isoformat(),
        "files": [asdict(f) for f in files],
        "total_files": len(files)
    }

    return files, manifest

# ============================================================================
# STEP 2: ANALYZE
# ============================================================================

@dataclass
class Analysis:
    source_path: str
    title: str
    frontmatter: Optional[Dict]
    headings: List[Dict]
    links: List[Dict]
    first_paragraph: str
    word_count: int
    has_code: bool
    has_tables: bool
    category: str
    content: str

def extract_title(content: str, filename: str) -> str:
    """Extract title from first H1 or use filename"""
    match = re.search(r'^# (.+)$', content, re.MULTILINE)
    if match:
        return match.group(1).strip()

    # Use filename, convert hyphens to spaces, titlecase
    title = Path(filename).stem.replace('-', ' ').replace('_', ' ')
    return title.title()

def extract_frontmatter(content: str) -> Tuple[Optional[Dict], str]:
    """Extract YAML frontmatter if present"""
    if not content.startswith('---'):
        return None, content

    lines = content.split('\n')
    if len(lines) < 2:
        return None, content

    end_idx = None
    for i in range(1, len(lines)):
        if lines[i].startswith('---'):
            end_idx = i
            break

    if end_idx is None:
        return None, content

    # Simple YAML parsing (not full parser, just basic key: value)
    fm_lines = lines[1:end_idx]
    fm_dict = {}
    for line in fm_lines:
        if ':' in line:
            key, val = line.split(':', 1)
            fm_dict[key.strip()] = val.strip()

    remaining = '\n'.join(lines[end_idx+1:])
    return fm_dict if fm_dict else None, remaining

def extract_headings(content: str) -> List[Dict]:
    """Extract heading hierarchy"""
    headings = []
    for i, line in enumerate(content.split('\n')):
        match = re.match(r'^(#{1,6})\s+(.+)$', line)
        if match:
            level = len(match.group(1))
            text = match.group(2).strip()
            headings.append({
                'level': level,
                'text': text,
                'line': i
            })
    return headings

def extract_links(content: str) -> List[Dict]:
    """Extract markdown links"""
    links = []
    pattern = r'\[([^\]]+)\]\(([^)]+)\)'
    for match in re.finditer(pattern, content):
        text = match.group(1)
        target = match.group(2)
        is_internal = not target.startswith(('http://', 'https://', 'mailto:'))
        links.append({
            'text': text,
            'target': target,
            'is_internal': is_internal
        })
    return links

def extract_first_paragraph(content: str, min_chars: int = 20) -> str:
    """Extract first substantial paragraph"""
    # Skip frontmatter and H1
    lines = content.split('\n')
    lines = [l for l in lines if l.strip() and not l.startswith('#')]

    paragraph = ''
    for line in lines:
        if line.startswith('>') or line.startswith('|'):
            continue
        paragraph += line + ' '
        if len(paragraph) >= min_chars:
            break

    return paragraph.strip()[:200]

def detect_category(filename: str, content: str) -> str:
    """Detect document category from content and name"""
    fname_lower = Path(filename).stem.lower()
    content_lower = content.lower()

    # Meta: readme, changelog, contributing, index, license
    if fname_lower in {'readme', 'changelog', 'contributing', 'index', 'license'}:
        return 'meta'

    # Tutorial: getting started, step 1, step 2, ## Step, numbered lists
    if any(term in content_lower for term in ['getting started', 'step 1', 'step 2', '## step']):
        return 'tutorial'
    if re.search(r'^\d+\.\s+', content, re.MULTILINE):
        return 'tutorial'

    # Ops: deploy, install, troubleshoot, debug, error:, production, monitoring
    if any(term in content_lower for term in ['deploy', 'install', 'troubleshoot', 'debug',
                                               'production', 'monitoring', 'error:', 'troubleshoot']):
        return 'ops'

    # Ref: ## API, ## Reference, ## Configuration, Parameters:, Returns:, Arguments:
    if any(term in content_lower for term in ['## api', '## reference', '## configuration',
                                              'parameters:', 'returns:', 'arguments:']):
        return 'ref'

    # Default to concept
    return 'concept'

def analyze_files(files: List[DiscoveryFile], source_dir: str) -> Tuple[List[Analysis], Dict]:
    """Analyze all discovered files"""
    analyses = []

    for file_obj in files:
        file_path = Path(source_dir) / file_obj.source_path

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            frontmatter, clean_content = extract_frontmatter(content)
            title = extract_title(content, file_obj.source_path)
            headings = extract_headings(clean_content)
            links = extract_links(clean_content)
            first_para = extract_first_paragraph(clean_content)
            word_count = len(clean_content.split())
            has_code = '```' in clean_content
            has_tables = '|' in clean_content and re.search(r'\|.*\|.*\|', clean_content)
            category = detect_category(file_obj.source_path, clean_content)

            analysis = Analysis(
                source_path=file_obj.source_path,
                title=title,
                frontmatter=frontmatter,
                headings=headings,
                links=links,
                first_paragraph=first_para,
                word_count=word_count,
                has_code=has_code,
                has_tables=has_tables,
                category=category,
                content=clean_content
            )
            analyses.append(analysis)

        except Exception as e:
            print(f"ANALYZE ERROR: {file_obj.source_path}: {str(e)}")
            continue

    # Build category counts
    categories = defaultdict(int)
    for analysis in analyses:
        categories[analysis.category] += 1

    return analyses, dict(categories)

# ============================================================================
# STEP 3: ASSIGN IDs
# ============================================================================

def slugify(text: str, max_len: int = 40) -> str:
    """Convert text to URL-safe slug"""
    slug = text.lower()
    slug = re.sub(r'[^a-z0-9\s-]', '', slug)
    slug = re.sub(r'[\s_-]+', '-', slug)
    slug = slug.strip('-')
    return slug[:max_len]

def assign_ids(analyses: List[Analysis]) -> Tuple[List[Analysis], Dict]:
    """Assign unique IDs to all documents"""
    link_map = {}
    id_counts = defaultdict(int)

    for analysis in analyses:
        # Extract subcategory from parent folder
        parts = Path(analysis.source_path).parts
        subcategory = parts[-2].lower() if len(parts) > 1 else 'general'

        # Create slug from filename
        filename_stem = Path(analysis.source_path).stem
        slug = slugify(filename_stem)

        # Handle duplicates
        unique_key = f"{analysis.category}/{subcategory}/{slug}"
        id_counts[unique_key] += 1
        if id_counts[unique_key] > 1:
            slug = f"{slug}-{id_counts[unique_key]}"

        doc_id = f"{analysis.category}/{subcategory}/{slug}"
        new_filename = f"{analysis.category}-{subcategory}-{slug}.md"

        # Store mapping
        link_map[analysis.source_path] = {
            'id': doc_id,
            'filename': new_filename,
            'subcategory': subcategory,
            'slug': slug
        }

    return analyses, link_map

# ============================================================================
# STEP 4: TRANSFORM
# ============================================================================

def fix_headings(content: str) -> str:
    """Fix heading structure issues"""
    lines = content.split('\n')
    heading_lines = []

    for i, line in enumerate(lines):
        match = re.match(r'^(#{1,6})\s+(.+)$', line)
        if match:
            heading_lines.append((i, len(match.group(1))))

    # Ensure no skipped levels
    for j in range(1, len(heading_lines)):
        prev_level = heading_lines[j-1][1]
        curr_level = heading_lines[j][1]
        if curr_level > prev_level + 1:
            # Demote to prev_level + 1
            line_idx = heading_lines[j][0]
            new_hashes = '#' * (prev_level + 1)
            lines[line_idx] = new_hashes + ' ' + lines[line_idx].lstrip('#').lstrip()

    # Ensure max level 4
    result = []
    for line in lines:
        match = re.match(r'^(#{5,6})\s+(.+)$', line)
        if match:
            line = '#### ' + match.group(2)
        result.append(line)

    return '\n'.join(result)

def extract_tags(analysis: Analysis) -> List[str]:
    """Generate tags from category, content, and headings"""
    tags = set()

    # Category tag
    tags.add(analysis.category)

    # Heading nouns
    for heading in analysis.headings[:3]:
        words = heading['text'].lower().split()
        for word in words:
            if len(word) > 4 and word not in {'this', 'that', 'these', 'those', 'about', 'guide'}:
                tags.add(word)

    # Tech terms (common programming keywords)
    tech_terms = {'golang', 'python', 'javascript', 'api', 'rest', 'graphql', 'cli', 'database',
                  'docker', 'kubernetes', 'testing', 'deployment', 'configuration', 'authentication'}
    content_lower = analysis.content.lower()
    for term in tech_terms:
        if term in content_lower:
            tags.add(term)

    return sorted(list(tags))[:5]

def generate_frontmatter(doc_id: str, analysis: Analysis) -> str:
    """Generate YAML frontmatter"""
    tags = extract_tags(analysis)
    tags_str = ', '.join(f'"{t}"' for t in tags)

    fm = f"""---
id: {doc_id}
title: {analysis.title}
category: {analysis.category}
tags: [{tags_str}]
---"""
    return fm

def rewrite_links(content: str, source_path: str, link_map: Dict) -> Tuple[str, List[str]]:
    """Rewrite internal links to new filenames"""
    broken_links = []
    source_dir = str(Path(source_path).parent)

    def replace_link(match):
        text = match.group(1)
        target = match.group(2)

        # Keep external links
        if target.startswith(('http://', 'https://', 'mailto:', '#')):
            return match.group(0)

        # Resolve relative path
        if target.startswith('./'):
            resolved = str(Path(source_dir) / target.lstrip('./'))
        else:
            resolved = str(Path(source_dir) / target)

        # Look up in link_map
        for src_path, mapping in link_map.items():
            if resolved.endswith(Path(src_path).name) or src_path.endswith(Path(resolved).name):
                new_filename = mapping['filename']
                return f"[{text}](./{new_filename})"

        # Not found - keep original
        broken_links.append(target)
        return match.group(0)

    pattern = r'\[([^\]]+)\]\(([^)]+)\)'
    result = re.sub(pattern, replace_link, content)

    return result, broken_links

def transform_file(analysis: Analysis, doc_id: str, link_map: Dict) -> Tuple[str, str]:
    """Transform single file"""
    # Fix headings
    content = fix_headings(analysis.content)

    # Generate frontmatter
    frontmatter = generate_frontmatter(doc_id, analysis)

    # Rewrite links
    content, broken = rewrite_links(content, analysis.source_path, link_map)

    # Ensure single H1
    if not re.search(r'^# [^#]', content, re.MULTILINE):
        content = f"# {analysis.title}\n\n{content}"

    # Add context block if missing
    if '> **Context**:' not in content:
        context_text = analysis.first_paragraph[:150] or analysis.title
        context = f"> **Context**: {context_text}\n"
        # Insert after H1
        content = re.sub(r'^(# .+\n)', r'\1\n' + context + '\n', content)

    # Add See Also section if missing
    if '## See Also' not in content:
        see_also = "\n## See Also\n\n- [Documentation Index](./COMPASS.md)\n"
        content += see_also

    # Assemble final content
    final = f"{frontmatter}\n\n{content}"

    return final, '\n'.join(broken) if broken else ''

# ============================================================================
# STEP 5: CHUNK
# ============================================================================

@dataclass
class Chunk:
    doc_id: str
    chunk_id: str
    heading_path: List[str]
    chunk_type: str
    tokens: int
    summary: str
    content: str

def estimate_tokens(text: str) -> int:
    """Estimate token count (rough: word_count * 1.3)"""
    word_count = len(text.split())
    return int(word_count * 1.3)

def create_chunks(doc_id: str, content: str) -> List[Chunk]:
    """Split document into chunks at H2 boundaries"""
    chunks = []
    chunk_num = 1

    # Split at H2 headings
    h2_pattern = r'^## (.+)$'
    sections = re.split(h2_pattern, content, flags=re.MULTILINE)

    heading_path = []
    current_heading = None

    for i in range(0, len(sections), 2):
        if i + 1 < len(sections):
            current_heading = sections[i + 1]
            section_content = sections[i + 2] if i + 2 < len(sections) else ''
        else:
            section_content = sections[i]

        if not section_content.strip():
            continue

        section_content = f"## {current_heading}\n{section_content}" if current_heading else section_content

        # Detect chunk type
        code_lines = len(re.findall(r'^```', section_content, re.MULTILINE))
        has_table = bool(re.search(r'\|.*\|', section_content))

        if code_lines > 5:
            chunk_type = 'code'
        elif has_table:
            chunk_type = 'table'
        else:
            chunk_type = 'prose'

        tokens = estimate_tokens(section_content)
        summary = section_content.split('\n')[0][:100].strip()

        chunk_id = f"{doc_id}#chunk-{chunk_num}"
        chunk = Chunk(
            doc_id=doc_id,
            chunk_id=chunk_id,
            heading_path=heading_path + [current_heading] if current_heading else heading_path,
            chunk_type=chunk_type,
            tokens=tokens,
            summary=summary,
            content=section_content
        )
        chunks.append(chunk)
        chunk_num += 1

    return chunks

# ============================================================================
# STEP 6: INDEX
# ============================================================================

def build_index(analyses: List[Analysis], link_map: Dict, chunks_data: Dict) -> Dict:
    """Build machine-readable INDEX.json"""
    documents = []
    keywords = defaultdict(list)
    graph_links = []

    for analysis in analyses:
        mapping = link_map.get(analysis.source_path, {})
        doc_id = mapping.get('id', '')

        # Extract keywords from tags and content
        content_words = set(analysis.content.lower().split())
        stopwords = {'the', 'a', 'an', 'and', 'or', 'for', 'to', 'of', 'in', 'on', 'with', 'is', 'are'}

        for word in analysis.headings:
            heading_words = word['text'].lower().split()
            for w in heading_words:
                if w not in stopwords and len(w) > 3:
                    keywords[w].append(doc_id)

        doc_entry = {
            'id': doc_id,
            'title': analysis.title,
            'path': f"docs/{mapping.get('filename', '')}",
            'category': analysis.category,
            'subcategory': mapping.get('subcategory', ''),
            'tags': extract_tags(analysis),
            'summary': analysis.first_paragraph,
            'word_count': analysis.word_count,
            'chunk_ids': chunks_data.get(doc_id, [])
        }
        documents.append(doc_entry)

        # Build graph from links
        for link in analysis.links:
            if link['is_internal']:
                graph_links.append({
                    'from': doc_id,
                    'to': link['target'],
                    'type': 'internal_link'
                })

    index = {
        'version': '4.2',
        'generated': datetime.now().isoformat(),
        'stats': {
            'doc_count': len(documents),
            'chunk_count': sum(len(cids) for cids in chunks_data.values())
        },
        'documents': documents,
        'graph': {'links': graph_links},
        'keywords': dict(keywords)
    }

    return index

def build_compass(analyses: List[Analysis], link_map: Dict) -> str:
    """Build COMPASS.md navigation document"""
    by_category = defaultdict(list)
    for analysis in analyses:
        mapping = link_map.get(analysis.source_path, {})
        by_category[analysis.category].append({
            'title': analysis.title,
            'filename': mapping.get('filename', ''),
            'tags': extract_tags(analysis)
        })

    compass = f"""---
id: meta/navigation/compass
title: Documentation Compass
generated: {datetime.now().isoformat()}
---

# Documentation Compass

> **{len(analyses)} documents** | Last updated: {datetime.now().strftime('%Y-%m-%d')}

## Quick Navigation

### By Category
"""

    for category in ['tutorial', 'concept', 'ref', 'ops', 'meta']:
        docs = by_category.get(category, [])
        if docs:
            compass += f"\n### {category.title()}\n"
            for doc in docs[:5]:
                tags = ' '.join(f'`{t}`' for t in doc['tags'][:2])
                compass += f"- [{doc['title']}](./docs/{doc['filename']}) {tags}\n"

    compass += f"""

## All Documents

| Title | Category | Tags |
|-------|----------|------|
"""

    for category in sorted(by_category.keys()):
        for doc in by_category[category]:
            tags = ', '.join(doc['tags'][:2])
            compass += f"| [{doc['title']}](./docs/{doc['filename']}) | {category} | {tags} |\n"

    return compass

# ============================================================================
# STEP 7: VALIDATE
# ============================================================================

@dataclass
class ValidationResult:
    rule: str
    severity: str
    passed: bool
    message: str

def validate_file(filename: str, content: str) -> List[ValidationResult]:
    """Validate transformed document"""
    results = []

    # V001: single_h1
    h1_count = len(re.findall(r'^# [^#]', content, re.MULTILINE))
    results.append(ValidationResult(
        rule='V001',
        severity='error',
        passed=h1_count == 1,
        message=f"Heading H1 count: {h1_count} (expected 1)"
    ))

    # V002: frontmatter_exists
    has_fm = content.startswith('---')
    results.append(ValidationResult(
        rule='V002',
        severity='error',
        passed=has_fm,
        message="Frontmatter present" if has_fm else "No frontmatter"
    ))

    # V003: required_fields
    required = {'id:', 'title:', 'category:', 'tags:'}
    has_all = all(field in content[:500] for field in required)
    results.append(ValidationResult(
        rule='V003',
        severity='error',
        passed=has_all,
        message="All required frontmatter fields" if has_all else f"Missing fields"
    ))

    # V004: no_skipped_headings
    headings = re.findall(r'^(#{1,6})\s+', content, re.MULTILINE)
    levels = [len(h) for h in headings]
    valid_levels = all(levels[i+1] <= levels[i] + 1 for i in range(len(levels)-1)) if levels else True
    results.append(ValidationResult(
        rule='V004',
        severity='error',
        passed=valid_levels,
        message="No skipped heading levels" if valid_levels else "Invalid heading hierarchy"
    ))

    # V005: links_resolve - skip for now (would need manifest)

    # V006: min_tags
    tags_match = re.search(r'tags:\s*\[([^\]]+)\]', content)
    tag_count = len(tags_match.group(1).split(',')) if tags_match else 0
    results.append(ValidationResult(
        rule='V006',
        severity='warning',
        passed=tag_count >= 3,
        message=f"Tag count: {tag_count} (expected 3+)"
    ))

    # V007: has_context
    has_context = '> **Context**:' in content
    results.append(ValidationResult(
        rule='V007',
        severity='warning',
        passed=has_context,
        message="Context block present" if has_context else "No context block"
    ))

    # V008: has_see_also
    has_see_also = '## See Also' in content
    results.append(ValidationResult(
        rule='V008',
        severity='warning',
        passed=has_see_also,
        message="See Also section present" if has_see_also else "No See Also"
    ))

    return results

# ============================================================================
# MAIN ORCHESTRATION
# ============================================================================

def run_transformer(source_dir: str, output_dir: str) -> None:
    """Execute all transformation steps"""

    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True, parents=True)

    docs_dir = output_path / 'docs'
    chunks_dir = output_path / 'chunks'
    docs_dir.mkdir(exist_ok=True)
    chunks_dir.mkdir(exist_ok=True)

    manifest = {}

    print("\n" + "="*70)
    print("DOC_TRANSFORMER v4.2")
    print("="*70)

    # STEP 1: DISCOVER
    print("\n[STEP 1] DISCOVER")
    files, discover_manifest = discover_files(source_dir)
    print(f"  DISCOVER: Found {len(files)} files")
    manifest.update(discover_manifest)

    # STEP 2: ANALYZE
    print("\n[STEP 2] ANALYZE")
    analyses, categories = analyze_files(files, source_dir)
    print(f"  ANALYZE: Processed {len(analyses)} files")
    print(f"    Categories: ref={categories.get('ref', 0)} concept={categories.get('concept', 0)} " \
          f"tutorial={categories.get('tutorial', 0)} ops={categories.get('ops', 0)} meta={categories.get('meta', 0)}")

    # STEP 3: ASSIGN IDs
    print("\n[STEP 3] ASSIGN IDs")
    analyses, link_map = assign_ids(analyses)
    print(f"  ASSIGN: Generated {len(analyses)} IDs")

    # STEP 4: TRANSFORM
    print("\n[STEP 4] TRANSFORM")
    success_count = 0
    error_count = 0
    for analysis in analyses:
        mapping = link_map.get(analysis.source_path, {})
        doc_id = mapping.get('id', '')
        filename = mapping.get('filename', '')

        try:
            transformed, broken = transform_file(analysis, doc_id, link_map)

            # Write to output
            output_file = docs_dir / filename
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(transformed)

            success_count += 1
        except Exception as e:
            print(f"    TRANSFORM ERROR: {analysis.source_path}: {str(e)}")
            error_count += 1

    print(f"  TRANSFORM: {success_count}/{len(analyses)} files ({error_count} errors)")

    # STEP 5: CHUNK
    print("\n[STEP 5] CHUNK")
    chunks_manifest = {}
    total_chunks = 0

    for analysis in analyses:
        mapping = link_map.get(analysis.source_path, {})
        doc_id = mapping.get('id', '')

        try:
            chunks = create_chunks(doc_id, analysis.content)
            chunks_manifest[doc_id] = [c.chunk_id for c in chunks]

            for chunk in chunks:
                chunk_filename = chunk.chunk_id.replace('/', '-').replace('#', '-') + '.md'
                chunk_content = f"""---
doc_id: {chunk.doc_id}
chunk_id: {chunk.chunk_id}
chunk_type: {chunk.chunk_type}
tokens: {chunk.tokens}
summary: {chunk.summary}
---
{chunk.content}"""

                chunk_file = chunks_dir / chunk_filename
                with open(chunk_file, 'w', encoding='utf-8') as f:
                    f.write(chunk_content)

                total_chunks += 1
        except Exception as e:
            print(f"    CHUNK ERROR: {doc_id}: {str(e)}")

    print(f"  CHUNK: Generated {total_chunks} chunks from {len(analyses)} documents")

    # STEP 6: INDEX
    print("\n[STEP 6] INDEX")
    index = build_index(analyses, link_map, chunks_manifest)
    compass = build_compass(analyses, link_map)

    # Write INDEX.json
    index_file = output_path / 'INDEX.json'
    with open(index_file, 'w', encoding='utf-8') as f:
        json.dump(index, f, indent=2)

    # Write COMPASS.md
    compass_file = output_path / 'COMPASS.md'
    with open(compass_file, 'w', encoding='utf-8') as f:
        f.write(compass)

    print(f"  INDEX: Created COMPASS.md and INDEX.json")

    # STEP 7: VALIDATE
    print("\n[STEP 7] VALIDATE")
    validation_results = {}
    files_passed = 0
    total_errors = 0
    total_warnings = 0

    for doc_file in docs_dir.glob('*.md'):
        with open(doc_file, 'r', encoding='utf-8') as f:
            content = f.read()

        results = validate_file(doc_file.name, content)
        validation_results[doc_file.name] = results

        errors = [r for r in results if r.severity == 'error' and not r.passed]
        warnings = [r for r in results if r.severity == 'warning' and not r.passed]

        if not errors:
            files_passed += 1

        total_errors += len(errors)
        total_warnings += len(warnings)

    # Write validation report
    validation_report = {
        'run_at': datetime.now().isoformat(),
        'summary': {
            'files_checked': len(validation_results),
            'files_passed': files_passed,
            'files_failed': len(validation_results) - files_passed,
            'total_errors': total_errors,
            'total_warnings': total_warnings
        },
        'details': {fname: [asdict(r) for r in results]
                   for fname, results in validation_results.items()}
    }

    report_file = output_path / 'validation_report.json'
    with open(report_file, 'w', encoding='utf-8') as f:
        json.dump(validation_report, f, indent=2)

    print(f"  VALIDATE: {files_passed}/{len(validation_results)} files passed. " \
          f"{total_errors} errors {total_warnings} warnings")

    # FINAL SUMMARY
    print("\n" + "="*70)
    print("COMPLETE")
    print("="*70)
    print(f"Source:     {source_dir}")
    print(f"Output:     {output_dir}")
    print(f"Documents:  {len(analyses)} transformed")
    print(f"Chunks:     {total_chunks} generated")
    print(f"Validation: {files_passed}/{len(validation_results)} passed")
    print(f"Errors:     {total_errors}")
    print(f"Warnings:   {total_warnings}")
    print("="*70 + "\n")

if __name__ == '__main__':
    import sys

    if len(sys.argv) < 3:
        print("Usage: python index_cue_docs.py <source_dir> <output_dir>")
        print("Example: python index_cue_docs.py ./cue_docs ./docs/indexed/cue")
        sys.exit(1)

    source_dir = sys.argv[1]
    output_dir = sys.argv[2]

    run_transformer(source_dir, output_dir)
