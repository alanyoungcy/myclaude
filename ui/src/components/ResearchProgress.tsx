import { useState } from 'react';

export interface ResearchStep {
  id: string;
  name: string;
  description: string;
  status: 'pending' | 'running' | 'completed' | 'error';
  duration?: number;
  startTime?: number;
}

interface ResearchProgressProps {
  steps: ResearchStep[];
  isCollapsed?: boolean;
}

export default function ResearchProgress({ steps, isCollapsed: initialCollapsed = false }: ResearchProgressProps) {
  const [isCollapsed, setIsCollapsed] = useState(initialCollapsed);

  const totalDuration = steps
    .filter(s => s.status === 'completed' && s.duration)
    .reduce((sum, s) => sum + (s.duration || 0), 0);

  const getStatusIcon = (status: ResearchStep['status']) => {
    switch (status) {
      case 'completed':
        return (
          <div className="w-5 h-5 rounded-full bg-green-100 flex items-center justify-center">
            <svg className="w-3 h-3 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
            </svg>
          </div>
        );
      case 'running':
        return (
          <div className="w-5 h-5 rounded-full border-2 border-blue-500 border-t-transparent animate-spin"></div>
        );
      case 'error':
        return (
          <div className="w-5 h-5 rounded-full bg-red-100 flex items-center justify-center">
            <svg className="w-3 h-3 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </div>
        );
      default:
        return (
          <div className="w-5 h-5 rounded-full border-2 border-gray-300"></div>
        );
    }
  };

  return (
    <div className="my-4 bg-white border border-gray-200 rounded-lg overflow-hidden shadow-sm">
      {/* Header */}
      <button
        onClick={() => setIsCollapsed(!isCollapsed)}
        className="w-full flex items-center justify-between px-4 py-3 bg-gray-50 hover:bg-gray-100 transition-colors"
      >
        <div className="flex items-center space-x-3">
          <svg
            className={`w-4 h-4 text-gray-600 transition-transform ${isCollapsed ? '' : 'rotate-90'}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
          <span className="font-semibold text-gray-900">Reasoning Trace</span>
          {totalDuration > 0 && (
            <span className="text-sm text-gray-500">({(totalDuration / 1000).toFixed(1)}s)</span>
          )}
        </div>
        <div className="flex items-center space-x-2">
          {steps.some(s => s.status === 'running') && (
            <div className="flex items-center space-x-2 text-sm text-blue-600">
              <div className="w-2 h-2 bg-blue-600 rounded-full animate-pulse"></div>
              <span>researching...</span>
            </div>
          )}
        </div>
      </button>

      {/* Steps */}
      {!isCollapsed && (
        <div className="px-4 py-3 space-y-4">
          {steps.map((step) => (
            <div key={step.id} className="flex space-x-3">
              <div className="flex-shrink-0 mt-0.5">
                {getStatusIcon(step.status)}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center space-x-2 mb-1">
                  <span className="font-medium text-gray-900">{step.name}</span>
                  {step.status === 'completed' && step.duration && (
                    <span className="text-xs text-gray-500">
                      {(step.duration / 1000).toFixed(1)}s
                    </span>
                  )}
                </div>
                <p className="text-sm text-gray-600 leading-relaxed italic">
                  {step.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
