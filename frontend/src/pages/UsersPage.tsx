import { useEffect, useState } from 'react';
import api from '../api/client';
import type { User } from '../types';

export default function UsersPage() {
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    api.get<User[]>('/api/users').then((res) => setUsers(res.data)).catch(console.error);
  }, []);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Utilisateurs</h1>

      <div className="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-700">
              <th className="text-left p-3 text-gray-400">Nom</th>
              <th className="text-left p-3 text-gray-400">Email</th>
              <th className="text-left p-3 text-gray-400">Role</th>
              <th className="text-left p-3 text-gray-400">Inscription</th>
            </tr>
          </thead>
          <tbody>
            {users.map((user) => (
              <tr key={user.id} className="border-b border-gray-700/50">
                <td className="p-3 font-medium">{user.username}</td>
                <td className="p-3 text-gray-300">{user.email}</td>
                <td className="p-3">
                  <span
                    className={`px-2 py-1 rounded text-xs ${
                      user.role === 'admin'
                        ? 'bg-purple-900 text-purple-300'
                        : 'bg-gray-700 text-gray-300'
                    }`}
                  >
                    {user.role}
                  </span>
                </td>
                <td className="p-3 text-gray-400">
                  {new Date(user.created_at).toLocaleDateString('fr-FR')}
                </td>
              </tr>
            ))}
            {users.length === 0 && (
              <tr>
                <td colSpan={4} className="p-6 text-center text-gray-400">
                  Aucun utilisateur
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
