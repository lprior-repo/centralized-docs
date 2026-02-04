# Spider-rs Options (v2.39.21)

Reference sheet for every spider-rs option currently available in the pinned version (`2.39.21`) and how we can leverage them inside this repo. Source: `/cache/cargo-shared/registry/src/index.crates.io-1949cf8c6b5b557f/spider-2.39.21/`.

## Feature Flags

- default: `basic` + `io_uring`
- basic: `__basic` + `basic_tls`
- __basic: `sync`, `cookies`, `ua_generator`, `encoding`, `string_interner_buffer_backend`, `balance`, `real_browser`, `disk_native_tls`, `time`
- crawling/core: `sitemap`, `regex`, `glob`, `tracing`, `full_resources`, `shared_queue`, `control`, `depth` (implied via core config)
- HTTP/TLS: `reqwest_json`, `reqwest_multipart`, `reqwest_native_tls*`, `reqwest_rustls_tls`, `reqwest_hickory_dns`, `socks`, `headers`, `remote_addr`, `http2_prior_knowledge` (core field), `encoding`
- caching: `cache`, `cache_mem`, `cache_request`, `cache_chrome_hybrid`, `cache_chrome_hybrid_mem`, `cache_openai`, `cache_gemini`, `cache_remote_cache*`
- Chrome/JS: `chrome`, `chrome_headless_new`, `chrome_headed`, `chrome_cpu`, `chrome_intercept`, `chrome_screenshot`, `chrome_tls_connection`, `chrome_store_page`, `chrome_remote_cache`, `chrome_remote_cache_disk`, `chrome_remote_cache_mem`, `chrome_simd`, `chrome_stealth`, `chrome_serde_stacker`, `chrome_screenshot`, `adblock`, `smart`, `cmd`
- Anti-detection: `ua_generator`, `spoof`, `headers`, `cookies`, `fingerprint` (via chrome), `modify_headers`, `modify_http_client_headers`
- AI copilots: `openai`, `openai_slim_fit`, `gemini` (drive Chrome), `decentralized`, `decentralized_headers`, `firewall`
- Scheduling/ops: `cron`, `cowboy`, `disk`, `disk_aws`, `disk_native_tls`, `fs`, `tokio_io_std`, `wreq`, `tokio_io_std`, `inline-more`, `balance`, `simd`, `page_error_status_details`

## Configuration Fields (spider::configuration::Configuration)

- Crawl bounds: `respect_robots_txt`, `subdomains`, `tld`, `depth`, `depth_distance`, `crawl_timeout`, `concurrency_limit`, `shared_queue`, `shared`, `normalize`, `only_html`, `full_resources`, `return_page_links`, `no_control_thread`, `cron_str`/`cron_type` (feature: cron)
- URL filtering: `whitelist_url`, `blacklist_url`, `configure_allowlist()`, `allowlist/blacklist` inner sets (regex-capable when feature `regex` is on), `budget`, `wild_card_budgeting`, `external_domains_caseless`
- Networking: `delay` (ms), `request_timeout`, `http2_prior_knowledge`, `proxies` (`RequestProxy` with `ProxyIgnore`), `headers` (`SerializableHeaderMap`), `referer`, `network_interface`, `local_address`, `default_http_connect_timeout`, `default_http_read_timeout`
- Redirects: `redirect_limit`, `redirect_policy` (`Loose`, `Strict`, `None`)
- Robots/sitemap: `sitemap_url`, `ignore_sitemap` (feature: sitemap)
- Identity/stealth: `user_agent`, `stealth_mode` tier, `modify_headers`, `modify_http_client_headers`, `preserve_host_header`, `accept_invalid_certs`, `fingerprint` (chrome), `bypass_csp` (chrome)
- Retries/timeouts: `retry` (u8), `max_page_bytes`, `max_bytes_allowed`, `cache_policy`
- Content capture: `viewport`, `screenshot`, `capture_*`, `track_events`, `execution_scripts`, `automation_scripts`, `wait_for*`, `dismiss_dialogs`, `service_worker_enabled`, `auto_geolocation` (all gated on chrome variants)
- Auth/session: `cookie_str` (feature: cookies), `auth_challenge_response` (chrome_intercept), `proxies`, `emulation` (feature: wreq), `openai_config` (feature: openai), `gemini_config` (feature: gemini)
- Resources/storage: `cache` (cache_request/cache_* features), `cache_policy`, `shared` (disk feature), `shared_queue`, `budget`/`inner_budget`, `max_page_bytes`, `max_bytes_allowed`

## How We Map These Today

- Enabled features in `doc_transformer`: `sitemap`, `sync` (others are defined but off to avoid 2.39 build bugs). See `doc_transformer/Cargo.toml` for tiered presets.
- Actively set fields in `scrape.rs`: `delay`, `respect_robots_txt`, `user_agent`, `concurrency_limit=1`, `modify_headers` (stealth), `retry`, `request_timeout`, `with_limit(max_pages)`, `normalize`, `with_whitelist_url` + `configure_allowlist` (path filter), plus manual caps on content/page counts.
- Placeholders reserved (not yet wired due to upstream issues): enhanced filtering allow/deny lists, JavaScript/Chrome options, anti-detection extras, caching knobs.

### CLI Exposure (doc_transformer)

- `--redirect-policy` (loose|strict|none) → `configuration.redirect_policy`
- `--request-timeout-secs` → `configuration.request_timeout`
- `--max-retries` → `configuration.retry` (also drives our backoff)
- `--max-page-bytes` → `configuration.max_page_bytes`
- `--max-total-bytes` → `configuration.max_bytes_allowed`
- `--concurrency` (capped 1-2) → `configuration.concurrency_limit`

## Next Steps (if we want full leverage)

- Decide which optional feature tiers to turn on once spider-rs fixes the 2.39 compilation bugs (regex/glob for native URL filtering, cache_* for HTTP caching, chrome/smart for JS rendering, ua_generator/spoof/headers/cookies for anti-detection).
- Expose configuration surface in our CLI for: `redirect_policy`, `proxies`, `headers`, `max_page_bytes`/`max_bytes_allowed`, `shared_queue` vs `no_control_thread`, and chrome automation hooks where available.
- Add regression tests around `configure_allowlist`, `retry`, and `respect_robots_txt` when enabling new flags to catch spider-rs breaking changes.
