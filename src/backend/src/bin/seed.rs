//! Database seeder for development — populates Postgres with realistic fake data.
//!
//! Uses `rand_seeder` for deterministic generation (same data on every run)
//! and `fake` for realistic text. All IDs, scores, and timestamps are
//! reproducible from the seed string.
//!
//! # Usage
//!
//! ```bash
//! cargo run --bin seed
//! ```
//!
//! Requires `DATABASE_URL` in env or `.env` file.
//! Idempotent — skips seeding if the demo user already exists.

use scoutnet::application::auth::password::hash_password;
use chrono::{Duration, Utc};
use fake::Fake;
use rand::{Rng, SeedableRng, rngs::StdRng};
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use uuid::Uuid;

/// Demo user credentials: dev@scoutnet.io / password
const DEMO_EMAIL: &str = "dev@scoutnet.io";
const DEMO_PASSWORD: &str = "password";

/// Stub channel data: (title, source_type, url).
const CHANNELS: &[(&str, &str, &str)] = &[
    ("@FL_Hunter", "telegram", "https://t.me/fl_hunter"),
    ("@VuejsJobs", "telegram", "https://t.me/vuejs_jobs"),
    ("@Dev_Market", "telegram", "https://t.me/dev_market"),
    ("@Stack_Jobs", "telegram", "https://t.me/stack_jobs"),
    ("@gDev_Digest", "telegram", "https://t.me/gdev_digest"),
    ("@Outsors_Chat", "telegram", "https://t.me/outsors_chat"),
    ("@Startup_Jobs", "telegram", "https://t.me/startup_jobs"),
    ("@gFL_Network", "telegram", "https://t.me/gfl_network"),
    ("@FullStack_WS", "telegram", "https://t.me/fullstack_ws"),
    ("@Next_Orders", "telegram", "https://t.me/next_orders"),
    ("@gDev_Telega", "telegram", "https://t.me/gdev_telega"),
    ("@Lead_Hunter", "telegram", "https://t.me/lead_hunter"),
    ("@TG_Freelance", "telegram", "https://t.me/tg_freelance"),
    ("@Crypto_Dev", "telegram", "https://t.me/crypto_dev"),
    ("@OpenSea_WS", "telegram", "https://t.me/opensea_ws"),
    ("@Team_Leads", "telegram", "https://t.me/team_leads"),
    ("@Upwork_Feed", "web", "https://upwork.com/feed"),
    ("@Kadrop_Bot", "web", "https://kadrop.io/jobs"),
];

/// Realistic Russian freelance job descriptions for seed messages.
const JOB_TEXTS: &[&str] = &[
    "Вёрстка многостраничного сайта на Tilda. 8 страниц, блог, портфолио, SEO.",
    "Быстрая задача: обновить баннеры для маркетплейса. Ozon + WB, 20 карточек.",
    "Дизайн интернет-магазина на Tilda. 15 страниц, каталог, корзина, мобильная версия.",
    "Отрисовка иконок для приложения. 40 штук, line-стиль, SVG. Фирменные цвета дадим.",
    "Нужен веб-дизайнер для лендинга онлайн-школы. Figma, дрейфер, до 10 экранов.",
    "Vue 3 + TypeScript SPA — senior dev для миграции с Nuxt 2. Удалённо, долгосрок.",
    "React Native приложение — MVP за 3 недели. iOS + Android, API готово.",
    "Телеграм-бот для записи на услуги. Интеграция с Google Calendar и CRM.",
    "Fullstack разработчик Node.js + React. E-commerce проект, 3 месяца, удалённо.",
    "Парсинг и агрегация данных с 50+ сайтов. Python, Scrapy, PostgreSQL.",
    "Мобильное приложение для доставки еды. Flutter, Firebase, Stripe.",
    "Разработка CRM-системы. Laravel + Vue.js, REST API, 2 месяца.",
    "Автоматизация тестирования. Playwright + TypeScript, CI/CD, 15 сценариев.",
    "Дизайн системы для SaaS продукта. Figma, токены, компоненты, документация.",
    "Backend на Rust — высоконагруженный сервис обработки платежей. Axum, sqlx.",
    "Верстка email-рассылок. 10 шаблонов, адаптивные, Mailchimp-совместимые.",
    "Разработка Telegram Mini App. React, TON Connect, кошелёк, NFT витрина.",
    "SEO-оптимизация интернет-магазина. Аудит, мета-теги, schema.org, 200 страниц.",
    "3D визуализация интерьера. Blender, 5 комнат, фотореализм.",
    "Чат-бот для поддержки клиентов. OpenAI API, RAG, база знаний компании.",
];

/// Budget ranges in RUB for realistic price variation.
const BUDGETS: &[(i32, i32)] = &[
    (15_000, 25_000),
    (30_000, 50_000),
    (50_000, 80_000),
    (70_000, 120_000),
    (100_000, 200_000),
    (150_000, 300_000),
];

