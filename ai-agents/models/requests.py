from pydantic import BaseModel, Field
from typing import Dict, List, Optional, Any
from enum import Enum

class TaskType(str, Enum):
    CALENDAR_EVENT = "calendar_event"
    EMAIL_COMPOSE = "email_compose"
    NOTE_TAKING = "note_taking"
    TASK_MANAGEMENT = "task_management"
    DOCUMENT_CREATION = "document_creation"
    MEETING_SCHEDULING = "meeting_scheduling"

class ResearchDepth(str, Enum):
    QUICK = "quick"
    STANDARD = "standard"
    DEEP = "deep"

class SummaryStyle(str, Enum):
    CONCISE = "concise"
    DETAILED = "detailed"
    BULLET_POINTS = "bullet_points"
    ACADEMIC = "academic"

class AnalyzeContentRequest(BaseModel):
    content: str = Field(..., description="Content to analyze")
    url: Optional[str] = Field(None, description="URL of the content source")
    context: Optional[str] = Field(None, description="Additional context for analysis")

class GenerateSummaryRequest(BaseModel):
    content: str = Field(..., description="Content to summarize")
    max_length: Optional[int] = Field(150, description="Maximum length of summary")
    style: Optional[SummaryStyle] = Field(SummaryStyle.CONCISE, description="Summary style")

class ChatRequest(BaseModel):
    message: str = Field(..., description="User message")
    context: Optional[str] = Field(None, description="Conversation context")
    history: Optional[List[Dict[str, str]]] = Field(None, description="Chat history")

class ResearchRequest(BaseModel):
    query: str = Field(..., description="Research query")
    depth: Optional[ResearchDepth] = Field(ResearchDepth.STANDARD, description="Research depth")
    sources: Optional[List[str]] = Field(None, description="Specific sources to search")

class ProductivityTaskRequest(BaseModel):
    task_type: TaskType = Field(..., description="Type of productivity task")
    data: Dict[str, Any] = Field(..., description="Task data")
    integrations: Optional[List[str]] = Field(None, description="Required integrations")

class N8nTriggerRequest(BaseModel):
    workflow_id: str = Field(..., description="N8n workflow ID")
    data: Dict[str, Any] = Field(..., description="Data to send to workflow")
    context: Optional[str] = Field(None, description="Additional context")

class ContentExtractionRequest(BaseModel):
    url: str = Field(..., description="URL to extract content from")
    extract_links: bool = Field(True, description="Extract links from content")
    extract_images: bool = Field(True, description="Extract images from content")
    extract_metadata: bool = Field(True, description="Extract metadata from content")

class SearchRequest(BaseModel):
    query: str = Field(..., description="Search query")
    engine: Optional[str] = Field("duckduckgo", description="Search engine to use")
    limit: Optional[int] = Field(10, description="Maximum number of results")
    filters: Optional[Dict[str, str]] = Field(None, description="Search filters")

class BookmarkRequest(BaseModel):
    url: str = Field(..., description="URL to bookmark")
    title: str = Field(..., description="Bookmark title")
    tags: Optional[List[str]] = Field(None, description="Bookmark tags")
    folder: Optional[str] = Field(None, description="Bookmark folder")
    auto_summarize: bool = Field(False, description="Auto-generate AI summary")

class WorkspaceRequest(BaseModel):
    name: str = Field(..., description="Workspace name")
    description: Optional[str] = Field(None, description="Workspace description")
    tabs: Optional[List[str]] = Field(None, description="Initial tab URLs")
    ai_context: Optional[str] = Field(None, description="AI context for workspace")

class PreferencesRequest(BaseModel):
    theme: Optional[str] = Field(None, description="UI theme")
    default_search_engine: Optional[str] = Field(None, description="Default search engine")
    ai_provider: Optional[str] = Field(None, description="Default AI provider")
    privacy_mode: Optional[bool] = Field(None, description="Privacy mode setting")
    auto_summarize: Optional[bool] = Field(None, description="Auto-summarize setting")
    sidebar_position: Optional[str] = Field(None, description="Sidebar position")
    custom_css: Optional[str] = Field(None, description="Custom CSS styles") 