import { useState, useRef, useEffect } from 'react';
import { useStore } from '../store';
import { sendMessage } from '../api';
import MessageCanvas from './MessageCanvas';

export default function ChatView() {
  const { currentConversation, messages, addMessage, isLoading, setLoading, config } = useStore();
  const [input, setInput] = useState('');
  const [dragOver, setDragOver] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || !currentConversation || isLoading) return;

    const userMessage = input.trim();
    setInput('');
    setLoading(true);

    try {
      const response = await sendMessage({
        conversation_id: currentConversation.id,
        message: userMessage,
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
    console.log('Files dropped:', files);
    // TODO: Implement file upload
    alert('File upload coming soon!');
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
    <div
      className="flex-1 flex flex-col bg-background"
      onDragOver={(e) => {
        e.preventDefault();
        setDragOver(true);
      }}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
    >
      {dragOver && (
        <div className="absolute inset-0 bg-primary bg-opacity-10 border-4 border-primary border-dashed flex items-center justify-center z-10 rounded-xl">
          <div className="text-2xl font-semibold text-primary">Drop files here</div>
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
            <div className="card p-4">
              <div className="flex space-x-2">
                <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                <div className="w-2 h-2 bg-primary rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      <div className="border-t border-border p-6 bg-surface">
        <div className="flex space-x-3 max-w-4xl mx-auto">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Type your message... (Shift+Enter for new line)"
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
  );
}
