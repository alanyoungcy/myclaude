import { useEffect, useState } from 'react';
import { useStore } from './store';
import { getConfig, getConversations, createConversation, getConversation } from './api';
import Sidebar from './components/Sidebar';
import ChatView from './components/ChatView';
import Settings from './components/Settings';
import PromptsManager from './components/PromptsManager';
import SkillsManager from './components/SkillsManager';

function App() {
  const { setConfig, setConversations, currentConversation, setCurrentConversation, setMessages } = useStore();
  const [showSettings, setShowSettings] = useState(false);
  const [showPrompts, setShowPrompts] = useState(false);
  const [showSkills, setShowSkills] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    console.log('App mounting, loading initial data...');
    
    const loadData = async () => {
      try {
        console.log('Loading config...');
        const config = await getConfig();
        console.log('Config loaded:', config);
        setConfig(config);

        console.log('Loading conversations...');
        const conversations = await getConversations();
        console.log('Conversations loaded:', conversations.length);
        setConversations(conversations);

        if (!currentConversation) {
          if (conversations.length > 0) {
            console.log('Loading first conversation...');
            const conv = await getConversation(conversations[0].id);
            if (conv) {
              setCurrentConversation(conv.conversation);
              setMessages(conv.messages);
              console.log('Loaded conversation with', conv.messages.length, 'messages');
            }
          } else {
            console.log('Creating initial conversation...');
            const newConv = await createConversation('New Chat');
            setCurrentConversation(newConv);
            setMessages([]);
            setConversations([newConv]);
            console.log('Initial conversation created');
          }
        }
        
        setIsLoading(false);
        console.log('App initialization complete');
      } catch (error) {
        console.error('Failed to load initial data:', error);
        setError(String(error));
        setIsLoading(false);
      }
    };

    loadData();
  }, []);

  if (isLoading) {
    return (
      <div className="flex h-screen bg-background text-text items-center justify-center">
        <div className="text-center">
          <div className="spinner w-12 h-12 mb-4 mx-auto"></div>
          <div className="text-xl font-medium mb-2">Loading MyClaude...</div>
          <div className="text-text-secondary">Initializing application</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex h-screen bg-background text-text items-center justify-center">
        <div className="text-center max-w-2xl p-8">
          <div className="w-16 h-16 mx-auto mb-4 text-error">
            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <div className="text-2xl font-semibold mb-4 text-error">Error Loading Application</div>
          <div className="text-text-secondary mb-6">{error}</div>
          <button
            onClick={() => window.location.reload()}
            className="btn btn-primary"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-screen bg-background text-text">
      <Sidebar
        onOpenSettings={() => setShowSettings(true)}
        onOpenPrompts={() => setShowPrompts(true)}
        onOpenSkills={() => setShowSkills(true)}
      />
      <ChatView />

      {showSettings && <Settings onClose={() => setShowSettings(false)} />}
      {showPrompts && <PromptsManager onClose={() => setShowPrompts(false)} />}
      {showSkills && <SkillsManager onClose={() => setShowSkills(false)} />}
    </div>
  );
}

export default App;
