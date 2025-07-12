use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarIntegration {
    pub provider: String,
    pub api_key: Option<String>,
    #[serde(skip)]
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub is_all_day: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarEvent {
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub is_all_day: bool,
}

impl CalendarIntegration {
    pub fn new(provider: String, api_key: Option<String>) -> Self {
        Self {
            provider,
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get_events(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        match self.provider.as_str() {
            "google" => self.get_google_events(start_date, end_date).await,
            "outlook" => self.get_outlook_events(start_date, end_date).await,
            _ => Err("Unsupported calendar provider".to_string()),
        }
    }

    pub async fn create_event(&self, event: CreateCalendarEvent) -> Result<CalendarEvent, String> {
        match self.provider.as_str() {
            "google" => self.create_google_event(event).await,
            "outlook" => self.create_outlook_event(event).await,
            _ => Err("Unsupported calendar provider".to_string()),
        }
    }

    async fn get_google_events(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        let api_key = self.api_key.as_ref().ok_or("Google API key not set")?;
        
        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/primary/events?timeMin={}&timeMax={}&key={}",
            start_date.to_rfc3339(),
            end_date.to_rfc3339(),
            api_key
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Google events: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut events = Vec::new();
        
        if let Some(items) = json["items"].as_array() {
            for item in items {
                let event = CalendarEvent {
                    id: item["id"].as_str().unwrap_or("").to_string(),
                    title: item["summary"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().map(|s| s.to_string()),
                    start_time: DateTime::parse_from_rfc3339(item["start"]["dateTime"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid start time: {}", e))?.with_timezone(&Utc),
                    end_time: DateTime::parse_from_rfc3339(item["end"]["dateTime"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid end time: {}", e))?.with_timezone(&Utc),
                    location: item["location"].as_str().map(|s| s.to_string()),
                    attendees: vec![],
                    is_all_day: item["start"]["date"].is_string(),
                };
                events.push(event);
            }
        }
        
        Ok(events)
    }

    async fn get_outlook_events(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        let api_key = self.api_key.as_ref().ok_or("Outlook API key not set")?;
        
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/events?$filter=start/dateTime ge '{}' and end/dateTime le '{}'",
            start_date.to_rfc3339(),
            end_date.to_rfc3339()
        );
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Outlook events: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut events = Vec::new();
        
        if let Some(items) = json["value"].as_array() {
            for item in items {
                let event = CalendarEvent {
                    id: item["id"].as_str().unwrap_or("").to_string(),
                    title: item["subject"].as_str().unwrap_or("").to_string(),
                    description: item["body"]["content"].as_str().map(|s| s.to_string()),
                    start_time: DateTime::parse_from_rfc3339(item["start"]["dateTime"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid start time: {}", e))?.with_timezone(&Utc),
                    end_time: DateTime::parse_from_rfc3339(item["end"]["dateTime"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid end time: {}", e))?.with_timezone(&Utc),
                    location: item["location"]["displayName"].as_str().map(|s| s.to_string()),
                    attendees: vec![],
                    is_all_day: item["isAllDay"].as_bool().unwrap_or(false),
                };
                events.push(event);
            }
        }
        
        Ok(events)
    }

    async fn create_google_event(&self, event: CreateCalendarEvent) -> Result<CalendarEvent, String> {
        let api_key = self.api_key.as_ref().ok_or("Google API key not set")?;
        
        let url = format!("https://www.googleapis.com/calendar/v3/calendars/primary/events?key={}", api_key);
        
        let event_data = serde_json::json!({
            "summary": event.title,
            "description": event.description,
            "start": {
                "dateTime": event.start_time.to_rfc3339(),
                "timeZone": "UTC"
            },
            "end": {
                "dateTime": event.end_time.to_rfc3339(),
                "timeZone": "UTC"
            },
            "location": event.location
        });
        
        let response = self.client
            .post(&url)
            .json(&event_data)
            .send()
            .await
            .map_err(|e| format!("Failed to create Google event: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let created_event = CalendarEvent {
            id: json["id"].as_str().unwrap_or("").to_string(),
            title: json["summary"].as_str().unwrap_or("").to_string(),
            description: json["description"].as_str().map(|s| s.to_string()),
            start_time: DateTime::parse_from_rfc3339(json["start"]["dateTime"].as_str().unwrap_or(""))
                .map_err(|e| format!("Invalid start time: {}", e))?.with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(json["end"]["dateTime"].as_str().unwrap_or(""))
                .map_err(|e| format!("Invalid end time: {}", e))?.with_timezone(&Utc),
            location: json["location"].as_str().map(|s| s.to_string()),
            attendees: vec![],
            is_all_day: false,
        };
        
        Ok(created_event)
    }

    async fn create_outlook_event(&self, event: CreateCalendarEvent) -> Result<CalendarEvent, String> {
        let api_key = self.api_key.as_ref().ok_or("Outlook API key not set")?;
        
        let url = "https://graph.microsoft.com/v1.0/me/events";
        
        let event_data = serde_json::json!({
            "subject": event.title,
            "body": {
                "contentType": "HTML",
                "content": event.description.unwrap_or_default()
            },
            "start": {
                "dateTime": event.start_time.to_rfc3339(),
                "timeZone": "UTC"
            },
            "end": {
                "dateTime": event.end_time.to_rfc3339(),
                "timeZone": "UTC"
            },
            "location": {
                "displayName": event.location.unwrap_or_default()
            }
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&event_data)
            .send()
            .await
            .map_err(|e| format!("Failed to create Outlook event: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let created_event = CalendarEvent {
            id: json["id"].as_str().unwrap_or("").to_string(),
            title: json["subject"].as_str().unwrap_or("").to_string(),
            description: json["body"]["content"].as_str().map(|s| s.to_string()),
            start_time: DateTime::parse_from_rfc3339(json["start"]["dateTime"].as_str().unwrap_or(""))
                .map_err(|e| format!("Invalid start time: {}", e))?.with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(json["end"]["dateTime"].as_str().unwrap_or(""))
                .map_err(|e| format!("Invalid end time: {}", e))?.with_timezone(&Utc),
            location: json["location"]["displayName"].as_str().map(|s| s.to_string()),
            attendees: vec![],
            is_all_day: json["isAllDay"].as_bool().unwrap_or(false),
        };
        
        Ok(created_event)
    }
}

impl Default for CalendarIntegration {
    fn default() -> Self {
        Self::new(
            std::env::var("CALENDAR_PROVIDER").unwrap_or_else(|_| "google".to_string()),
            std::env::var("CALENDAR_API_KEY").ok(),
        )
    }
} 