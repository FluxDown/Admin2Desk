import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { listTargets, createTarget, deleteTarget } from '../api/targets';
import type { SshTarget } from '../types';

export default function TargetsPage() {
  const { isAdmin } = useAuth();
  const [targets, setTargets] = useState<SshTarget[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [name, setName] = useState('');
  const [hostname, setHostname] = useState('');
  const [port, setPort] = useState('22');
  const [description, setDescription] = useState('');
  const [error, setError] = useState('');

  const load = () => listTargets().then(setTargets).catch(console.error);

  useEffect(() => {
    load();
  }, []);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    try {
      await createTarget({
        name,
        hostname,
        port: parseInt(port) || 22,
        description: description || undefined,
      });
      setShowForm(false);
      setName('');
      setHostname('');
      setPort('22');
      setDescription('');
      load();
    } catch (err: any) {
      setError(err.response?.data?.error || 'Erreur');
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Supprimer cette cible ?')) return;
    try {
      await deleteTarget(id);
      load();
    } catch (err: any) {
      alert(err.response?.data?.error || 'Erreur');
    }
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Cibles SSH</h1>
        <button
          onClick={() => setShowForm(!showForm)}
          className="px-4 py-2 bg-cyan-600 hover:bg-cyan-700 rounded text-sm transition-colors"
        >
          {showForm ? 'Annuler' : 'Nouvelle cible'}
        </button>
      </div>

      {showForm && (
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6 mb-6">
          {error && (
            <div className="mb-4 p-3 bg-red-900/50 border border-red-700 rounded text-red-300 text-sm">
              {error}
            </div>
          )}
          <form onSubmit={handleCreate} className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm text-gray-400 mb-1">Nom</label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
                required
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Hostname</label>
              <input
                type="text"
                value={hostname}
                onChange={(e) => setHostname(e.target.value)}
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
                required
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Port</label>
              <input
                type="number"
                value={port}
                onChange={(e) => setPort(e.target.value)}
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Description</label>
              <input
                type="text"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
              />
            </div>
            <div className="md:col-span-2">
              <button
                type="submit"
                className="px-6 py-2 bg-cyan-600 hover:bg-cyan-700 rounded text-sm transition-colors"
              >
                Creer
              </button>
            </div>
          </form>
        </div>
      )}

      <div className="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-700">
              <th className="text-left p-3 text-gray-400">Nom</th>
              <th className="text-left p-3 text-gray-400">Hote</th>
              <th className="text-left p-3 text-gray-400">Port</th>
              <th className="text-left p-3 text-gray-400">Description</th>
              <th className="text-right p-3 text-gray-400">Actions</th>
            </tr>
          </thead>
          <tbody>
            {targets.map((target) => (
              <tr key={target.id} className="border-b border-gray-700/50 hover:bg-gray-750">
                <td className="p-3 font-medium">{target.name}</td>
                <td className="p-3 text-gray-300">{target.hostname}</td>
                <td className="p-3 text-gray-300">{target.port}</td>
                <td className="p-3 text-gray-400">{target.description || '-'}</td>
                <td className="p-3 text-right space-x-2">
                  <Link
                    to={`/connect/${target.id}`}
                    className="px-3 py-1 bg-green-600 hover:bg-green-700 rounded text-xs transition-colors"
                  >
                    Connecter
                  </Link>
                  {isAdmin && (
                    <button
                      onClick={() => handleDelete(target.id)}
                      className="px-3 py-1 bg-red-600 hover:bg-red-700 rounded text-xs transition-colors"
                    >
                      Supprimer
                    </button>
                  )}
                </td>
              </tr>
            ))}
            {targets.length === 0 && (
              <tr>
                <td colSpan={5} className="p-6 text-center text-gray-400">
                  Aucune cible configuree
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
