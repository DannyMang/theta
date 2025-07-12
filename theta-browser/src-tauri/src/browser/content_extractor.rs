use reqwest::Client;
use regex::Regex;
use std::collections::HashMap;

pub struct ContentExtractor {
    client: Client,
    title_regex: Regex,
    meta_regex: Regex,
    link_regex: Regex,
}

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub title: String,
    pub content: String,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub word_count: usize,
    pub reading_time: u32,
}

impl ContentExtractor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            title_regex: Regex::new(r"<title[^>]*>([^<]+)</title>").unwrap(),
            meta_regex: Regex::new(r#"<meta[^>]*name=["']([^"']+)["'][^>]*content=["']([^"']+)["'][^>]*>"#).unwrap(),
            link_regex: Regex::new(r#"<a[^>]*href=["']([^"']+)["'][^>]*>"#).unwrap(),
        }
    }

    pub async fn extract_from_url(&self, url: &str) -> Result<ExtractedContent, String> {
        let response = self.client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch URL: {}", e))?;

        let html = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(self.extract_from_html(&html))
    }

    pub fn extract_from_html(&self, html: &str) -> ExtractedContent {
        let title = self.extract_title(html);
        let content = self.extract_main_content(html);
        let meta_description = self.extract_meta_description(html);
        let meta_keywords = self.extract_meta_keywords(html);
        let links = self.extract_links(html);
        let images = self.extract_images(html);
        let word_count = self.count_words(&content);
        let reading_time = self.calculate_reading_time(word_count);

        ExtractedContent {
            title,
            content,
            meta_description,
            meta_keywords,
            links,
            images,
            word_count,
            reading_time,
        }
    }

    fn extract_title(&self, html: &str) -> String {
        if let Some(captures) = self.title_regex.captures(html) {
            captures.get(1).map_or("Untitled".to_string(), |m| m.as_str().trim().to_string())
        } else {
            "Untitled".to_string()
        }
    }

    fn extract_main_content(&self, html: &str) -> String {
        // Remove script and style tags
        let script_regex = Regex::new(r"<script[^>]*>.*?</script>").unwrap();
        let style_regex = Regex::new(r"<style[^>]*>.*?</style>").unwrap();
        let tag_regex = Regex::new(r"<[^>]*>").unwrap();
        
        let mut content = script_regex.replace_all(html, "").to_string();
        content = style_regex.replace_all(&content, "").to_string();
        content = tag_regex.replace_all(&content, " ").to_string();
        
        // Clean up whitespace
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        content = whitespace_regex.replace_all(&content, " ").to_string();
        
        content.trim().to_string()
    }

    fn extract_meta_description(&self, html: &str) -> Option<String> {
        let description_regex = Regex::new(r#"<meta[^>]*name=["']description["'][^>]*content=["']([^"']+)["'][^>]*>"#).unwrap();
        description_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn extract_meta_keywords(&self, html: &str) -> Option<String> {
        let keywords_regex = Regex::new(r#"<meta[^>]*name=["']keywords["'][^>]*content=["']([^"']+)["'][^>]*>"#).unwrap();
        keywords_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn extract_links(&self, html: &str) -> Vec<String> {
        self.link_regex
            .captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn extract_images(&self, html: &str) -> Vec<String> {
        let img_regex = Regex::new(r#"<img[^>]*src=["']([^"']+)["'][^>]*>"#).unwrap();
        img_regex
            .captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }

    fn calculate_reading_time(&self, word_count: usize) -> u32 {
        // Average reading speed is ~200 words per minute
        ((word_count as f32 / 200.0).ceil() as u32).max(1)
    }
} 