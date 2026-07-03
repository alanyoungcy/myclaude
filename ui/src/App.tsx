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
      <div className="flex h-screen bg-gray-900 text-white items-center justify-center">
        <div className="text-center">
          <div className="text-2xl mb-4">Loading MyClaude...</div>
          <div className="text-gray-400">Initializing application</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex h-screen bg-gray-900 text-white items-center justify-center">
        <div className="text-center max-w-2xl p-8">
          <div className="text-2xl mb-4 text-red-400">Error Loading Application</div>
          <div className="text-gray-300 mb-4">{error}</div>
          <button 
            onClick={() => window.location.reload()} 
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-screen bg-gray-900 text-white">
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
