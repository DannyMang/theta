import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import BrowserInterface from "./components/BrowserInterface";
import AIChatPanel from "./components/AIChatPanel";
import Sidebar from "./components/Sidebar";
import AddressBar from "./components/AddressBar";
import TabBar from "./components/TabBar";
import { AIService } from "./services/AIService";
import { BrowserService } from "./services/BrowserService";
import { Tab, AIChat, Workspace } from "./types";
import "./App.css";

function App() {
  const [currentUrl, setCurrentUrl] = useState("https://www.google.com");
  const [tabs, setTabs] = useState<Tab[]>([]);
  const [activeTabId, setActiveTabId] = useState<string | null>(null);
  const [showAIPanel, setShowAIPanel] = useState(false);
  const [aiChats, setAiChats] = useState<AIChat[]>([]);
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [currentWorkspace, setCurrentWorkspace] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const aiService = new AIService();
  const browserService = new BrowserService();

  useEffect(() => {
    initializeBrowser();
  }, []);

  const initializeBrowser = async () => {
    try {
      setLoading(true);
      
      // Create initial tab
      const initialTab = await browserService.createTab(currentUrl, "New Tab");
      setTabs([initialTab]);
      setActiveTabId(initialTab.id);

      // Load workspaces
      const workspaceList = await browserService.getWorkspaces();
      setWorkspaces(workspaceList);

    } catch (error) {
      console.error("Failed to initialize browser:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleNavigate = async (url: string) => {
    try {
      setLoading(true);
      setCurrentUrl(url);
      
      // Update current tab
      if (activeTabId) {
        const updatedTabs = tabs.map(tab => 
          tab.id === activeTabId 
            ? { ...tab, url, title: "Loading..." }
            : tab
        );
        setTabs(updatedTabs);
      }

      // Navigate via Tauri command
      await invoke("navigate_to_url", { url });
      
    } catch (error) {
      console.error("Navigation failed:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async (query: string) => {
    try {
      const results = await invoke("search_web", { 
        query, 
        searchEngine: "duckduckgo" 
      });
      
      // Navigate to first result or perform search
      if (results && results.length > 0) {
        await handleNavigate(results[0].url);
      } else {
        await handleNavigate(`https://duckduckgo.com/?q=${encodeURIComponent(query)}`);
      }
    } catch (error) {
      console.error("Search failed:", error);
    }
  };

  const handleNewTab = async (url: string = "https://www.google.com") => {
    try {
      const newTab = await browserService.createTab(url, "New Tab");
      setTabs([...tabs, newTab]);
      setActiveTabId(newTab.id);
      setCurrentUrl(url);
    } catch (error) {
      console.error("Failed to create new tab:", error);
    }
  };

  const handleCloseTab = async (tabId: string) => {
    try {
      await browserService.closeTab(tabId);
      const updatedTabs = tabs.filter(tab => tab.id !== tabId);
      setTabs(updatedTabs);
      
      // Switch to another tab if the active one was closed
      if (activeTabId === tabId && updatedTabs.length > 0) {
        setActiveTabId(updatedTabs[0].id);
        setCurrentUrl(updatedTabs[0].url);
      }
    } catch (error) {
      console.error("Failed to close tab:", error);
    }
  };

  const handleSwitchTab = (tabId: string) => {
    const tab = tabs.find(t => t.id === tabId);
    if (tab) {
      setActiveTabId(tabId);
      setCurrentUrl(tab.url);
    }
  };

  const handleBookmark = async () => {
    try {
      const activeTab = tabs.find(tab => tab.id === activeTabId);
      if (activeTab) {
        await invoke("bookmark_page", {
          url: activeTab.url,
          title: activeTab.title,
          tags: [],
          folder: null
        });
      }
    } catch (error) {
      console.error("Failed to bookmark page:", error);
    }
  };

  const handleAIChat = async (message: string) => {
    try {
      const response = await aiService.chat(message, currentUrl);
      return response;
    } catch (error) {
      console.error("AI chat failed:", error);
      throw error;
    }
  };

  const handleAnalyzeContent = async () => {
    try {
      const content = await invoke("get_page_content", { url: currentUrl });
      const analysis = await aiService.analyzeContent(content.content, currentUrl);
      return analysis;
    } catch (error) {
      console.error("Content analysis failed:", error);
      throw error;
    }
  };

  if (loading && tabs.length === 0) {
    return (
      <div className="loading-screen">
        <div className="loading-spinner"></div>
        <p>Initializing Theta Browser...</p>
      </div>
    );
  }

  return (
    <div className="app">
      <div className="browser-header">
        <TabBar
          tabs={tabs}
          activeTabId={activeTabId}
          onSwitchTab={handleSwitchTab}
          onNewTab={handleNewTab}
          onCloseTab={handleCloseTab}
        />
        <AddressBar
          currentUrl={currentUrl}
          onNavigate={handleNavigate}
          onSearch={handleSearch}
          onBookmark={handleBookmark}
          onToggleAI={() => setShowAIPanel(!showAIPanel)}
        />
      </div>

      <div className="browser-content">
        <Sidebar
          collapsed={sidebarCollapsed}
          onToggle={() => setSidebarCollapsed(!sidebarCollapsed)}
          workspaces={workspaces}
          currentWorkspace={currentWorkspace}
          onWorkspaceChange={setCurrentWorkspace}
        />

        <main className="main-content">
          <BrowserInterface
            currentUrl={currentUrl}
            loading={loading}
            onNavigate={handleNavigate}
            onAnalyzeContent={handleAnalyzeContent}
          />
        </main>

        {showAIPanel && (
          <AIChatPanel
            onClose={() => setShowAIPanel(false)}
            onChat={handleAIChat}
            currentUrl={currentUrl}
            chats={aiChats}
          />
        )}
      </div>
    </div>
  );
}

export default App;
