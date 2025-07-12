export interface Tab {
  id: string;
  url: string;
  title: string;
  favicon?: string;
  isActive: boolean;
  isPinned: boolean;
  workspaceId?: string;
  createdAt: Date;
  lastVisited: Date;
}

export interface Workspace {
  id: string;
  name: string;
  description?: string;
  tabs: Tab[];
  aiContext?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface AIChat {
  id: string;
  messages: ChatMessage[];
  context?: string;
  model: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  metadata?: any;
}

export interface Bookmark {
  id: string;
  title: string;
  url: string;
  tags: string[];
  folder?: string;
  aiSummary?: string;
  createdAt: Date;
}

export interface ContentAnalysis {
  summary: string;
  keywords: string[];
  sentiment: number;
  topics: string[];
  readingTime: number;
  complexityScore: number;
  language?: string;
  wordCount: number;
}

export interface SearchResult {
  title: string;
  url: string;
  snippet: string;
  relevanceScore: number;
  metadata: any;
}

export interface AIResponse {
  response: string;
  contextUsed?: string;
  sources?: string[];
  suggestions?: string[];
  processingTime: number;
  modelUsed: string;
}

export interface UserPreferences {
  theme: string;
  defaultSearchEngine: string;
  aiProvider: string;
  privacyMode: boolean;
  autoSummarize: boolean;
  sidebarPosition: string;
  customCss?: string;
}

export interface WebPageContent {
  url: string;
  title: string;
  content: string;
  html: string;
  links: string[];
  images: string[];
  metadata: any;
  extractedAt: Date;
} 