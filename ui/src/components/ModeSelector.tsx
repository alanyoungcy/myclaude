import { ChatMode, CHAT_MODES } from '../chatModes';

interface ModeSelectorProps {
  currentMode: ChatMode;
  onModeChange: (mode: ChatMode) => void;
}

export default function ModeSelector({ currentMode, onModeChange }: ModeSelectorProps) {
  const modes: ChatMode[] = ['code', 'research', 'write'];

  return (
    <div className="flex items-center space-x-2 p-3 border-b border-border bg-surface">
      <span className="text-sm text-text-secondary mr-2">Mode:</span>
      <div className="flex space-x-1">
        {modes.map((mode) => {
          const config = CHAT_MODES[mode];
          const isActive = currentMode === mode;

          return (
            <button
              key={mode}
              onClick={() => onModeChange(mode)}
              className={`
                flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-sm font-medium
                transition-all duration-200
                ${isActive
                  ? 'bg-primary text-white shadow-sm'
                  : 'bg-background-secondary text-text-secondary hover:bg-background-tertiary hover:text-text'
                }
              `}
              title={config.description}
            >
              <span className="text-base">{config.icon}</span>
              <span>{config.name}</span>
            </button>
          );
        })}
      </div>

      <div className="flex-1" />

      {/* Mode description */}
      <div className="text-xs text-text-tertiary hidden md:block">
        {CHAT_MODES[currentMode].description}
      </div>
    </div>
  );
}
