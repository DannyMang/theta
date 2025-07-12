import React from 'react';
import { Tab } from '../types';

interface TabBarProps {
  tabs: Tab[];
  activeTabId: string | null;
  onSwitchTab: (tabId: string) => void;
  onNewTab: () => void;
  onCloseTab: (tabId: string) => void;
}

const TabBar: React.FC<TabBarProps> = ({
  tabs,
  activeTabId,
  onSwitchTab,
  onNewTab,
  onCloseTab
}) => {
  return (
    <div className="tab-bar">
      <div className="tabs">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            className={`tab ${tab.id === activeTabId ? 'active' : ''}`}
            onClick={() => onSwitchTab(tab.id)}
          >
            <span className="tab-title">{tab.title}</span>
            <button
              className="close-tab"
              onClick={(e) => {
                e.stopPropagation();
                onCloseTab(tab.id);
              }}
            >
              ×
            </button>
          </div>
        ))}
        <button className="new-tab-btn" onClick={onNewTab}>
          +
        </button>
      </div>
    </div>
  );
};

export default TabBar; 