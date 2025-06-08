CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TYPE e_db_internal_status AS ENUM('new', 'viewed', 'not_interested', 'rejected', 'interested', 'applied');
CREATE TYPE e_db_pay_type AS ENUM('hourly', 'fixed');
CREATE TYPE e_db_experience_level AS ENUM('expert', 'intermediate', 'entry_level');
CREATE TYPE e_db_time_estimate AS ENUM('less_than_one_month', 'one_to_three_months', 'more_than_six_months', 'three_to_six_months');
CREATE TYPE e_db_search_query AS ENUM(
  'react',
  'svelte',
  'angular',
  'vue',
  'astro',
  'nextjs',
  'nestjs',
  'python',
  'php',
  'rust',
  'javascript',
  'wordpress',
  'webflow',
  'etl',
  'data_processing',
  'data_science',
  'web_development',
  'programming',
  'software'
);
CREATE TABLE
  postings (
    id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
    uid BIGINT UNIQUE NOT NULL,
    url TEXT NOT NULL,
    search_query e_db_search_query NOT NULL,
    internal_status e_db_internal_status NOT NULL DEFAULT 'new',
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    skills TEXT[] NOT NULL,
    experience_level e_db_experience_level NOT NULL,
    time_estimate e_db_time_estimate DEFAULT NULL,
    pay_type e_db_pay_type NOT NULL,
    budget INTEGER,
    max_rate INTEGER,
    min_rate INTEGER,
    created_at TIMESTAMPTZ NOT NULL
  );
CREATE INDEX idx_postings_uid ON postings (uid);
CREATE INDEX idx_postings_internal_status ON postings (internal_status);
CREATE INDEX idx_postings_search_query ON postings (search_query);
CREATE INDEX idx_postings_pay_type ON postings (pay_type);
CREATE INDEX idx_postings_max_rate ON postings (max_rate);
CREATE INDEX idx_postings_created_at ON postings (created_at);
CREATE TABLE
  tool_results (
    id UUID PRIMARY KEY REFERENCES postings (id) ON DELETE CASCADE,
    term TEXT NOT NULL,
    stage TEXT NOT NULL,
    tech_field TEXT NOT NULL,
    sectors TEXT[] NOT NULL,
    conditions TEXT[] NOT NULL,
    miscs TEXT[] NOT NULL,
    languages TEXT[] NOT NULL,
    local_project TEXT NOT NULL,
    global_project TEXT NOT NULL,
    frontier_project TEXT NOT NULL,
    targeted_pitch TEXT NOT NULL
  );