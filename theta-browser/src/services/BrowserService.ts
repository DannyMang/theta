import { Tab, Workspace, Bookmark, WebPageContent } from '../types';

export class BrowserService {
  private tabs: Map<string, Tab> = new Map();
  private workspaces: Map<string, Workspace> = new Map();

  async createTab(url: string, title: string): Promise<Tab> {
    const tab: Tab = {
      id: this.generateId(),
      url,
      title,
      isActive: true,
      isPinned: false,
      createdAt: new Date(),
      lastVisited: new Date()
    };

    this.tabs.set(tab.id, tab);
    return tab;
  }

  async closeTab(tabId: string): Promise<void> {
    this.tabs.delete(tabId);
  }

  async getTab(tabId: string): Promise<Tab | null> {
    return this.tabs.get(tabId) || null;
  }

  async getAllTabs(): Promise<Tab[]> {
    return Array.from(this.tabs.values());
  }

  async updateTab(tabId: string, updates: Partial<Tab>): Promise<Tab | null> {
    const tab = this.tabs.get(tabId);
    if (!tab) return null;

    const updatedTab = { ...tab, ...updates };
    this.tabs.set(tabId, updatedTab);
    return updatedTab;
  }

  async createWorkspace(name: string, description?: string): Promise<Workspace> {
    const workspace: Workspace = {
      id: this.generateId(),
      name,
      description,
      tabs: [],
      createdAt: new Date(),
      updatedAt: new Date()
    };

    this.workspaces.set(workspace.id, workspace);
    return workspace;
  }

  async getWorkspaces(): Promise<Workspace[]> {
    return Array.from(this.workspaces.values());
  }

  async getWorkspace(workspaceId: string): Promise<Workspace | null> {
    return this.workspaces.get(workspaceId) || null;
  }

  async updateWorkspace(workspaceId: string, updates: Partial<Workspace>): Promise<Workspace | null> {
    const workspace = this.workspaces.get(workspaceId);
    if (!workspace) return null;

    const updatedWorkspace = { ...workspace, ...updates, updatedAt: new Date() };
    this.workspaces.set(workspaceId, updatedWorkspace);
    return updatedWorkspace;
  }

  async deleteWorkspace(workspaceId: string): Promise<void> {
    this.workspaces.delete(workspaceId);
  }

  async addTabToWorkspace(workspaceId: string, tabId: string): Promise<void> {
    const workspace = this.workspaces.get(workspaceId);
    const tab = this.tabs.get(tabId);
    
    if (workspace && tab) {
      tab.workspaceId = workspaceId;
      workspace.tabs.push(tab);
      workspace.updatedAt = new Date();
      
      this.tabs.set(tabId, tab);
      this.workspaces.set(workspaceId, workspace);
    }
  }

  async removeTabFromWorkspace(workspaceId: string, tabId: string): Promise<void> {
    const workspace = this.workspaces.get(workspaceId);
    const tab = this.tabs.get(tabId);
    
    if (workspace && tab) {
      workspace.tabs = workspace.tabs.filter(t => t.id !== tabId);
      workspace.updatedAt = new Date();
      
      tab.workspaceId = undefined;
      
      this.tabs.set(tabId, tab);
      this.workspaces.set(workspaceId, workspace);
    }
  }

  async searchTabs(query: string): Promise<Tab[]> {
    const tabs = Array.from(this.tabs.values());
    return tabs.filter(tab => 
      tab.title.toLowerCase().includes(query.toLowerCase()) ||
      tab.url.toLowerCase().includes(query.toLowerCase())
    );
  }

  async getRecentTabs(limit: number = 10): Promise<Tab[]> {
    const tabs = Array.from(this.tabs.values());
    return tabs
      .sort((a, b) => b.lastVisited.getTime() - a.lastVisited.getTime())
      .slice(0, limit);
  }

  async getPinnedTabs(): Promise<Tab[]> {
    const tabs = Array.from(this.tabs.values());
    return tabs.filter(tab => tab.isPinned);
  }

  async pinTab(tabId: string): Promise<void> {
    const tab = this.tabs.get(tabId);
    if (tab) {
      tab.isPinned = true;
      this.tabs.set(tabId, tab);
    }
  }

  async unpinTab(tabId: string): Promise<void> {
    const tab = this.tabs.get(tabId);
    if (tab) {
      tab.isPinned = false;
      this.tabs.set(tabId, tab);
    }
  }

  async duplicateTab(tabId: string): Promise<Tab | null> {
    const originalTab = this.tabs.get(tabId);
    if (!originalTab) return null;

    return this.createTab(originalTab.url, originalTab.title);
  }

  async getTabHistory(tabId: string): Promise<string[]> {
    // This would typically be stored in a more sophisticated way
    // For now, returning the current URL
    const tab = this.tabs.get(tabId);
    return tab ? [tab.url] : [];
  }

  async clearTabHistory(tabId: string): Promise<void> {
    // Implementation for clearing tab history
    // This would typically interact with the actual browser history
  }

  async exportWorkspace(workspaceId: string): Promise<any> {
    const workspace = this.workspaces.get(workspaceId);
    if (!workspace) return null;

    return {
      ...workspace,
      exportedAt: new Date().toISOString()
    };
  }

  async importWorkspace(workspaceData: any): Promise<Workspace> {
    const workspace: Workspace = {
      id: this.generateId(),
      name: workspaceData.name,
      description: workspaceData.description,
      tabs: workspaceData.tabs || [],
      aiContext: workspaceData.aiContext,
      createdAt: new Date(),
      updatedAt: new Date()
    };

    this.workspaces.set(workspace.id, workspace);
    return workspace;
  }

  private generateId(): string {
    return Math.random().toString(36).substr(2, 9);
  }
} 