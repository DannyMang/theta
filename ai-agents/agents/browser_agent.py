from crewai import Agent, Task, Crew, Process
from crewai.tools import BaseTool
from typing import Dict, List, Optional, Any
import asyncio
from datetime import datetime
import json

class BrowserAgent:
    def __init__(self, ai_service, database_service, integration_service):
        self.ai_service = ai_service
        self.database_service = database_service
        self.integration_service = integration_service
        self.active_tasks = 0
        self.total_tasks = 0
        self.last_activity = datetime.now()
        self.performance_metrics = {
            "avg_response_time": 0.0,
            "success_rate": 1.0,
            "total_conversations": 0
        }
        
        self.agent = Agent(
            role='AI Browser Assistant',
            goal='Provide intelligent browsing assistance and answer user questions',
            backstory="""You are an advanced AI browser assistant integrated into Theta Browser. 
            You help users navigate the web, understand content, and manage their browsing experience. 
            You can analyze web pages, provide summaries, answer questions, and help with productivity tasks.""",
            verbose=True,
            allow_delegation=False,
            tools=[
                self.create_web_search_tool(),
                self.create_content_analyzer_tool(),
                self.create_bookmark_tool(),
                self.create_tab_manager_tool()
            ]
        )

    def create_web_search_tool(self):
        class WebSearchTool(BaseTool):
            name: str = "web_search"
            description: str = "Search the web for information"
            
            def _run(self, query: str, engine: str = "duckduckgo") -> str:
                try:
                    results = asyncio.run(self.ai_service.search_web(query, engine))
                    return json.dumps(results[:5])  # Return top 5 results
                except Exception as e:
                    return f"Search failed: {str(e)}"
        
        return WebSearchTool()

    def create_content_analyzer_tool(self):
        class ContentAnalyzerTool(BaseTool):
            name: str = "analyze_content"
            description: str = "Analyze web page content for insights"
            
            def _run(self, content: str, url: str = None) -> str:
                try:
                    analysis = asyncio.run(self.ai_service.analyze_content(content, url))
                    return json.dumps(analysis)
                except Exception as e:
                    return f"Analysis failed: {str(e)}"
        
        return ContentAnalyzerTool()

    def create_bookmark_tool(self):
        class BookmarkTool(BaseTool):
            name: str = "bookmark_page"
            description: str = "Bookmark a page for later reference"
            
            def _run(self, url: str, title: str, tags: List[str] = None) -> str:
                try:
                    bookmark = asyncio.run(self.database_service.save_bookmark(url, title, tags or []))
                    return f"Bookmarked: {title} at {url}"
                except Exception as e:
                    return f"Bookmark failed: {str(e)}"
        
        return BookmarkTool()

    def create_tab_manager_tool(self):
        class TabManagerTool(BaseTool):
            name: str = "manage_tabs"
            description: str = "Manage browser tabs and workspaces"
            
            def _run(self, action: str, data: Dict[str, Any] = None) -> str:
                try:
                    if action == "create_workspace":
                        workspace = asyncio.run(self.database_service.create_workspace(
                            data.get("name", "New Workspace"),
                            data.get("description"),
                            data.get("tabs", [])
                        ))
                        return f"Created workspace: {workspace['name']}"
                    elif action == "list_tabs":
                        tabs = asyncio.run(self.database_service.get_active_tabs())
                        return json.dumps(tabs)
                    else:
                        return f"Unknown action: {action}"
                except Exception as e:
                    return f"Tab management failed: {str(e)}"
        
        return TabManagerTool()

    async def chat(self, message: str, context: Optional[str] = None, history: Optional[List[Dict[str, str]]] = None) -> Dict[str, Any]:
        start_time = datetime.now()
        self.active_tasks += 1
        
        try:
            # Build conversation context
            conversation_context = ""
            if history:
                for msg in history[-5:]:  # Use last 5 messages
                    conversation_context += f"User: {msg.get('user', '')}\nAssistant: {msg.get('assistant', '')}\n"
            
            if context:
                conversation_context += f"Context: {context}\n"
            
            # Create task for the agent
            task = Task(
                description=f"""
                User message: {message}
                
                Conversation context: {conversation_context}
                
                Provide a helpful response to the user's message. If the user is asking about web content, 
                use your tools to search or analyze as needed. If they want to bookmark something or manage 
                tabs, use the appropriate tools.
                """,
                expected_output="A comprehensive response to the user's question or request",
                agent=self.agent
            )
            
            # Create crew and execute
            crew = Crew(
                agents=[self.agent],
                tasks=[task],
                process=Process.sequential,
                verbose=True
            )
            
            result = crew.kickoff()
            
            # Update metrics
            processing_time = (datetime.now() - start_time).total_seconds()
            self.performance_metrics["avg_response_time"] = (
                self.performance_metrics["avg_response_time"] * self.performance_metrics["total_conversations"] +
                processing_time
            ) / (self.performance_metrics["total_conversations"] + 1)
            
            self.performance_metrics["total_conversations"] += 1
            self.total_tasks += 1
            self.last_activity = datetime.now()
            
            return {
                "response": str(result),
                "context_used": context,
                "sources": [],
                "suggestions": self._generate_suggestions(message),
                "processing_time": processing_time,
                "model_used": "CrewAI Browser Agent"
            }
            
        except Exception as e:
            self.performance_metrics["success_rate"] = (
                self.performance_metrics["success_rate"] * self.total_tasks + 0
            ) / (self.total_tasks + 1)
            
            return {
                "response": f"I encountered an error: {str(e)}",
                "context_used": context,
                "sources": [],
                "suggestions": [],
                "processing_time": (datetime.now() - start_time).total_seconds(),
                "model_used": "CrewAI Browser Agent"
            }
        
        finally:
            self.active_tasks -= 1

    async def chat_stream(self, message: str) -> str:
        # Simplified streaming version
        response = await self.chat(message)
        return response["response"]

    def _generate_suggestions(self, message: str) -> List[str]:
        """Generate follow-up suggestions based on the user's message"""
        suggestions = []
        
        message_lower = message.lower()
        
        if "search" in message_lower:
            suggestions.append("Would you like me to search for more specific information?")
            suggestions.append("Should I bookmark any of these results?")
        
        if "bookmark" in message_lower:
            suggestions.append("Would you like to organize your bookmarks into folders?")
            suggestions.append("Should I create a workspace for related tabs?")
        
        if "summary" in message_lower or "summarize" in message_lower:
            suggestions.append("Would you like a more detailed analysis of this content?")
            suggestions.append("Should I save this summary for later reference?")
        
        if not suggestions:
            suggestions = [
                "Is there anything specific you'd like to know more about?",
                "Would you like me to search for related information?",
                "Should I help you organize this information?"
            ]
        
        return suggestions[:3]  # Return max 3 suggestions

    async def get_status(self) -> Dict[str, Any]:
        return {
            "name": "Browser Agent",
            "status": "active" if self.active_tasks > 0 else "idle",
            "active_tasks": self.active_tasks,
            "total_tasks": self.total_tasks,
            "last_activity": self.last_activity.isoformat(),
            "performance_metrics": self.performance_metrics
        } 