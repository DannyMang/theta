import React, { useState, useRef, useEffect } from 'react';
import { AIChat, AIResponse } from '../types';

interface AIChatPanelProps {
  onClose: () => void;
  onChat: (message: string) => Promise<AIResponse>;
  currentUrl: string;
  chats: AIChat[];
}

const AIChatPanel: React.FC<AIChatPanelProps> = ({
  onClose,
  onChat,
  currentUrl,
  chats
}) => {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<Array<{role: 'user' | 'assistant', content: string}>>([]);
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!message.trim() || isLoading) return;

    const userMessage = message.trim();
    setMessage('');
    setMessages(prev => [...prev, { role: 'user', content: userMessage }]);
    setIsLoading(true);

    try {
      const response = await onChat(userMessage);
      setMessages(prev => [...prev, { role: 'assistant', content: response.response }]);
    } catch (error) {
      setMessages(prev => [...prev, { 
        role: 'assistant', 
        content: 'Sorry, I encountered an error. Please try again.' 
      }]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleQuickAction = (action: string) => {
    const quickActions = {
      'summarize': `Summarize the content of this page: ${currentUrl}`,
      'analyze': `Analyze the content and key points from: ${currentUrl}`,
      'questions': `What questions could I ask about this content?`,
      'related': `What are some related topics to explore?`
    };
    
    setMessage(quickActions[action as keyof typeof quickActions] || '');
  };

  return (
    <div className="ai-chat-panel">
      <div className="chat-header">
        <h3>AI Assistant</h3>
        <button className="close-btn" onClick={onClose}>×</button>
      </div>
      
      <div className="quick-actions">
        <button onClick={() => handleQuickAction('summarize')} className="quick-btn">
          📝 Summarize
        </button>
        <button onClick={() => handleQuickAction('analyze')} className="quick-btn">
          🔍 Analyze
        </button>
        <button onClick={() => handleQuickAction('questions')} className="quick-btn">
          ❓ Questions
        </button>
        <button onClick={() => handleQuickAction('related')} className="quick-btn">
          🔗 Related
        </button>
      </div>
      
      <div className="messages-container">
        {messages.length === 0 ? (
          <div className="welcome-message">
            <p>👋 Hi! I'm your AI assistant. I can help you:</p>
            <ul>
              <li>Summarize web pages</li>
              <li>Answer questions about content</li>
              <li>Research topics</li>
              <li>Manage your browsing</li>
            </ul>
            <p>What would you like to know?</p>
          </div>
        ) : (
          <>
            {messages.map((msg, index) => (
              <div key={index} className={`message ${msg.role}`}>
                <div className="message-content">{msg.content}</div>
              </div>
            ))}
            {isLoading && (
              <div className="message assistant">
                <div className="message-content">
                  <div className="typing-indicator">
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              </div>
            )}
          </>
        )}
        <div ref={messagesEndRef} />
      </div>
      
      <form onSubmit={handleSubmit} className="message-form">
        <input
          type="text"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="Ask me anything..."
          className="message-input"
          disabled={isLoading}
        />
        <button type="submit" disabled={isLoading || !message.trim()} className="send-btn">
          Send
        </button>
      </form>
    </div>
  );
};

export default AIChatPanel; 