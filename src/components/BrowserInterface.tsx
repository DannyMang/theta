import React from 'react';
import { ContentAnalysis } from '../types';

interface BrowserInterfaceProps {
  currentUrl: string;
  loading: boolean;
  onNavigate: (url: string) => void;
  onAnalyzeContent: () => Promise<ContentAnalysis>;
}

const BrowserInterface: React.FC<BrowserInterfaceProps> = ({
  currentUrl,
  loading,
  onNavigate,
  onAnalyzeContent
}) => {
  return (
    <div className="browser-interface">
      <div className="webview-container">
        {loading ? (
          <div className="loading-indicator">
            <div className="spinner"></div>
            <p>Loading...</p>
          </div>
        ) : (
          <iframe
            src={currentUrl}
            className="webview"
            title="Browser Content"
            sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-navigation"
          />
        )}
      </div>
      
      <div className="browser-controls">
        <button 
          onClick={() => onAnalyzeContent()}
          className="analyze-button"
          disabled={loading}
        >
          🔍 Analyze Content
        </button>
      </div>
    </div>
  );
};

export default BrowserInterface; 