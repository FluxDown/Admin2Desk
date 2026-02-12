import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { listTargets } from '../api/targets';
import { listSessions } from '../api/sessions';
import type { SshTarget, Session } from '../types';

export default function DashboardPage() {
  const { user } = useAuth();
  const [targets, setTargets] = useState<SshTarget[]>([]);
  const [sessions, setSessions] = useState<Session[]>([]);

  useEffect(() => {
    listTargets().then(setTargets).catch(console.error);
    listSessions().then(setSessions).catch(console.error);
  }, []);

  const activeSessions = sessions.filter((s) => s.status === 'active').length;
  const recentSessions = sessions.slice(0, 5);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Bonjour, {user?.username}</h1>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
          <p className="text-sm text-gray-400">Cibles SSH</p>
          <p className="text-3xl font-bold text-cyan-400 mt-2">{targets.length}</p>
        </div>
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
          <p className="text-sm text-gray-400">Sessions actives</p>
          <p className="text-3xl font-bold text-green-400 mt-2">{activeSessions}</p>
        </div>
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
          <p className="text-sm text-gray-400">Total sessions</p>
          <p className="text-3xl font-bold text-gray-300 mt-2">{sessions.length}</p>
        </div>
      </div>

      {/* Quick connect */}
      <h2 className="text-lg font-semibold mb-4">Connexion rapide</h2>
      {targets.length === 0 ? (
        <p className="text-gray-400">
          Aucune cible configuree.{' '}
          <Link to="/targets" className="text-cyan-400 hover:underline">
            Ajouter une cible
          </Link>
        </p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
          {targets.map((target) => (
            <Link
              key={target.id}
              to={`/connect/${target.id}`}
              className="bg-gray-800 border border-gray-700 hover:border-cyan-500 rounded-lg p-4 transition-colors"
            >
              <h3 className="font-medium text-gray-100">{target.name}</h3>
              <p className="text-sm text-gray-400 mt-1">
                {target.hostname}:{target.port}
              </p>
              {target.description && (
                <p className="text-xs text-gray-500 mt-2">{target.description}</p>
              )}
            </Link>
          ))}
        </div>
      )}

      {/* Recent sessions */}
      {recentSessions.length > 0 && (
        <>
          <h2 className="text-lg font-semibold mb-4">Sessions recentes</h2>
          <div className="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
            <table className="w-full text-sm">
              <thead className="bg-gray-750">
                <tr className="border-b border-gray-700">
                  <th className="text-left p-3 text-gray-400">Debut</th>
                  <th className="text-left p-3 text-gray-400">Statut</th>
                  <th className="text-left p-3 text-gray-400">Fin</th>
                </tr>
              </thead>
              <tbody>
                {recentSessions.map((session) => (
                  <tr key={session.id} className="border-b border-gray-700/50">
                    <td className="p-3">{new Date(session.started_at).toLocaleString('fr-FR')}</td>
                    <td className="p-3">
                      <span
                        className={`px-2 py-1 rounded text-xs ${
                          session.status === 'active'
                            ? 'bg-green-900 text-green-300'
                            : session.status === 'completed'
                            ? 'bg-blue-900 text-blue-300'
                            : 'bg-red-900 text-red-300'
                        }`}
                      >
                        {session.status}
                      </span>
                    </td>
                    <td className="p-3 text-gray-400">
                      {session.ended_at
                        ? new Date(session.ended_at).toLocaleString('fr-FR')
                        : '-'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </>
      )}
    </div>
  );
}
