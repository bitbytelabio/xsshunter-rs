CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key TEXT UNIQUE,
    value TEXT
);

CREATE INDEX settings_key_btree ON settings USING btree (key);

CREATE TABLE payload_fire_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    url TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    referer TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    cookies TEXT NOT NULL,
    title TEXT NOT NULL,
    dom TEXT NOT NULL,
    text TEXT NOT NULL,
    origin TEXT NOT NULL,
    screenshot_id TEXT,
    was_iframe BOOLEAN NOT NULL,
    browser_timestamp BIGINT NOT NULL
);

CREATE INDEX payload_fire_results_url_btree ON payload_fire_results USING btree (url);
CREATE INDEX payload_fire_results_ip_address_btree ON payload_fire_results USING btree (ip_address);
CREATE INDEX payload_fire_results_referer_btree ON payload_fire_results USING btree (referer);
CREATE INDEX payload_fire_results_user_agent_btree ON payload_fire_results USING btree (user_agent);
CREATE INDEX payload_fire_results_cookies_btree ON payload_fire_results USING btree (cookies);
CREATE INDEX payload_fire_results_title_btree ON payload_fire_results USING btree (title);
CREATE INDEX payload_fire_results_origin_btree ON payload_fire_results USING btree (origin);
CREATE INDEX payload_fire_results_was_iframe_btree ON payload_fire_results USING btree (was_iframe);
CREATE INDEX payload_fire_results_browser_timestamp_btree ON payload_fire_results USING btree (browser_timestamp);

CREATE TABLE collected_pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    uri TEXT NOT NULL,
    html TEXT
);

CREATE INDEX collected_pages_uri_btree ON collected_pages USING btree (uri);

CREATE TABLE injection_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request TEXT NOT NULL,
    injection_key TEXT NOT NULL
);

CREATE UNIQUE INDEX injection_requests_injection_key_btree ON injection_requests USING btree (injection_key);