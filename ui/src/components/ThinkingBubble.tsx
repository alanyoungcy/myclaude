import { useState } from 'react';

interface ThinkingBubbleProps {
  content: string;
  isThinking?: boolean;
}

export default function ThinkingBubble({ content, isThinking = false }: ThinkingBubbleProps) {
  const [isExpanded, setIsExpanded] = useState(true);

  return (
    <div className="my-4 max-w-3xl">
      <div className="bg-purple-50 border border-purple-200 rounded-lg overflow-hidden">
        {/* Header */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="w-full flex items-center justify-between px-4 py-3 hover:bg-purple-100 transition-colors"
        >
          <div className="flex items-center space-x-2">
            {isThinking ? (
              <div className="flex space-x-1">
                <div className="w-2 h-2 bg-purple-600 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 bg-purple-600 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                <div className="w-2 h-2 bg-purple-600 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
              </div>
            ) : (
              <svg className="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
            )}
            <span className="text-sm font-medium text-purple-900">
              {isThinking ? 'Thinking...' : 'Thought process'}
            </span>
          </div>
          <svg
            className={`w-5 h-5 text-purple-600 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        {/* Content */}
        {isExpanded && (
          <div className="px-4 py-3 border-t border-purple-200 bg-white">
            <div className="prose prose-sm prose-purple max-w-none">
              <pre className="text-xs text-gray-700 whitespace-pre-wrap font-mono bg-gray-50 p-3 rounded">
                {content}
              </pre>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
