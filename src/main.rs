use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chrono::NaiveDate;
use html_escape::encode_text;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::io;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug)]
struct Post {
    title: String,
    subtitle: Option<String>,
    date: String,
    slug: String,
    draft: bool,
    body: String,
}

#[tokio::main]
async fn main() {
    if std::env::args().any(|arg| arg == "--export") {
        if let Err(err) = export_site() {
            eprintln!("Export failed: {err}");
            std::process::exit(1);
        }
        println!("Exported site to ./public");
        return;
    }

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/healthz", get(health_handler))
        .route_service(
            "/favicon.ico",
            ServeFile::new("assets/favicons/favicon.ico"),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/posts/:slug", get(post_handler))
        .route("/posts/:slug/", get(post_handler))
        .route("/drafts/:slug", get(draft_handler))
        .route("/drafts/:slug/", get(draft_handler))
        .fallback(fallback_handler);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind address");

    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.expect("server failed");
}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(render_not_found()))
        .into_response()
}

fn export_site() -> io::Result<()> {
    let posts = load_all_posts();
    fs::create_dir_all("public/posts")?;
    fs::write("public/index.html", render_index(&posts))?;

    for post in posts {
        let html = render_post(&post, "Home");
        let slug = &post.slug;
        if post.draft {
            let slug_dir = format!("public/drafts/{slug}");
            fs::create_dir_all(&slug_dir)?;
            fs::write(format!("{slug_dir}/index.html"), &html)?;
            fs::write(format!("public/drafts/{slug}.html"), &html)?;
        } else {
            let slug_dir = format!("public/posts/{slug}");
            fs::create_dir_all(&slug_dir)?;
            fs::write(format!("{slug_dir}/index.html"), &html)?;
            fs::write(format!("public/posts/{slug}.html"), &html)?;
        }
    }

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    let posts = load_all_posts();
    Html(render_index(&posts)).into_response()
}

async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

async fn post_handler(
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match load_post_by_slug(&slug) {
        Some(post) if !post.draft => {
            let label = back_label_from_referer(headers.get(header::REFERER));
            Html(render_post(&post, &label)).into_response()
        }
        _ => (StatusCode::NOT_FOUND, Html(render_not_found()))
            .into_response(),
    }
}

async fn draft_handler(
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match load_post_by_slug(&slug) {
        Some(post) if post.draft => {
            let label = back_label_from_referer(headers.get(header::REFERER));
            Html(render_post(&post, &label)).into_response()
        }
        Some(post) if !post.draft => {
            let label = back_label_from_referer(headers.get(header::REFERER));
            Html(render_post(&post, &label)).into_response()
        }
        _ => (StatusCode::NOT_FOUND, Html(render_not_found()))
            .into_response(),
    }
}

fn load_all_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    let entries = match fs::read_dir("posts") {
        Ok(entries) => entries,
        Err(_) => return posts,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("example.md") {
            continue;
        }
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => continue,
        };
        if let Some(post) = parse_post(&contents) {
            if !post.draft {
                posts.push(post);
            }
        }
    }

    posts.sort_by(|a, b| {
        let a_date = NaiveDate::parse_from_str(&a.date, "%Y-%m-%d").ok();
        let b_date = NaiveDate::parse_from_str(&b.date, "%Y-%m-%d").ok();
        b_date
            .cmp(&a_date)
            .then_with(|| b.title.cmp(&a.title))
    });

    posts
}

fn load_post_by_slug(target_slug: &str) -> Option<Post> {
    let entries = fs::read_dir("posts").ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("example.md") {
            continue;
        }
        let contents = fs::read_to_string(&path).ok()?;
        let post = parse_post(&contents)?;
        if post.slug == target_slug {
            return Some(post);
        }
    }
    None
}