/// Skills pool for random assignment.
const SKILLS: &[&str] = &[
    "vue", "nuxt", "react", "typescript", "figma", "tailwind",
    "rust", "python", "node", "flutter", "laravel", "blender",
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("🌱 Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Idempotent check — skip if demo user exists
    let existing = db
        .fetch_optional(sqlx::query("SELECT id FROM users WHERE email = $1").bind(DEMO_EMAIL))
        .await?;

    if existing.is_some() {
        println!("✅ Demo user already exists — skipping seed.");
        return Ok(());
    }

    // Deterministic RNG — same seed = same data on every run
    let mut rng = StdRng::seed_from_u64(0x5C0DF_2026);

    // ── 1. Create demo user ────────────────────────────────────
    println!("👤 Creating demo user: {DEMO_EMAIL}");
    let password_hash = hash_password(DEMO_PASSWORD)
        .map_err(|e| anyhow::anyhow!("password hash: {e}"))?;
    let user_id = det_uuid(&mut rng);

    db.execute(
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, tier) VALUES ($1, $2, $3, 'pro'::tier)",
        )
        .bind(user_id)
        .bind(DEMO_EMAIL)
        .bind(&password_hash),
    )
    .await?;

    // ── 2. Create user profile ─────────────────────────────────
    println!("📋 Creating user profile...");
    db.execute(
        sqlx::query(
            "INSERT INTO user_profiles (id, user_id, bio, skills, min_budget, score_threshold)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind("Fullstack developer, Vue.js + Rust. 5 лет опыта, фриланс.")
        .bind(&["vue", "nuxt", "typescript", "rust", "figma", "tailwind"] as &[&str])
        .bind(20_000_i32)
        .bind(0.5_f32),
    )
    .await?;

    // ── 3. Insert channels ─────────────────────────────────────
    println!("📡 Inserting {} channels...", CHANNELS.len());
    let mut channel_ids: Vec<(Uuid, &str, &str)> = Vec::new();
    for (title, source, url) in CHANNELS {
        let ch_id = Uuid::new_v4();
        db.execute(
            sqlx::query(
                "INSERT INTO channels (id, source, url, title, is_active)
                 VALUES ($1, $2::source_type, $3, $4, true)",
            )
            .bind(ch_id)
            .bind(*source)
            .bind(*url)
            .bind(*title),
        )
        .await?;
        channel_ids.push((ch_id, title, source));
    }

    // ── 4. Insert messages → leads → matches ───────────────────
    let num = JOB_TEXTS.len();
    println!("📨 Inserting {num} messages → leads → matches...");

    let now = Utc::now();
    for (i, text) in JOB_TEXTS.iter().enumerate() {
        let (_, ch_title, ch_source) = &channel_ids[i % channel_ids.len()];
        let posted_at = now - Duration::minutes((i as i64) * 8 + det_range(&mut rng, 1, 30));

        // Message
        let msg_id = Uuid::new_v4();
        db.execute(
            sqlx::query(
                "INSERT INTO messages (id, source_id, source_type, channel, text, posted_at)
                 VALUES ($1, $2, $3::source_type, $4, $5, $6)",
            )
            .bind(msg_id)
            .bind(format!("seed-msg-{i}"))
            .bind(*ch_source)
            .bind(*ch_title)
            .bind(*text)
            .bind(posted_at),
        )
        .await?;

        // Lead — with budget and random skills
        let lead_id = Uuid::new_v4();
        let (b_min, b_max) = BUDGETS[det_usize(&mut rng, BUDGETS.len())];
        let budget = det_range_i32(&mut rng, b_min, b_max);
        let n_skills = det_usize(&mut rng, 3) + 1;
        let skills: Vec<&str> = (0..n_skills)
            .map(|_| SKILLS[det_usize(&mut rng, SKILLS.len())])
            .collect();

        db.execute(
            sqlx::query(
                "INSERT INTO leads (id, message_id, skills, budget_min, budget_max, currency)
                 VALUES ($1, $2, $3, $4, $5, 'RUB')",
            )
            .bind(lead_id)
            .bind(msg_id)
            .bind(&skills as &[&str])
            .bind(budget)
            .bind(budget + det_range_i32(&mut rng, 5_000, 30_000)),
        )
        .await?;

        // Match
        let score = (0.97 - (i as f32) * 0.032).max(0.35);
        db.execute(
            sqlx::query(
                "INSERT INTO matches (id, user_id, lead_id, score, reason, status)
                 VALUES ($1, $2, $3, $4, $5, 'pending'::match_status)",
            )
            .bind(Uuid::new_v4())
            .bind(user_id)
            .bind(lead_id)
            .bind(score)
            .bind(*text),
        )
        .await?;

        let preview: String = text.chars().take(40).collect();
        println!("  ✓ [{:.0}%] {ch_title}: {preview}", score * 100.0);
    }

    println!("\n🎉 Seed complete! {num} matches for {DEMO_EMAIL}");
    println!("   Login: {DEMO_EMAIL} / {DEMO_PASSWORD}");
    Ok(())
}

/// Deterministic UUID from StdRng.
fn det_uuid(rng: &mut StdRng) -> Uuid {
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    Uuid::from_bytes(bytes)
}

/// Deterministic i64 in [min, max).
fn det_range(rng: &mut StdRng, min: i64, max: i64) -> i64 {
    rng.gen_range(min..max)
}

/// Deterministic i32 in [min, max).
fn det_range_i32(rng: &mut StdRng, min: i32, max: i32) -> i32 {
    rng.gen_range(min..max)
}

/// Deterministic usize in [0, max).
fn det_usize(rng: &mut StdRng, max: usize) -> usize {
    rng.gen_range(0..max.max(1))
}
