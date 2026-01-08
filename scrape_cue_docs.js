#!/usr/bin/env node

const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');
const { convert } = require('html-to-text');
const { URL } = require('url');

// All CUE documentation URLs
const URLS = [
  // Introduction
  "https://cuelang.org/docs/introduction/",
  "https://cuelang.org/docs/introduction/installation/",

  // Tour
  "https://cuelang.org/docs/tour/",
  "https://cuelang.org/docs/tour/basics/",
  "https://cuelang.org/docs/tour/types/",
  "https://cuelang.org/docs/tour/references/",
  "https://cuelang.org/docs/tour/expressions/",
  "https://cuelang.org/docs/tour/packages/",

  // Tutorials
  "https://cuelang.org/docs/tutorial/",
  "https://cuelang.org/docs/tutorial/new-to-cue/",
  "https://cuelang.org/docs/tutorial/get-started-cue-java/",
  "https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/",

  // How-to Guides
  "https://cuelang.org/docs/howto/",
  "https://cuelang.org/docs/howto/about-commented-cue-guides/",
  "https://cuelang.org/docs/howto/search-this-site/",
  "https://cuelang.org/docs/howto/popular-guides/",

  // Concept Guides
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

  // Reference
  "https://cuelang.org/docs/reference/",
  "https://cuelang.org/docs/reference/spec/",
  "https://cuelang.org/docs/reference/command/",
  "https://cuelang.org/docs/reference/modules/",
  "https://cuelang.org/docs/reference/glossary/",
  "https://cuelang.org/docs/reference/code-of-conduct/",

  // Integration
  "https://cuelang.org/docs/integration/",

  // Other
  "https://cuelang.org/",
  "https://cuelang.org/community/",
];

