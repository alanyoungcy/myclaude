import { useState } from 'react';
import Canvas from './Canvas';

interface ArtifactsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  content: string;
  title: string;
  type: 'markdown' | 'code';
  language?: string;
}

export default function ArtifactsPanel({
  isOpen,
  onClose,
  content,
  title,
  type,
  language = 'javascript'
}: ArtifactsPanelProps) {
  const [viewMode, setViewMode] = useState<'preview' | 'code'>('preview');

  if (!isOpen) return null;

  return (
    <div className="fixed inset-y-0 right-0 w-[600px] bg-surface border-l border-border shadow-2xl z-50 flex flex-col animate-slide-in">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-background-secondary">
        <div className="flex items-center space-x-3">
          <div className="flex items-center space-x-2">
            <svg className="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <span className="font-medium text-text">{title}</span>
          </div>

          {/* View mode toggle */}
          {type === 'code' && (
            <div className="flex items-center space-x-1 ml-4">
              <button
                onClick={() => setViewMode('preview')}
                className={`px-3 py-1 text-xs rounded transition-colors ${
                  viewMode === 'preview'
                    ? 'bg-primary text-white'
                    : 'bg-background-tertiary text-text-secondary hover:bg-background-secondary'
                }`}
              >
                Preview
              </button>
              <button
                onClick={() => setViewMode('code')}
                className={`px-3 py-1 text-xs rounded transition-colors ${
                  viewMode === 'code'
                    ? 'bg-primary text-white'
                    : 'bg-background-tertiary text-text-secondary hover:bg-background-secondary'
                }`}
              >
                Code
              </button>
            </div>
          )}
        </div>

        {/* Actions */}
        <div className="flex items-center space-x-2">
          <button
            onClick={() => {
              navigator.clipboard.writeText(content);
            }}
            className="btn btn-secondary btn-sm"
            title="Copy"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
          </button>

          <button
            onClick={onClose}
            className="btn btn-ghost btn-sm"
            title="Close"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-6">
        <Canvas
          content={content}
          title={title}
          type={type === 'code' && viewMode === 'code' ? 'code' : 'markdown'}
          language={language}
        />
      </div>

      {/* Footer info */}
      <div className="px-4 py-2 border-t border-border bg-background-secondary text-xs text-text-tertiary">
        {type === 'markdown' ? 'Markdown Document' : `${language} Code`} • {content.split('\n').length} lines
      </div>
    </div>
  );
}
