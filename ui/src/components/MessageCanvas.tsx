import { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface MessageCanvasProps {
  content: string;
  role: 'user' | 'assistant';
}

export default function MessageCanvas({ content, role }: MessageCanvasProps) {
  const [copiedBlocks, setCopiedBlocks] = useState<Set<number>>(new Set());

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
      <div className="bg-blue-600 text-white rounded-lg p-4 shadow-md">
        <div className="whitespace-pre-wrap">{content}</div>
      </div>
    );
  }

  let codeBlockIndex = 0;

  return (
    <div className="bg-gray-800 text-gray-100 rounded-lg p-6 shadow-lg border border-gray-700">
      <ReactMarkdown
        className="prose prose-invert max-w-none prose-headings:text-gray-100 prose-p:text-gray-200 prose-strong:text-gray-100 prose-code:text-blue-300 prose-code:bg-gray-900 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded"
        components={{
          code({ node, inline, className, children, ...props }) {
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
                      className="flex items-center space-x-1 px-2.5 py-1.5 text-xs bg-gray-700 hover:bg-gray-600 text-gray-200 rounded transition-all opacity-0 group-hover:opacity-100"
                    >
                      {isCopied ? (
                        <>
                          <svg className="w-3 h-3 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                          </svg>
                          <span className="text-green-400">Copied</span>
                        </>
                      ) : (
                        <>
                          <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                          </svg>
                          <span>Copy</span>
                        </>
                      )}
                    </button>
                  </div>
                  <SyntaxHighlighter
                    style={vscDarkPlus}
                    language={match[1]}
                    PreTag="div"
                    customStyle={{
                      borderRadius: '0.5rem',
                      border: '1px solid rgb(55, 65, 81)',
                      padding: '1rem',
                      margin: 0,
                    }}
                    {...props}
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
            <h1 className="text-2xl font-bold mt-6 mb-4 text-gray-100">{children}</h1>
          ),
          h2: ({ children }) => (
            <h2 className="text-xl font-bold mt-5 mb-3 text-gray-100">{children}</h2>
          ),
          h3: ({ children }) => (
            <h3 className="text-lg font-semibold mt-4 mb-2 text-gray-100">{children}</h3>
          ),
          ul: ({ children }) => (
            <ul className="list-disc list-inside space-y-1 my-3 text-gray-200">{children}</ul>
          ),
          ol: ({ children }) => (
            <ol className="list-decimal list-inside space-y-1 my-3 text-gray-200">{children}</ol>
          ),
          blockquote: ({ children }) => (
            <blockquote className="border-l-4 border-blue-500 pl-4 py-2 my-3 italic text-gray-300 bg-gray-900 rounded-r">
              {children}
            </blockquote>
          ),
          a: ({ href, children }) => (
            <a
              href={href}
              className="text-blue-400 hover:text-blue-300 underline"
              target="_blank"
              rel="noopener noreferrer"
            >
              {children}
            </a>
          ),
        }}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}