class CUEDocScraper {
  constructor(outputDir = 'cue_docs') {
    this.outputDir = outputDir;
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }
    this.results = {
      scrape_time: new Date().toISOString(),
      total_urls: URLS.length,
      successful: [],
      failed: [],
      validation_results: {}
    };
  }

  async scrapePage(url, page) {
    try {
      console.log(`[Scraping] ${url}`);
      await page.goto(url, { waitUntil: 'networkidle2', timeout: 30000 });

      // Get title
      const title = await page.title();

      // Get HTML content
      const htmlContent = await page.content();

      // Convert to markdown
      const markdown = convert(htmlContent, {
        wordwrap: false,
        preserveNewlines: true,
        selectors: [
          { selector: 'a', options: { ignoreHref: false } },
          { selector: 'img', options: { ignoreAltText: false } }
        ]
      });

      return {
        url,
        title,
        html: htmlContent,
        markdown,
        status: 'success'
      };
    } catch (error) {
      console.error(`[Error] Failed to scrape ${url}: ${error.message}`);
      return {
        url,
        error: error.message,
        status: 'failed'
      };
    }
  }

  validateMarkdown(url, markdown) {
    const validation = {
      url,
      checks: {}
    };

    // Check 1: Content is not empty
    validation.checks.not_empty = markdown.trim().length > 0;

    // Check 2: Has actual content (text beyond just whitespace)
    const lines = markdown.split('\n');
    const contentLines = lines.filter(l => l.trim().length > 0);
    validation.checks.has_content = contentLines.length > 5;

    // Check 3: Markdown is valid (basic checks - matching backticks)
    const codeBlockCount = (markdown.match(/```/g) || []).length;
    validation.checks.valid_markdown = codeBlockCount % 2 === 0;

    // Check 4: Not just navigation/boilerplate - should have substantive content
    validation.checks.substantive_content = markdown.length > 500;

    // Overall validation - all checks must pass
    validation.is_valid = Object.values(validation.checks).every(v => v === true);

    return validation;
  }

  saveMarkdown(url, markdown, title) {
    try {
      const urlObj = new URL(url);
      const pathParts = urlObj.pathname.split('/').filter(p => p);
      const filename = pathParts.join('_') + '.md';
      const filePath = path.join(this.outputDir, filename);

      // Clean up excessive whitespace (max 2 consecutive newlines)
      const cleanedMarkdown = markdown
        .replace(/\n{4,}/g, '\n\n')  // Replace 4+ newlines with 2
        .trim();

      const content = `# ${title}\n\n**Source:** ${url}\n\n${cleanedMarkdown}`;
      fs.writeFileSync(filePath, content, 'utf-8');
      console.log(`[Saved] ${filePath}`);
      return filePath;
    } catch (error) {
      console.error(`[Error] Failed to save ${url}: ${error.message}`);
      return null;
    }
  }

  async scrapeAll(browser) {
    for (const url of URLS) {
      const page = await browser.newPage();
      page.setDefaultTimeout(30000);

      try {
        const result = await this.scrapePage(url, page);

        if (result.status === 'success') {
          this.results.successful.push(url);

          // Save markdown
          this.saveMarkdown(url, result.markdown, result.title);

          // Validate
          const validation = this.validateMarkdown(url, result.markdown);
          this.results.validation_results[url] = validation;
        } else {
          this.results.failed.push(url);
          this.results.validation_results[url] = {
            url,
            is_valid: false,
            error: result.error
          };
        }
      } finally {
        await page.close();
      }

      // Add delay to be respectful
      await new Promise(resolve => setTimeout(resolve, 500));
    }
  }

  printReport() {
    console.log('\n' + '='.repeat(80));
    console.log('CUE DOCUMENTATION SCRAPER REPORT');
    console.log('='.repeat(80));
    console.log(`Scrape Time: ${this.results.scrape_time}`);
    console.log(`Total URLs: ${this.results.total_urls}`);
    console.log(`Successful: ${this.results.successful.length}`);
    console.log(`Failed: ${this.results.failed.length}`);

    console.log('\n' + '-'.repeat(80));
    console.log('VALIDATION RESULTS');
    console.log('-'.repeat(80));

    let validCount = 0;
    let invalidCount = 0;

    for (const [url, validation] of Object.entries(this.results.validation_results)) {
      const isValid = validation.is_valid ?? false;
      const status = isValid ? '✓ VALID' : '✗ INVALID';

      if (isValid) validCount++;
      else invalidCount++;

      console.log(`\n${status}: ${url}`);

      if (validation.checks) {
        for (const [check, result] of Object.entries(validation.checks)) {
          const checkStatus = result ? '✓' : '✗';
          console.log(`  ${checkStatus} ${check}: ${result}`);
        }
      }

      if (validation.error) {
        console.log(`  Error: ${validation.error}`);
      }
    }

    console.log('\n' + '-'.repeat(80));
    console.log(`Valid Documents: ${validCount}`);
    console.log(`Invalid Documents: ${invalidCount}`);
    console.log('-'.repeat(80) + '\n');

    return { validCount, invalidCount };
  }

  saveReport() {
    const reportPath = path.join(this.outputDir, 'report.json');
    fs.writeFileSync(reportPath, JSON.stringify(this.results, null, 2), 'utf-8');
    console.log(`[Report] Saved to: ${reportPath}`);
    return reportPath;
  }
}

async function main() {
  const scraper = new CUEDocScraper('cue_docs');

  console.log('Starting CUE Documentation Scraper...');
  console.log(`Output directory: ${path.resolve(scraper.outputDir)}`);
  console.log(`URLs to scrape: ${URLS.length}\n`);

  const browser = await puppeteer.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  try {
    await scraper.scrapeAll(browser);
    const { validCount, invalidCount } = scraper.printReport();
    scraper.saveReport();

    if (invalidCount > 0) {
      console.log(`⚠️  ${invalidCount} documents failed validation!`);
      process.exit(1);
    } else {
      console.log('✓ All documents are valid!');
      process.exit(0);
    }
  } finally {
    await browser.close();
  }
}

main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});
