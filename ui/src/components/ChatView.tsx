import { useState, useRef, useEffect } from 'react';
import { useStore } from '../store';
import { sendMessage } from '../api';
import MessageCanvas from './MessageCanvas';
import ModeSelector from './ModeSelector';
import { getPlaceholderForMode } from '../chatModes';
import { listen } from '@tauri-apps/api/event';
import ResearchProgress, { ResearchStep } from './ResearchProgress';

export default function ChatView() {
  const { currentConversation, messages, addMessage, isLoading, setLoading, config, chatMode, setChatMode } = useStore();
  const [input, setInput] = useState('');
  const [dragOver, setDragOver] = useState(false);
  const [uploadedFiles, setUploadedFiles] = useState<File[]>([]);
  const [loadingStep, setLoadingStep] = useState(0);
  const [researchSteps, setResearchSteps] = useState<ResearchStep[]>([]);
  const [codeAgentSteps, setCodeAgentSteps] = useState<ResearchStep[]>([]);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Listen for research logs
  useEffect(() => {
    const unlistenResearch = listen<string>('research-log', (event) => {
      console.log('Research log:', event.payload);
      const log = event.payload;

      setResearchSteps((prev) => {
        const newSteps = [...prev];

        // Parse the log message and update steps
        if (log.includes('Phase 1:')) {
          newSteps.push({
            id: 'phase1',
            name: 'Planning',
            description: log,
            status: 'running',
            startTime: Date.now(),
          });
        } else if (log.includes('Generated')) {
          const phase1 = newSteps.find(s => s.id === 'phase1');
          if (phase1) {
            phase1.status = 'completed';
            phase1.duration = Date.now() - (phase1.startTime || 0);
            phase1.description = log;
          }
        } else if (log.includes('Phase 2:')) {
          newSteps.push({
            id: 'phase2',
            name: 'Researching',
            description: 'Conducting web searches for each research question...',
            status: 'running',
            startTime: Date.now(),
          });
        } else if (log.includes('Searching:')) {
          const phase2 = newSteps.find(s => s.id === 'phase2');
          if (phase2 && phase2.status === 'running') {
            phase2.description = log;
          }
        } else if (log.includes('Phase 3:')) {
          const phase2 = newSteps.find(s => s.id === 'phase2');
          if (phase2) {
            phase2.status = 'completed';
            phase2.duration = Date.now() - (phase2.startTime || 0);
          }
          newSteps.push({
            id: 'phase3',
            name: 'Writing',
            description: 'Compiling findings into comprehensive report...',
            status: 'running',
            startTime: Date.now(),
          });
        } else if (log.includes('completed successfully')) {
          const phase3 = newSteps.find(s => s.id === 'phase3');
          if (phase3) {
            phase3.status = 'completed';
            phase3.duration = Date.now() - (phase3.startTime || 0);
            phase3.description = 'Research report generated successfully';
          }
        }

        return newSteps;
      });
    });

    const unlistenCodeAgent = listen<string>('code-agent-log', (event) => {
      console.log('Code agent log:', event.payload);
      const log = event.payload;

      setCodeAgentSteps((prev) => {
        const newSteps = [...prev];

        // Parse code agent logs
        if (log.includes('starting')) {
          newSteps.push({
            id: 'init',
            name: 'Initializing',
            description: log,
            status: 'completed',
            startTime: Date.now(),
            duration: 100,
          });
        } else if (log.includes('iteration')) {
          const iterMatch = log.match(/iteration (\d+)/);
          if (iterMatch) {
            const iter = iterMatch[1];
            newSteps.push({
              id: `iter${iter}`,
              name: `Iteration ${iter}`,
              description: 'Analyzing task and planning next action...',
              status: 'running',
              startTime: Date.now(),
            });
          }
        } else if (log.includes('requested') && log.includes('tool')) {
          const lastStep = newSteps[newSteps.length - 1];
          if (lastStep && lastStep.status === 'running') {
            lastStep.status = 'completed';
            lastStep.duration = Date.now() - (lastStep.startTime || 0);
            lastStep.description = log;
          }
        } else if (log.includes('Executing tool:')) {
          const toolMatch = log.match(/Executing tool: (\w+)/);
          if (toolMatch) {
            const toolName = toolMatch[1];
            newSteps.push({
              id: `tool-${Date.now()}`,
              name: `Tool: ${toolName}`,
              description: log,
              status: 'running',
              startTime: Date.now(),
            });
          }
        } else if (log.includes('Reading file:') || log.includes('Writing file:') || log.includes('Listing directory:') || log.includes('Executing command:')) {
          const lastStep = newSteps[newSteps.length - 1];
          if (lastStep && lastStep.status === 'running') {
            lastStep.description = log;
          }
        } else if (log.includes('completed task')) {
          const lastStep = newSteps[newSteps.length - 1];
          if (lastStep && lastStep.status === 'running') {
            lastStep.status = 'completed';
            lastStep.duration = Date.now() - (lastStep.startTime || 0);
          }
          newSteps.push({
            id: 'complete',
            name: 'Completed',
            description: 'Code agent finished task successfully',
            status: 'completed',
            startTime: Date.now(),
            duration: 100,
          });
        }

        return newSteps;
      });
    });

    return () => {
      unlistenResearch.then((fn) => fn());
      unlistenCodeAgent.then((fn) => fn());
    };
  }, []);

  // Progress steps cycle
  useEffect(() => {
    if (!isLoading) {
      setLoadingStep(0);
      setResearchSteps([]);
      setCodeAgentSteps([]);
      return;
    }

    const steps = [
      'Processing your request...',
      'Analyzing context...',
      'Searching for relevant information...',
      'Generating response...',
      'Finalizing output...',
    ];

    const interval = setInterval(() => {
      setLoadingStep((prev) => (prev + 1) % steps.length);
    }, 2000);

    return () => clearInterval(interval);
  }, [isLoading]);

  const handleSend = async () => {
    if (!input.trim() || !currentConversation || isLoading) return;

    const userMessage = input.trim();
    setInput('');
    setLoading(true);

    try {
      const response = await sendMessage({
        conversation_id: currentConversation.id,
        message: userMessage,
        mode: chatMode,
      });

      addMessage(response.message);
      addMessage(response.assistant_message);
    } catch (error) {
      console.error('Failed to send message:', error);
      alert('Failed to send message: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length > 0) {
      setUploadedFiles(prev => [...prev, ...files]);
      console.log('Files dropped:', files.map(f => f.name));
    }
  };

  const removeFile = (index: number) => {
    setUploadedFiles(prev => prev.filter((_, i) => i !== index));
  };

  if (!config?.api_key || !config?.base_url) {
    return (
      <div className="flex-1 flex items-center justify-center bg-background">
        <div className="text-center max-w-md">
          <div className="mb-6">
            <svg className="w-16 h-16 mx-auto text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
            </svg>
          </div>
          <h2 className="text-2xl font-semibold mb-4 text-text">Welcome to MyClaude</h2>
          <p className="text-text-secondary mb-2">Please configure your API settings to get started.</p>
          <p className="text-sm text-text-tertiary">Click Settings in the sidebar to configure.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col bg-background">
      <div
        className="flex-1 flex flex-col"
        onDragOver={(e) => {
          e.preventDefault();
          setDragOver(true);
        }}
        onDragLeave={() => setDragOver(false)}
        onDrop={handleDrop}
      >
        {dragOver && (
          <div className="absolute inset-0 bg-primary bg-opacity-10 border-4 border-primary border-dashed flex items-center justify-center z-10 rounded-xl">
            <div className="text-center">
              <svg className="w-16 h-16 mx-auto mb-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
              </svg>
              <div className="text-2xl font-semibold text-primary">Drop files here</div>
              <div className="text-sm text-text-secondary mt-2">Supported: PDF, TXT, MD, DOCX</div>
            </div>
          </div>
        )}

      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {messages.length === 0 && (
          <div className="text-center text-text-secondary mt-20">
            <svg className="w-12 h-12 mx-auto mb-4 text-primary opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
            </svg>
            <p className="text-xl font-medium mb-2">Start a conversation</p>
            <p className="text-sm">Claude can search the web when needed</p>
          </div>
        )}

        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'} animate-in`}
          >
            <div className="max-w-3xl w-full">
              <MessageCanvas content={msg.content} role={msg.role as 'user' | 'assistant'} />
            </div>
          </div>
        ))}

        {isLoading && (
          <div className="flex justify-start animate-in">
            {chatMode === 'research' && researchSteps.length > 0 ? (
              <div className="w-full max-w-3xl">
                <ResearchProgress steps={researchSteps} />
              </div>
            ) : chatMode === 'code' && codeAgentSteps.length > 0 ? (
              <div className="w-full max-w-3xl">
                <ResearchProgress steps={codeAgentSteps} />
              </div>
            ) : (
              <div className="card p-4">
                <div className="flex items-center space-x-3">
                  <div className="flex space-x-2">
                    <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                    <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                    <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
                  </div>
                  <span className="text-sm text-text-secondary">
                    {['Processing your request...', 'Analyzing context...', 'Searching for relevant information...', 'Generating response...', 'Finalizing output...'][loadingStep]}
                  </span>
                </div>
              </div>
            )}
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      <div className="border-t border-border p-6 bg-surface">
        <div className="max-w-4xl mx-auto space-y-3">
          {/* Mode Selector - moved to bottom */}
          <ModeSelector currentMode={chatMode} onModeChange={setChatMode} />

          {/* Uploaded files display */}
          {uploadedFiles.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {uploadedFiles.map((file, index) => (
                <div
                  key={index}
                  className="flex items-center space-x-2 px-3 py-2 bg-background-secondary rounded-lg border border-border"
                >
                  <svg className="w-4 h-4 text-text-tertiary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
                  </svg>
                  <span className="text-sm text-text-secondary">{file.name}</span>
                  <button
                    onClick={() => removeFile(index)}
                    className="text-text-tertiary hover:text-error transition-colors"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              ))}
            </div>
          )}

          <div className="flex space-x-3">
            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder={getPlaceholderForMode(chatMode)}
              className="input flex-1 resize-none min-h-[60px]"
              rows={2}
              disabled={isLoading}
            />
            <button
              onClick={handleSend}
              disabled={!input.trim() || isLoading}
              className="btn btn-primary px-8 self-end"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
    </div>
  );
}
