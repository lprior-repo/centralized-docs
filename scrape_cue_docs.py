#!/usr/bin/env python3
"""
CUE Documentation Scraper and Validator
Scrapes all CUE documentation pages and validates them
"""

import asyncio
import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional
from urllib.parse import urlparse
import html2text
from playwright.async_api import async_playwright
from bs4 import BeautifulSoup
import logging

# Setup logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# All CUE documentation URLs
URLS = [
    # Introduction
    "https://cuelang.org/docs/introduction/",
    "https://cuelang.org/docs/introduction/installation/",

    # Tour
    "https://cuelang.org/docs/tour/",
    "https://cuelang.org/docs/tour/basics/",
    "https://cuelang.org/docs/tour/types/",
    "https://cuelang.org/docs/tour/references/",
    "https://cuelang.org/docs/tour/expressions/",
    "https://cuelang.org/docs/tour/packages/",

    # Tutorials
    "https://cuelang.org/docs/tutorial/",
    "https://cuelang.org/docs/tutorial/new-to-cue/",
    "https://cuelang.org/docs/tutorial/get-started-cue-java/",
    "https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/",

    # How-to Guides
    "https://cuelang.org/docs/howto/",
    "https://cuelang.org/docs/howto/about-commented-cue-guides/",
    "https://cuelang.org/docs/howto/search-this-site/",
    "https://cuelang.org/docs/howto/popular-guides/",

    # Concept Guides
    "https://cuelang.org/docs/concept/",
    "https://cuelang.org/docs/concept/popular-guides/",
    "https://cuelang.org/docs/concept/the-logic-of-cue/",
    "https://cuelang.org/docs/concept/modules/",
    "https://cuelang.org/docs/concept/faq/",
    "https://cuelang.org/docs/concept/how-cue-works-with-json/",
    "https://cuelang.org/docs/concept/how-cue-works-with-yaml/",
    "https://cuelang.org/docs/concept/how-cue-works-with-go/",
    "https://cuelang.org/docs/concept/how-cue-works-with-toml/",
    "https://cuelang.org/docs/concept/how-cue-works-with-openapi/",
    "https://cuelang.org/docs/concept/how-cue-works-with-protocol-buffers/",
    "https://cuelang.org/docs/concept/how-cue-works-with-json-schema/",

    # Reference
    "https://cuelang.org/docs/reference/",
    "https://cuelang.org/docs/reference/spec/",
    "https://cuelang.org/docs/reference/command/",
    "https://cuelang.org/docs/reference/modules/",
    "https://cuelang.org/docs/reference/glossary/",
    "https://cuelang.org/docs/reference/code-of-conduct/",

    # Integration
    "https://cuelang.org/docs/integration/",

    # Other
    "https://cuelang.org/",
    "https://cuelang.org/community/",
]

