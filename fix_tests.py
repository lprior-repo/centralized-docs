import os
import re

tests_dir = 'doc_transformer/tests'

for root, _, files in os.walk(tests_dir):
    for file in files:
        if not file.endswith('.rs'):
            continue
            
        path = os.path.join(root, file)
        with open(path, 'r') as f:
            content = f.read()
            
        original_content = content
        
        # Fix discover_files
        content = re.sub(r'discover_files\((.*?)\)', r'discover_files(\1, None)', content)
        # Revert if it was already correct or got doubled
        content = content.replace(', None, None)', ', None)')
        
        # Fix IndexDocument (missing content)
        # We need to find IndexDocument { ... } and add content: String::new(), if missing
        # We'll look for word_count: and prepend content: String::new(),
        # using the same indentation as word_count
        
        def replace_word_count(match):
            indent = match.group(1)
            return f"{indent}content: String::new(),\n{indent}word_count:"
            
        if 'IndexDocument' in content and 'content:' not in content:
            content = re.sub(r'([ \t]+)word_count:', replace_word_count, content)
            
        if content != original_content:
            with open(path, 'w') as f:
                f.write(content)
            print(f"Fixed {path}")