fn parse_post(contents: &str) -> Option<Post> {
    let mut title = None;
    let mut subtitle = None;
    let mut date = None;
    let mut raw_slug: Option<String> = None;
    let mut draft = false;
    let mut in_body = false;
    let mut body_lines = Vec::new();

    for line in contents.lines() {
        if !in_body {
            let trimmed = line.trim();
            if trimmed == "---" {
                in_body = true;
                continue;
            }
            if let Some(value) = line.strip_prefix("Title: ") {
                title = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = line.strip_prefix("Date: ") {
                date = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = line.strip_prefix("Subtitle: ") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    subtitle = Some(trimmed.to_string());
                }
                continue;
            }
            if let Some(value) = line.strip_prefix("Slug:") {
                raw_slug = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = line.strip_prefix("Draft:") {
                let normalized = value.trim().to_ascii_lowercase();
                draft = normalized == "true" || normalized == "yes";
                continue;
            }
        } else {
            body_lines.push(line);
        }
    }

    let title = title?;
    let date = date?;
    let slug_source = match raw_slug {
        Some(value) if !value.is_empty() && !value.contains('<') && !value.contains('>') => value,
        _ => title.clone(),
    };
    let slug = slugify_kebab(&slug_source);
    let body = body_lines.join("\n");

    Some(Post {
        title,
        subtitle,
        date,
        slug,
        draft,
        body,
    })
}

fn slugify_kebab(value: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn render_index(posts: &[Post]) -> String {
    let mut items_html = String::new();
    let mut prefetch_html = String::new();
    for post in posts {
        let title = encode_text(&post.title);
        let formatted_date = format_date_full(&post.date);
        let date = encode_text(&formatted_date);
        let slug = encode_text(&post.slug);
        prefetch_html.push_str(&format!(
            r#"<link rel="prefetch" href="/posts/{slug}" />"#
        ));
        items_html.push_str(&format!(
            r#"<li><a href="/posts/{slug}">{title}</a><span>{date}</span></li>"#
        ));
    }

    let list_html = if items_html.is_empty() {
        "<p>No posts yet.</p>".to_string()
    } else {
        format!(r#"<ul class="post-list">{items_html}</ul>"#)
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Fragments</title>
  <link rel="icon" href="/favicon.ico" sizes="any" />
  <link rel="icon" href="/assets/favicons/favicon.svg" type="image/svg+xml" />
  <link rel="apple-touch-icon" href="/assets/favicons/apple-touch-icon.png" />
  <link rel="manifest" href="/assets/favicons/site.webmanifest" />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=PT+Sans+Narrow:wght@400;700&display=swap" rel="stylesheet" />
  {prefetch_html}
  <style>
    :root {{
      color-scheme: light;
    }}
    body {{
      margin: 0;
      padding: 72px 20px 48px;
      font-family: "PT Sans Narrow", sans-serif;
      font-weight: 400;
      background: #ffffff;
      color: #141311;
    }}
    main {{
      max-width: 720px;
      margin: 0 auto 35vh;
    }}
    h1 {{
      font-family: "Instrument Serif", serif;
      font-size: 3rem;
      margin: 0 0 24px;
      letter-spacing: -0.01em;
      line-height: 1.3;
    }}
    .post-list {{
      list-style: none;
      padding: 0;
      margin: 0;
      display: grid;
      gap: 16px;
    }}
    .post-list li {{
      display: flex;
      flex-wrap: wrap;
      gap: 8px 16px;
      align-items: baseline;
      justify-content: space-between;
      padding: 12px 0;
      border-bottom: 1px solid #e7e5e4;
    }}
    .post-list a {{
      font-family: "Instrument Serif", serif;
      font-size: 1.5rem;
      color: inherit;
      text-decoration: none;
    }}
    .post-list span {{
      opacity: 0.65;
    }}
    @media (max-width: 640px) {{
      body {{
        padding: 32px 16px;
      }}
      h1 {{
        font-size: 2.4rem;
      }}
      .post-list a {{
        font-size: 1.3rem;
      }}
    }}
  </style>
</head>
<body>
  <main>
    <h1>Fragments</h1>
    {list_html}
  </main>
</body>
</html>"#
    )
}

fn render_post(post: &Post, back_label: &str) -> String {
    let mut body_html = String::new();
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(&post.body, options);
    html::push_html(&mut body_html, parser);

    let title = encode_text(&post.title);
    let subtitle = post.subtitle.as_ref().map(encode_text);
    let formatted_date = format_date_full(&post.date);
    let date = encode_text(&formatted_date);

    let subtitle_html = if let Some(subtitle) = subtitle {
        format!(r#"<h2 class="subtitle">{subtitle}</h2>"#)
    } else {
        String::new()
    };

    let back_label = encode_text(back_label);

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <link rel="icon" href="/favicon.ico" sizes="any" />
  <link rel="icon" href="/assets/favicons/favicon.svg" type="image/svg+xml" />
  <link rel="apple-touch-icon" href="/assets/favicons/apple-touch-icon.png" />
  <link rel="manifest" href="/assets/favicons/site.webmanifest" />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=PT+Sans+Narrow:wght@400;700&display=swap" rel="stylesheet" />
  <style>
    :root {{
      color-scheme: light;
    }}
    body {{
      margin: 0;
      padding: 48px 20px;
      font-family: "PT Sans Narrow", sans-serif;
      background: #ffffff;
      color: #1c1917;
    }}
    article {{
      max-width: 720px;
      margin: 0 auto;
      line-height: 1.7;
      font-size: 20px;
      padding-bottom: 24px;
    }}
    header {{
      margin-bottom: 32px;
      padding-top: 16px;
    }}
    h1 {{
      font-family: "Instrument Serif", serif;
      font-size: 3rem;
      margin: 0 0 8px;
      letter-spacing: -0.01em;
      line-height: 1.3;
    }}
    h2 {{
      font-family: "Instrument Serif", serif;
      font-size: 2rem;
      margin: 32px 0 12px;
      letter-spacing: -0.01em;
    }}
    .subtitle {{
      font-family: "Instrument Serif", serif;
      font-size: 1.6rem;
      font-weight: 400;
      margin: 0 0 16px;
      color: #292524;
    }}
    p {{
      margin: 0 0 16px;
    }}
    header p {{
      opacity: 0.65;
      margin-bottom: 36px;
    }}
    a {{
      color: inherit;
    }}
    pre, code {{
      font-family: "SFMono-Regular", Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    }}
    pre {{
      background: #efe6d6;
      padding: 16px;
      overflow-x: auto;
    }}
    .post-footer {{
      margin-top: 200px;
      text-align: center;
    }}
    .post-body {{
      color: #1c1917;
    }}
    .post-body p {{
      color: rgba(28, 25, 23, 0.7);
      font-weight: 300;
    }}
    .back-link {{
      display: inline-block;
      font-family: "Instrument Serif", serif;
      font-size: 1.1rem;
      font-style: italic;
      color: inherit;
      text-decoration: none;
      padding: 4px 0;
      border: none;
    }}
    @media (max-width: 640px) {{
      body {{
        padding: 32px 16px;
      }}
      article {{
        font-size: 18px;
      }}
      h1 {{
        font-size: 2.4rem;
      }}
    }}
    @media print {{
      @page {{
        margin: 1in 1.5in;
      }}
      body {{
        padding: 0;
      }}
      article {{
        font-size: 14px;
        line-height: 1.6;
        padding-bottom: 0;
      }}
      header {{
        margin-bottom: 20px;
      }}
      h1 {{
        font-size: 2.1rem;
      }}
      h2 {{
        break-after: avoid;
        page-break-after: avoid;
      }}
      h2 {{
        font-size: 1.4rem;
      }}
      .subtitle {{
        font-size: 1.12rem;
      }}
      .post-footer {{
        display: none;
      }}
    }}
  </style>
</head>
<body>
  <article>
    <header>
      <h1>{title}</h1>
      {subtitle_html}
      <p>{date}</p>
    </header>
    <div class="post-body">
      {body_html}
    </div>
    <div class="post-footer">
      <a class="back-link" href="/">- {back_label} -</a>
    </div>
  </article>
</body>
</html>"#
    )
}

fn render_not_found() -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Not found</title>
  <link rel="icon" href="/favicon.ico" sizes="any" />
  <link rel="icon" href="/assets/favicons/favicon.svg" type="image/svg+xml" />
  <link rel="apple-touch-icon" href="/assets/favicons/apple-touch-icon.png" />
  <link rel="manifest" href="/assets/favicons/site.webmanifest" />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=PT+Sans+Narrow:wght@400;700&display=swap" rel="stylesheet" />
  <style>
    :root {{
      color-scheme: light;
    }}
    body {{
      margin: 0;
      padding: 72px 20px 48px;
      font-family: "PT Sans Narrow", sans-serif;
      font-weight: 400;
      background: #ffffff;
      color: #141311;
      width: 100vw;
      height: 100vh;
      overflow: hidden;
      display: flex;
      align-items: center;
      justify-content: center;
    }}
    main {{
      max-width: 720px;
      margin: 0 auto;
      text-align: center;
      transform: translateY(-12px);
    }}
    h1 {{
      font-family: "Instrument Serif", serif;
      font-size: 3rem;
      margin: 0 0 8px;
      letter-spacing: -0.01em;
      line-height: 1.3;
    }}
    .back-link {{
      display: inline-block;
      font-family: "Instrument Serif", serif;
      font-size: 1.1rem;
      font-style: italic;
      color: inherit;
      text-decoration: none;
      padding: 4px 0;
    }}
  </style>
</head>
<body>
  <main>
    <h1>Not found</h1>
    <a class="back-link" href="/">- Home -</a>
  </main>
</body>
</html>"#
    )
}

fn back_label_from_referer(referer: Option<&axum::http::HeaderValue>) -> String {
    let Some(referer) = referer.and_then(|value| value.to_str().ok()) else {
        return "Home".to_string();
    };

    if referer.ends_with('/') || referer.ends_with("/index.html") {
        if !referer.contains("/posts/") {
            return "Back".to_string();
        }
    }

    "Home".to_string()
}

fn format_date_full(value: &str) -> String {
    match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        Ok(date) => date.format("%B %-d, %Y").to_string(),
        Err(_) => value.to_string(),
    }
}