class CUEDocScraper:
    def __init__(self, output_dir: str = "cue_docs"):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        self.h = html2text.HTML2Text()
        self.h.body_width = 0  # Don't wrap lines
        self.h.ignore_links = False
        self.results = {
            "scrape_time": datetime.now().isoformat(),
            "total_urls": len(URLS),
            "successful": [],
            "failed": [],
            "validation_results": {}
        }

    async def scrape_page(self, url: str, browser) -> Optional[dict]:
        """Scrape a single page using Playwright"""
        try:
            page = await browser.new_page()
            logger.info(f"Scraping: {url}")

            await page.goto(url, wait_until="networkidle")

            # Get HTML content
            html_content = await page.content()

            # Convert to Markdown
            markdown = self.h.handle(html_content)

            # Extract metadata
            title = await page.title()

            await page.close()

            return {
                "url": url,
                "title": title,
                "html": html_content,
                "markdown": markdown,
                "status": "success"
            }
        except Exception as e:
            logger.error(f"Failed to scrape {url}: {str(e)}")
            await page.close() if 'page' in locals() else None
            return {
                "url": url,
                "error": str(e),
                "status": "failed"
            }

    def validate_markdown(self, url: str, markdown: str) -> dict:
        """Validate markdown content"""
        validation = {
            "url": url,
            "checks": {}
        }

        # Check 1: Content is not empty
        validation["checks"]["not_empty"] = len(markdown.strip()) > 0

        # Check 2: Has headings
        has_headings = markdown.count('\n#') > 0
        validation["checks"]["has_headings"] = has_headings

        # Check 3: Has content
        lines = markdown.split('\n')
        has_content = len([l for l in lines if l.strip() and not l.startswith('#')]) > 0
        validation["checks"]["has_content"] = has_content

        # Check 4: Markdown is valid (basic checks)
        valid_markdown = True
        if markdown.count('```') % 2 != 0:
            valid_markdown = False  # Unmatched code blocks
        validation["checks"]["valid_markdown"] = valid_markdown

        # Check 5: No excessive whitespace
        validation["checks"]["no_excessive_whitespace"] = '\n\n\n\n' not in markdown

        # Overall validation
        validation["is_valid"] = all(validation["checks"].values())

        return validation

    def save_markdown(self, url: str, markdown: str, title: str):
        """Save markdown to file"""
        # Create safe filename from URL
        parsed_url = urlparse(url)
        path_parts = parsed_url.path.strip('/').split('/')
        filename = '_'.join(path_parts) + '.md'

        file_path = self.output_dir / filename

        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(f"# {title}\n\n")
            f.write(f"**Source:** {url}\n\n")
            f.write(markdown)

        logger.info(f"Saved: {file_path}")
        return file_path

    async def scrape_all(self):
        """Scrape all URLs"""
        async with async_playwright() as p:
            browser = await p.chromium.launch(headless=True)

            for url in URLS:
                result = await self.scrape_page(url, browser)

                if result["status"] == "success":
                    self.results["successful"].append(url)

                    # Save markdown
                    self.save_markdown(url, result["markdown"], result["title"])

                    # Validate
                    validation = self.validate_markdown(url, result["markdown"])
                    self.results["validation_results"][url] = validation
                else:
                    self.results["failed"].append(url)
                    self.results["validation_results"][url] = {
                        "url": url,
                        "is_valid": False,
                        "error": result.get("error")
                    }

            await browser.close()

    def print_report(self):
        """Print scraping and validation report"""
        print("\n" + "="*80)
        print("CUE DOCUMENTATION SCRAPER REPORT")
        print("="*80)
        print(f"Scrape Time: {self.results['scrape_time']}")
        print(f"Total URLs: {self.results['total_urls']}")
        print(f"Successful: {len(self.results['successful'])}")
        print(f"Failed: {len(self.results['failed'])}")

        print("\n" + "-"*80)
        print("VALIDATION RESULTS")
        print("-"*80)

        valid_count = 0
        invalid_count = 0

        for url, validation in self.results["validation_results"].items():
            is_valid = validation.get("is_valid", False)
            status = "✓ VALID" if is_valid else "✗ INVALID"

            if is_valid:
                valid_count += 1
            else:
                invalid_count += 1

            print(f"\n{status}: {url}")

            if "checks" in validation:
                for check, result in validation["checks"].items():
                    check_status = "✓" if result else "✗"
                    print(f"  {check_status} {check}: {result}")

            if "error" in validation:
                print(f"  Error: {validation['error']}")

        print("\n" + "-"*80)
        print(f"Valid Documents: {valid_count}")
        print(f"Invalid Documents: {invalid_count}")
        print("-"*80 + "\n")

        return valid_count, invalid_count

    def save_report(self):
        """Save JSON report"""
        report_path = self.output_dir / "report.json"
        with open(report_path, 'w', encoding='utf-8') as f:
            json.dump(self.results, f, indent=2)
        logger.info(f"Report saved to: {report_path}")
        return report_path

async def main():
    scraper = CUEDocScraper(output_dir="cue_docs")

    print("Starting CUE Documentation Scraper...")
    print(f"Output directory: {scraper.output_dir.absolute()}")
    print(f"URLs to scrape: {len(URLS)}\n")

    await scraper.scrape_all()

    valid, invalid = scraper.print_report()
    report_path = scraper.save_report()

    if invalid > 0:
        print(f"⚠️  {invalid} documents failed validation!")
        sys.exit(1)
    else:
        print("✓ All documents are valid!")
        sys.exit(0)

if __name__ == "__main__":
    asyncio.run(main())
