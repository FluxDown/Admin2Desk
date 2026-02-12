import { useEffect, useRef } from 'react';
import { Terminal as XTerminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';

interface TerminalProps {
  targetId: string;
  credentialId: string;
  token: string;
  onDisconnect?: () => void;
}

export default function Terminal({ targetId, credentialId, token, onDisconnect }: TerminalProps) {
  const termRef = useRef<HTMLDivElement>(null);
  const terminalRef = useRef<XTerminal | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!termRef.current) return;

    const term = new XTerminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: "'Cascadia Code', 'Fira Code', 'Consolas', monospace",
      theme: {
        background: '#1a1a2e',
        foreground: '#e0e0e0',
        cursor: '#00d4ff',
        selectionBackground: '#3a3a5e',
      },
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(termRef.current);
    fitAddon.fit();

    terminalRef.current = term;

    term.writeln('\x1b[36mConnexion au serveur...\x1b[0m');

    // Build WebSocket URL
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = window.location.host;
    const wsUrl = `${protocol}//${host}/api/sessions/connect?token=${encodeURIComponent(token)}&target_id=${encodeURIComponent(targetId)}&credential_id=${encodeURIComponent(credentialId)}`;

    const ws = new WebSocket(wsUrl);
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      term.writeln('\x1b[32mConnecte!\x1b[0m\r\n');
    };

    ws.onmessage = (event) => {
      const data = new Uint8Array(event.data);
      term.write(data);
    };

    ws.onclose = () => {
      term.writeln('\r\n\x1b[33m--- Session terminee ---\x1b[0m');
      onDisconnect?.();
    };

    ws.onerror = () => {
      term.writeln('\r\n\x1b[31mErreur de connexion WebSocket\x1b[0m');
    };

    // Forward terminal input to WebSocket
    term.onData((data) => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(new TextEncoder().encode(data));
      }
    });

    // Handle terminal resize
    term.onResize(({ cols, rows }) => {
      if (ws.readyState === WebSocket.OPEN) {
        const msg = new TextEncoder().encode(
          JSON.stringify({ type: 'resize', cols, rows })
        );
        const buf = new Uint8Array(1 + msg.length);
        buf[0] = 0x02; // Control message prefix
        buf.set(msg, 1);
        ws.send(buf);
      }
    });

    // Fit on window resize
    const handleResize = () => fitAddon.fit();
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      ws.close();
      term.dispose();
    };
  }, [targetId, credentialId, token, onDisconnect]);

  return (
    <div
      ref={termRef}
      className="w-full h-full min-h-[400px] rounded-lg overflow-hidden"
    />
  );
}
