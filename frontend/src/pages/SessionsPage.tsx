import { useEffect, useState } from 'react';
import { listSessions } from '../api/sessions';
import type { Session } from '../types';

export default function SessionsPage() {
  const [sessions, setSessions] = useState<Session[]>([]);

  useEffect(() => {
    listSessions().then(setSessions).catch(console.error);
  }, []);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Historique des sessions</h1>

      <div className="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-700">
              <th className="text-left p-3 text-gray-400">ID</th>
              <th className="text-left p-3 text-gray-400">Debut</th>
              <th className="text-left p-3 text-gray-400">Fin</th>
              <th className="text-left p-3 text-gray-400">Statut</th>
            </tr>
          </thead>
          <tbody>
            {sessions.map((session) => (
              <tr key={session.id} className="border-b border-gray-700/50">
                <td className="p-3 font-mono text-xs text-gray-400">
                  {session.id.slice(0, 8)}...
                </td>
                <td className="p-3">
                  {new Date(session.started_at).toLocaleString('fr-FR')}
                </td>
                <td className="p-3 text-gray-400">
                  {session.ended_at
                    ? new Date(session.ended_at).toLocaleString('fr-FR')
                    : '-'}
                </td>
                <td className="p-3">
                  <span
                    className={`px-2 py-1 rounded text-xs ${
                      session.status === 'active'
                        ? 'bg-green-900 text-green-300'
                        : session.status === 'completed'
                        ? 'bg-blue-900 text-blue-300'
                        : session.status === 'failed'
                        ? 'bg-red-900 text-red-300'
                        : 'bg-yellow-900 text-yellow-300'
                    }`}
                  >
                    {session.status}
                  </span>
                </td>
              </tr>
            ))}
            {sessions.length === 0 && (
              <tr>
                <td colSpan={4} className="p-6 text-center text-gray-400">
                  Aucune session
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
