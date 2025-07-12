import React from 'react';
import { Workspace } from '../types';

interface SidebarProps {
  collapsed: boolean;
  onToggle: () => void;
  workspaces: Workspace[];
  currentWorkspace: string | null;
  onWorkspaceChange: (workspaceId: string) => void;
}

const Sidebar: React.FC<SidebarProps> = ({
  collapsed,
  onToggle,
  workspaces,
  currentWorkspace,
  onWorkspaceChange
}) => {
  return (
    <div className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
      <div className="sidebar-header">
        <button className="toggle-btn" onClick={onToggle}>
          {collapsed ? '→' : '←'}
        </button>
        {!collapsed && <h3>Workspaces</h3>}
      </div>
      
      {!collapsed && (
        <div className="sidebar-content">
          <div className="workspaces">
            {workspaces.map((workspace) => (
              <div
                key={workspace.id}
                className={`workspace ${currentWorkspace === workspace.id ? 'active' : ''}`}
                onClick={() => onWorkspaceChange(workspace.id)}
              >
                <div className="workspace-name">{workspace.name}</div>
                <div className="workspace-tabs">{workspace.tabs.length} tabs</div>
              </div>
            ))}
          </div>
          
          <div className="sidebar-actions">
            <button className="action-btn">+ New Workspace</button>
            <button className="action-btn">📚 Bookmarks</button>
            <button className="action-btn">🔍 History</button>
          </div>
        </div>
      )}
    </div>
  );
};

export default Sidebar; 