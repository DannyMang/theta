from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Dict, List, Optional, Any
import asyncio
import os
from dotenv import load_dotenv

from agents.browser_agent import BrowserAgent
from agents.research_agent import ResearchAgent
from agents.productivity_agent import ProductivityAgent
from agents.content_agent import ContentAgent
from services.ai_service import AIService
from services.database_service import DatabaseService
from services.integration_service import IntegrationService
from models.requests import (
    AnalyzeContentRequest,
    GenerateSummaryRequest,
    ChatRequest,
    ResearchRequest,
    ProductivityTaskRequest
)
from models.responses import (
    AnalyzeContentResponse,
    GenerateSummaryResponse,
    ChatResponse,
    ResearchResponse,
    TaskResponse
)

load_dotenv()

app = FastAPI(title="Theta Browser AI Service", version="1.0.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

ai_service = AIService()
database_service = DatabaseService()
integration_service = IntegrationService()

browser_agent = BrowserAgent(ai_service, database_service, integration_service)
research_agent = ResearchAgent(ai_service, database_service, integration_service)
productivity_agent = ProductivityAgent(ai_service, database_service, integration_service)
content_agent = ContentAgent(ai_service, database_service, integration_service)

@app.on_event("startup")
async def startup_event():
    await database_service.initialize()
    await integration_service.initialize()
    await ai_service.initialize()

@app.get("/")
async def health_check():
    return {"status": "healthy", "service": "Theta Browser AI Service"}

@app.post("/analyze-content", response_model=AnalyzeContentResponse)
async def analyze_content(request: AnalyzeContentRequest):
    try:
        result = await content_agent.analyze_content(
            content=request.content,
            url=request.url,
            context=request.context
        )
        return AnalyzeContentResponse(**result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/generate-summary", response_model=GenerateSummaryResponse)
async def generate_summary(request: GenerateSummaryRequest):
    try:
        result = await content_agent.generate_summary(
            content=request.content,
            max_length=request.max_length,
            style=request.style
        )
        return GenerateSummaryResponse(**result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/chat", response_model=ChatResponse)
async def chat_with_ai(request: ChatRequest):
    try:
        result = await browser_agent.chat(
            message=request.message,
            context=request.context,
            history=request.history
        )
        return ChatResponse(**result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/research", response_model=ResearchResponse)
async def research_topic(request: ResearchRequest):
    try:
        result = await research_agent.research(
            query=request.query,
            depth=request.depth,
            sources=request.sources
        )
        return ResearchResponse(**result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/productivity-task", response_model=TaskResponse)
async def handle_productivity_task(request: ProductivityTaskRequest):
    try:
        result = await productivity_agent.handle_task(
            task_type=request.task_type,
            data=request.data,
            integrations=request.integrations
        )
        return TaskResponse(**result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/agents/status")
async def get_agent_status():
    return {
        "browser_agent": await browser_agent.get_status(),
        "research_agent": await research_agent.get_status(),
        "productivity_agent": await productivity_agent.get_status(),
        "content_agent": await content_agent.get_status()
    }

@app.get("/integrations/status")
async def get_integration_status():
    return await integration_service.get_status()

@app.post("/integrations/n8n/trigger")
async def trigger_n8n_workflow(workflow_id: str, data: Dict[str, Any]):
    try:
        result = await integration_service.trigger_n8n_workflow(workflow_id, data)
        return {"success": True, "result": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.websocket("/ws/chat")
async def websocket_chat(websocket):
    await websocket.accept()
    try:
        while True:
            data = await websocket.receive_text()
            response = await browser_agent.chat_stream(data)
            await websocket.send_text(response)
    except Exception as e:
        await websocket.send_text(f"Error: {str(e)}")
        await websocket.close()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000) 