import { AIResponse, ContentAnalysis, SearchResult } from '../types';

export class AIService {
  private baseUrl = 'http://localhost:8000';

  async chat(message: string, context?: string): Promise<AIResponse> {
    const response = await fetch(`${this.baseUrl}/chat`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        message,
        context,
        history: []
      }),
    });

    if (!response.ok) {
      throw new Error(`AI chat failed: ${response.statusText}`);
    }

    return response.json();
  }

  async analyzeContent(content: string, url?: string): Promise<ContentAnalysis> {
    const response = await fetch(`${this.baseUrl}/analyze-content`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        content,
        url,
        context: null
      }),
    });

    if (!response.ok) {
      throw new Error(`Content analysis failed: ${response.statusText}`);
    }

    const data = await response.json();
    return data.analysis;
  }

  async generateSummary(content: string, maxLength?: number): Promise<string> {
    const response = await fetch(`${this.baseUrl}/generate-summary`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        content,
        max_length: maxLength || 150,
        style: 'concise'
      }),
    });

    if (!response.ok) {
      throw new Error(`Summary generation failed: ${response.statusText}`);
    }

    const data = await response.json();
    return data.summary;
  }

  async research(query: string, depth?: string): Promise<SearchResult[]> {
    const response = await fetch(`${this.baseUrl}/research`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        query,
        depth: depth || 'standard',
        sources: null
      }),
    });

    if (!response.ok) {
      throw new Error(`Research failed: ${response.statusText}`);
    }

    const data = await response.json();
    return data.results;
  }

  async getAgentStatus(): Promise<any> {
    const response = await fetch(`${this.baseUrl}/agents/status`);
    
    if (!response.ok) {
      throw new Error(`Failed to get agent status: ${response.statusText}`);
    }

    return response.json();
  }

  async getIntegrationStatus(): Promise<any> {
    const response = await fetch(`${this.baseUrl}/integrations/status`);
    
    if (!response.ok) {
      throw new Error(`Failed to get integration status: ${response.statusText}`);
    }

    return response.json();
  }

  async triggerN8nWorkflow(workflowId: string, data: any): Promise<any> {
    const response = await fetch(`${this.baseUrl}/integrations/n8n/trigger`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ workflow_id: workflowId, data }),
    });

    if (!response.ok) {
      throw new Error(`N8n workflow trigger failed: ${response.statusText}`);
    }

    return response.json();
  }

  async handleProductivityTask(taskType: string, data: any): Promise<any> {
    const response = await fetch(`${this.baseUrl}/productivity-task`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        task_type: taskType,
        data,
        integrations: []
      }),
    });

    if (!response.ok) {
      throw new Error(`Productivity task failed: ${response.statusText}`);
    }

    return response.json();
  }

  // WebSocket connection for real-time chat
  connectWebSocket(onMessage: (message: string) => void): WebSocket {
    const ws = new WebSocket(`ws://localhost:8000/ws/chat`);
    
    ws.onmessage = (event) => {
      onMessage(event.data);
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    return ws;
  }
} 