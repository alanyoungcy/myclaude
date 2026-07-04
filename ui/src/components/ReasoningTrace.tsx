import { useState } from 'react';

export interface ReasoningStep {
  id: string;
  name: string;
  description: string;
  duration?: number;
  status: 'pending' | 'running' | 'completed';
  metadata?: {
    queries?: string[];
    sources?: string[];
    count?: number;
  };
}

interface ReasoningTraceProps {
  steps: ReasoningStep[];
  totalDuration?: number;
  isThinking?: boolean;
}

export default function ReasoningTrace({ steps, totalDuration, isThinking = false }: ReasoningTraceProps) {
  const [isExpanded, setIsExpanded] = useState(true);

  const formatDuration = (ms: number) => {
    return (ms / 1000).toFixed(1) + 's';
  };

  const getStatusIcon = (status: ReasoningStep['status']) => {
    switch (status) {
      case 'completed':
        return (
          <div className="w-5 h-5 rounded-full bg-green-100 flex items-center justify-center flex-shrink-0">
            <svg className="w-3 h-3 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          </div>
        );
      case 'running':
        return (
          <div className="w-5 h-5 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
            <div className="w-2 h-2 bg-blue-600 rounded-full animate-pulse"></div>
          </div>
        );
      case 'pending':
        return (
          <div className="w-5 h-5 rounded-full bg-gray-100 flex items-center justify-center flex-shrink-0">
            <div className="w-2 h-2 bg-gray-400 rounded-full"></div>
          </div>
        );
    }
  };

  return (
    <div className="my-4 max-w-3xl">
      <div className="bg-white border border-gray-200 rounded-lg overflow-hidden shadow-sm">
        {/* Header */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
        >
          <div className="flex items-center space-x-2">
            {isThinking ? (
              <div className="flex space-x-1">
                <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
              </div>
            ) : (
              <svg className="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
            )}
            <span className="text-sm font-medium text-gray-900">
              Reasoning Trace
            </span>
            {totalDuration && (
              <span className="text-xs text-gray-500">
                ({formatDuration(totalDuration)})
              </span>
            )}
          </div>
          <svg
            className={`w-5 h-5 text-gray-600 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        {/* Steps */}
        {isExpanded && steps.length > 0 && (
          <div className="px-4 py-3 border-t border-gray-200 space-y-3">
            {steps.map((step) => (
              <div key={step.id} className="flex space-x-3">
                {getStatusIcon(step.status)}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center space-x-2 mb-1">
                    <span className="font-medium text-gray-900 text-sm">{step.name}</span>
                    {step.duration && (
                      <span className="text-xs text-gray-500">
                        {formatDuration(step.duration)}
                      </span>
                    )}
                  </div>
                  <p className="text-sm text-gray-600 italic">
                    {step.description}
                  </p>

                  {/* Metadata - Queries */}
                  {step.metadata?.queries && step.metadata.queries.length > 0 && (
                    <div className="mt-2 flex items-start space-x-2">
                      <svg className="w-4 h-4 text-gray-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                      </svg>
                      <span className="text-xs text-gray-500">
                        {step.metadata.queries.length} {step.metadata.queries.length === 1 ? 'query' : 'queries'}
                      </span>
                    </div>
                  )}

                  {/* Metadata - Sources */}
                  {step.metadata?.sources && step.metadata.sources.length > 0 && (
                    <div className="mt-2 flex items-center space-x-2 flex-wrap">
                      {step.metadata.sources.slice(0, 5).map((source, idx) => (
                        <div key={idx} className="w-6 h-6 bg-gray-100 rounded flex items-center justify-center">
                          <span className="text-xs text-gray-600">{source.charAt(0).toUpperCase()}</span>
                        </div>
                      ))}
                      {step.metadata.sources.length > 5 && (
                        <span className="text-xs text-gray-500">
                          +{step.metadata.sources.length - 5}
                        </span>
                      )}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
