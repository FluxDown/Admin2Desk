import { useEffect, useState } from 'react';
import { listAuditLogs } from '../api/audit';
import type { AuditLog } from '../types';

export default function AuditLogsPage() {
  const [logs, setLogs] = useState<AuditLog[]>([]);

  useEffect(() => {
    listAuditLogs().then(setLogs).catch(console.error);
  }, []);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Logs d'audit</h1>

      <div className="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-700">
              <th className="text-left p-3 text-gray-400">Date</th>
              <th className="text-left p-3 text-gray-400">Action</th>
              <th className="text-left p-3 text-gray-400">Utilisateur</th>
              <th className="text-left p-3 text-gray-400">IP</th>
              <th className="text-left p-3 text-gray-400">Details</th>
            </tr>
          </thead>
          <tbody>
            {logs.map((log) => (
              <tr key={log.id} className="border-b border-gray-700/50">
                <td className="p-3 text-gray-300">
                  {new Date(log.created_at).toLocaleString('fr-FR')}
                </td>
                <td className="p-3">
                  <span className="px-2 py-1 bg-gray-700 rounded text-xs font-mono">
                    {log.action}
                  </span>
                </td>
                <td className="p-3 text-gray-400 font-mono text-xs">
                  {log.user_id ? log.user_id.slice(0, 8) + '...' : 'system'}
                </td>
                <td className="p-3 text-gray-400 font-mono text-xs">{log.ip_address}</td>
                <td className="p-3 text-gray-500 text-xs font-mono max-w-xs truncate">
                  {log.details ? JSON.stringify(log.details) : '-'}
                </td>
              </tr>
            ))}
            {logs.length === 0 && (
              <tr>
                <td colSpan={5} className="p-6 text-center text-gray-400">
                  Aucun log
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
