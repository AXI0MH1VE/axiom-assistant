import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './styles/axiom_dark.css';

type Message = { role: 'user' | 'assistant'; content: string };

function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [streaming, setStreaming] = useState('');

  useEffect(() => {
    const unlisten = listen('token', (event: any) => {
      setStreaming(prev => prev + event.payload);
    });
    return () => { unlisten.then((fn:any) => fn()); };
  }, []);

  const sendMessage = async () => {
    setMessages([...messages, { role: 'user', content: input }]);
    const msg = input;
    setInput('');
    setStreaming('');
    await invoke('send_message', { message: msg });
    setMessages(prev => [...prev, { role: 'assistant', content: streaming }]);
    setStreaming('');
  };

  return (
    <div className="axiom-container">
      <div className="chat-window">
        {messages.map((msg, idx) => (
          <div key={idx} className={`message ${msg.role}`}>
            {msg.content}
          </div>
        ))}
        {streaming && (
          <div className="message assistant streaming">
            {streaming}
            <span className="cursor">▋</span>
          </div>
        )}
      </div>
      <div className="input-bar">
        <input
          value={input}
          onChange={(e) => setInput((e.target as HTMLInputElement).value)}
          onKeyPress={(e) => (e.key === 'Enter') && sendMessage()}
          placeholder="Ask Axiom Assistant..."
        />
      </div>
    </div>
  );
}

export default App;
