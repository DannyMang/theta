import React, { useState } from 'react';

interface AddressBarProps {
  currentUrl: string;
  onNavigate: (url: string) => void;
  onSearch: (query: string) => void;
  onBookmark: () => void;
  onToggleAI: () => void;
}

const AddressBar: React.FC<AddressBarProps> = ({
  currentUrl,
  onNavigate,
  onSearch,
  onBookmark,
  onToggleAI
}) => {
  const [inputValue, setInputValue] = useState(currentUrl);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputValue.includes('.') || inputValue.startsWith('http')) {
      const url = inputValue.startsWith('http') ? inputValue : `https://${inputValue}`;
      onNavigate(url);
    } else {
      onSearch(inputValue);
    }
  };

  return (
    <div className="address-bar">
      <div className="nav-buttons">
        <button className="nav-btn">←</button>
        <button className="nav-btn">→</button>
        <button className="nav-btn">↻</button>
      </div>
      
      <form onSubmit={handleSubmit} className="url-form">
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder="Enter URL or search..."
          className="url-input"
        />
      </form>
      
      <div className="toolbar-buttons">
        <button onClick={onBookmark} className="toolbar-btn">⭐</button>
        <button onClick={onToggleAI} className="toolbar-btn ai-btn">🤖</button>
      </div>
    </div>
  );
};

export default AddressBar; 