import { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { extractArtifacts, Artifact } from '../utils/artifacts';
import ArtifactsPanel from './ArtifactsPanel';
import ThinkingBubble from './ThinkingBubble';

interface MessageCanvasProps {
  content: string;
  role: 'user' | 'assistant';
}

export default function MessageCanvas({ content, role }: MessageCanvasProps) {
  const [copiedBlocks, setCopiedBlocks] = useState<Set<number>>(new Set());
  const [selectedArtifact, setSelectedArtifact] = useState<Artifact | null>(null);
  const artifacts = role === 'assistant' ? extractArtifacts(content) : [];

  // Extract thinking content from <thinking> tags
  const thinkingMatch = content.match(/<thinking>([\s\S]*?)<\/thinking>/);
  const thinkingContent = thinkingMatch ? thinkingMatch[1].trim() : null;
  const mainContent = thinkingContent
    ? content.replace(/<thinking>[\s\S]*?<\/thinking>/, '').trim()
    : content;

  const handleCopyBlock = async (text: string, blockIndex: number) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedBlocks(new Set([...copiedBlocks, blockIndex]));
      setTimeout(() => {
        setCopiedBlocks((prev) => {
          const newSet = new Set(prev);
          newSet.delete(blockIndex);
          return newSet;
        });
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  if (role === 'user') {
    return (
      <div className="bg-primary text-white rounded-xl p-4 shadow-sm">
        <div className="whitespace-pre-wrap">{content}</div>
      </div>
    );
  }

  let codeBlockIndex = 0;

  return (
    <>
      <div className="card p-6">
        {/* Thinking bubble - show if thinking content exists */}
        {thinkingContent && (
          <ThinkingBubble content={thinkingContent} />
        )}

        {/* Artifacts indicator */}
        {artifacts.length > 0 && (
          <div className="mb-4 flex flex-wrap gap-2">
            {artifacts.map((artifact) => (
              <button
                key={artifact.id}
                onClick={() => setSelectedArtifact(artifact)}
                className="flex items-center space-x-2 px-3 py-2 bg-primary bg-opacity-10 border border-primary rounded-lg hover:bg-opacity-20 transition-colors"
              >
                <svg className="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                <span className="text-sm font-medium text-primary">{artifact.title}</span>
                <svg className="w-3 h-3 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                </svg>
              </button>
            ))}
          </div>
        )}

        <ReactMarkdown
          className="prose prose-slate max-w-none
          prose-headings:text-text prose-headings:font-semibold
          prose-p:text-text-secondary prose-p:leading-relaxed
          prose-a:text-primary prose-a:no-underline hover:prose-a:underline
          prose-strong:text-text prose-strong:font-semibold
          prose-code:text-accent prose-code:bg-background-secondary prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded
          prose-pre:bg-background-secondary prose-pre:border prose-pre:border-border
          prose-blockquote:border-l-primary prose-blockquote:border-l-4 prose-blockquote:bg-background-secondary
          prose-ul:text-text-secondary prose-ol:text-text-secondary
          prose-li:marker:text-text-tertiary"
          components={{
            code({ node, inline, className, children, ...props }: any) {
              const match = /language-(\w+)/.exec(className || '');
              const codeString = String(children).replace(/\n$/, '');

              if (!inline && match) {
                const currentBlockIndex = codeBlockIndex++;
                const isCopied = copiedBlocks.has(currentBlockIndex);

                return (
                  <div className="relative group my-4">
                    <div className="absolute right-2 top-2 z-10">
                      <button
                        onClick={() => handleCopyBlock(codeString, currentBlockIndex)}
                        className="flex items-center space-x-1.5 px-3 py-1.5 text-xs btn btn-secondary opacity-0 group-hover:opacity-100 transition-opacity"
                      >
                        {isCopied ? (
                          <>
                            <svg className="w-3.5 h-3.5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                            </svg>
                            <span className="text-success font-medium">Copied</span>
                          </>
                        ) : (
                          <>
                            <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                            </svg>
                            <span>Copy</span>
                          </>
                        )}
                      </button>
                    </div>
                    <SyntaxHighlighter
                      style={vscDarkPlus as any}
                      language={match[1]}
                      PreTag="div"
                      customStyle={{
                        borderRadius: '0.75rem',
                        border: '1px solid #e9ecef',
                        padding: '1rem',
                        margin: 0,
                        backgroundColor: '#f8f9fa',
                      }}
                    >
                      {codeString}
                    </SyntaxHighlighter>
                  </div>
                );
              }

              return (
                <code className={className} {...props}>
                  {children}
                </code>
              );
            },
            h1: ({ children }) => (
              <h1 className="text-2xl font-semibold mt-6 mb-4 text-text">{children}</h1>
            ),
            h2: ({ children }) => (
              <h2 className="text-xl font-semibold mt-5 mb-3 text-text">{children}</h2>
            ),
            h3: ({ children }) => (
              <h3 className="text-lg font-semibold mt-4 mb-2 text-text">{children}</h3>
            ),
            ul: ({ children }) => (
              <ul className="list-disc list-inside space-y-1 my-3 text-text-secondary">{children}</ul>
            ),
            ol: ({ children }) => (
              <ol className="list-decimal list-inside space-y-1 my-3 text-text-secondary">{children}</ol>
            ),
            blockquote: ({ children }) => (
              <blockquote className="border-l-4 border-primary pl-4 py-2 my-3 italic text-text-secondary bg-background-secondary rounded-r-lg">
                {children}
              </blockquote>
            ),
            a: ({ href, children }) => (
              <a
                href={href}
                className="text-primary hover:text-primary-hover underline-offset-2"
                target="_blank"
                rel="noopener noreferrer"
              >
                {children}
              </a>
            ),
          }}
        >
          {mainContent}
        </ReactMarkdown>
      </div>

      {/* Artifacts Panel - Fixed overlay */}
      {selectedArtifact && (
        <div className="fixed inset-0 z-40" onClick={() => setSelectedArtifact(null)}>
          <div onClick={(e) => e.stopPropagation()}>
            <ArtifactsPanel
              isOpen={!!selectedArtifact}
              onClose={() => setSelectedArtifact(null)}
              content={selectedArtifact.content}
              title={selectedArtifact.title}
              type={selectedArtifact.type}
              language={selectedArtifact.language}
            />
          </div>
        </div>
      )}
    </>
  );
}
