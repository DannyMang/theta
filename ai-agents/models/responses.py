from pydantic import BaseModel, Field
from typing import Dict, List, Optional, Any
from datetime import datetime

class ContentAnalysis(BaseModel):
    summary: str = Field(..., description="Content summary")
    keywords: List[str] = Field(..., description="Extracted keywords")
    sentiment: float = Field(..., description="Sentiment score (-1 to 1)")
    topics: List[str] = Field(..., description="Identified topics")
    reading_time: int = Field(..., description="Estimated reading time in minutes")
    complexity_score: float = Field(..., description="Complexity score (0-1)")
    language: Optional[str] = Field(None, description="Detected language")
    word_count: int = Field(..., description="Word count")

class AnalyzeContentResponse(BaseModel):
    analysis: ContentAnalysis = Field(..., description="Content analysis results")
    processing_time: float = Field(..., description="Processing time in seconds")
    model_used: str = Field(..., description="AI model used for analysis")
    confidence: Optional[float] = Field(None, description="Confidence score")

class GenerateSummaryResponse(BaseModel):
    summary: str = Field(..., description="Generated summary")
    original_length: int = Field(..., description="Original content length")
    summary_length: int = Field(..., description="Summary length")
    compression_ratio: float = Field(..., description="Compression ratio")
    processing_time: float = Field(..., description="Processing time in seconds")
    model_used: str = Field(..., description="AI model used")

class ChatResponse(BaseModel):
    response: str = Field(..., description="AI response")
    context_used: Optional[str] = Field(None, description="Context used for response")
    sources: Optional[List[str]] = Field(None, description="Sources referenced")
    suggestions: Optional[List[str]] = Field(None, description="Follow-up suggestions")
    processing_time: float = Field(..., description="Processing time in seconds")
    model_used: str = Field(..., description="AI model used")

class ResearchResult(BaseModel):
    title: str = Field(..., description="Result title")
    url: str = Field(..., description="Source URL")
    snippet: str = Field(..., description="Content snippet")
    relevance_score: float = Field(..., description="Relevance score")
    source: str = Field(..., description="Source name")
    date: Optional[datetime] = Field(None, description="Publication date")

class ResearchResponse(BaseModel):
    results: List[ResearchResult] = Field(..., description="Research results")
    summary: str = Field(..., description="Research summary")
    key_findings: List[str] = Field(..., description="Key findings")
    related_topics: List[str] = Field(..., description="Related topics")
    sources_count: int = Field(..., description="Number of sources")
    processing_time: float = Field(..., description="Processing time in seconds")

class TaskResponse(BaseModel):
    success: bool = Field(..., description="Task success status")
    result: Dict[str, Any] = Field(..., description="Task result data")
    message: str = Field(..., description="Result message")
    integrations_used: List[str] = Field(..., description="Integrations used")
    processing_time: float = Field(..., description="Processing time in seconds")
    errors: Optional[List[str]] = Field(None, description="Any errors encountered")

class SearchResult(BaseModel):
    title: str = Field(..., description="Result title")
    url: str = Field(..., description="Result URL")
    snippet: str = Field(..., description="Content snippet")
    relevance_score: float = Field(..., description="Relevance score")
    source: str = Field(..., description="Search engine used")
    metadata: Dict[str, Any] = Field(..., description="Additional metadata")

class SearchResponse(BaseModel):
    results: List[SearchResult] = Field(..., description="Search results")
    total_results: int = Field(..., description="Total results found")
    search_time: float = Field(..., description="Search time in seconds")
    query: str = Field(..., description="Original query")
    engine: str = Field(..., description="Search engine used")

class BookmarkResponse(BaseModel):
    id: str = Field(..., description="Bookmark ID")
    title: str = Field(..., description="Bookmark title")
    url: str = Field(..., description="Bookmark URL")
    tags: List[str] = Field(..., description="Bookmark tags")
    folder: Optional[str] = Field(None, description="Bookmark folder")
    ai_summary: Optional[str] = Field(None, description="AI-generated summary")
    created_at: datetime = Field(..., description="Creation timestamp")

class WorkspaceResponse(BaseModel):
    id: str = Field(..., description="Workspace ID")
    name: str = Field(..., description="Workspace name")
    description: Optional[str] = Field(None, description="Workspace description")
    tabs: List[Dict[str, Any]] = Field(..., description="Workspace tabs")
    ai_context: Optional[str] = Field(None, description="AI context")
    created_at: datetime = Field(..., description="Creation timestamp")
    updated_at: datetime = Field(..., description="Last update timestamp")

class IntegrationStatus(BaseModel):
    name: str = Field(..., description="Integration name")
    status: str = Field(..., description="Integration status")
    endpoint: Optional[str] = Field(None, description="Integration endpoint")
    last_check: datetime = Field(..., description="Last status check")
    capabilities: List[str] = Field(..., description="Integration capabilities")

class AgentStatus(BaseModel):
    name: str = Field(..., description="Agent name")
    status: str = Field(..., description="Agent status")
    active_tasks: int = Field(..., description="Number of active tasks")
    total_tasks: int = Field(..., description="Total tasks processed")
    last_activity: datetime = Field(..., description="Last activity timestamp")
    performance_metrics: Dict[str, float] = Field(..., description="Performance metrics")

class StatusResponse(BaseModel):
    service_status: str = Field(..., description="Overall service status")
    agents: List[AgentStatus] = Field(..., description="Agent status information")
    integrations: List[IntegrationStatus] = Field(..., description="Integration status")
    uptime: float = Field(..., description="Service uptime in seconds")
    memory_usage: float = Field(..., description="Memory usage in MB")
    cpu_usage: float = Field(..., description="CPU usage percentage")

class ErrorResponse(BaseModel):
    error: str = Field(..., description="Error message")
    code: str = Field(..., description="Error code")
    details: Optional[Dict[str, Any]] = Field(None, description="Error details")
    timestamp: datetime = Field(..., description="Error timestamp") 