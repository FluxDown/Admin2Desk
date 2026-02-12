import { useEffect, useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { getTarget, listCredentials, createCredential } from '../api/targets';
import Terminal from '../components/Terminal';
import type { SshTarget, SshCredential } from '../types';

export default function ConnectPage() {
  const { targetId } = useParams<{ targetId: string }>();
  const { token } = useAuth();
  const [target, setTarget] = useState<SshTarget | null>(null);
  const [credentials, setCredentials] = useState<SshCredential[]>([]);
  const [selectedCred, setSelectedCred] = useState('');
  const [connected, setConnected] = useState(false);
  const [showAddCred, setShowAddCred] = useState(false);
  const [credUsername, setCredUsername] = useState('');
  const [credAuthType, setCredAuthType] = useState('password');
  const [credValue, setCredValue] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    if (!targetId) return;
    getTarget(targetId).then(setTarget).catch(console.error);
    loadCredentials();
  }, [targetId]);

  const loadCredentials = () => {
    if (!targetId) return;
    listCredentials(targetId).then((creds) => {
      setCredentials(creds);
      if (creds.length > 0 && !selectedCred) {
        setSelectedCred(creds[0].id);
      }
    }).catch(console.error);
  };

  const handleConnect = () => {
    if (!selectedCred) {
      setError('Selectionnez un credential');
      return;
    }
    setConnected(true);
  };

  const handleAddCred = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!targetId) return;
    try {
      await createCredential(targetId, {
        username: credUsername,
        auth_type: credAuthType,
        credential: credValue,
      });
      setShowAddCred(false);
      setCredUsername('');
      setCredValue('');
      loadCredentials();
    } catch (err: any) {
      setError(err.response?.data?.error || 'Erreur');
    }
  };

  if (!target) {
    return <div className="text-gray-400">Chargement...</div>;
  }

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <div>
          <Link to="/targets" className="text-cyan-400 text-sm hover:underline">
            &larr; Retour aux cibles
          </Link>
          <h1 className="text-2xl font-bold mt-1">{target.name}</h1>
          <p className="text-sm text-gray-400">
            {target.hostname}:{target.port}
          </p>
        </div>
      </div>

      {!connected ? (
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
          {error && (
            <div className="mb-4 p-3 bg-red-900/50 border border-red-700 rounded text-red-300 text-sm">
              {error}
            </div>
          )}

          <h2 className="text-lg font-semibold mb-4">Selectionnez un credential</h2>

          {credentials.length === 0 ? (
            <p className="text-gray-400 mb-4">
              Aucun credential pour cette cible. Ajoutez-en un ci-dessous.
            </p>
          ) : (
            <div className="space-y-2 mb-4">
              {credentials.map((cred) => (
                <label
                  key={cred.id}
                  className={`flex items-center p-3 rounded border cursor-pointer transition-colors ${
                    selectedCred === cred.id
                      ? 'border-cyan-500 bg-cyan-900/20'
                      : 'border-gray-700 hover:border-gray-500'
                  }`}
                >
                  <input
                    type="radio"
                    name="credential"
                    value={cred.id}
                    checked={selectedCred === cred.id}
                    onChange={() => setSelectedCred(cred.id)}
                    className="mr-3"
                  />
                  <div>
                    <span className="font-medium">{cred.username}</span>
                    <span className="ml-2 px-2 py-0.5 text-xs bg-gray-700 rounded">
                      {cred.auth_type}
                    </span>
                  </div>
                </label>
              ))}
            </div>
          )}

          <div className="flex gap-3">
            <button
              onClick={handleConnect}
              disabled={!selectedCred}
              className="px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 rounded transition-colors"
            >
              Connecter
            </button>
            <button
              onClick={() => setShowAddCred(!showAddCred)}
              className="px-6 py-2 bg-gray-700 hover:bg-gray-600 rounded transition-colors"
            >
              {showAddCred ? 'Annuler' : 'Ajouter credential'}
            </button>
          </div>

          {showAddCred && (
            <form onSubmit={handleAddCred} className="mt-4 p-4 bg-gray-750 rounded border border-gray-600 space-y-3">
              <div>
                <label className="block text-sm text-gray-400 mb-1">Nom d'utilisateur SSH</label>
                <input
                  type="text"
                  value={credUsername}
                  onChange={(e) => setCredUsername(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
                  required
                />
              </div>
              <div>
                <label className="block text-sm text-gray-400 mb-1">Type</label>
                <select
                  value={credAuthType}
                  onChange={(e) => setCredAuthType(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
                >
                  <option value="password">Mot de passe</option>
                  <option value="key">Cle privee</option>
                </select>
              </div>
              <div>
                <label className="block text-sm text-gray-400 mb-1">
                  {credAuthType === 'password' ? 'Mot de passe' : 'Cle privee (PEM)'}
                </label>
                {credAuthType === 'password' ? (
                  <input
                    type="password"
                    value={credValue}
                    onChange={(e) => setCredValue(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500"
                    required
                  />
                ) : (
                  <textarea
                    value={credValue}
                    onChange={(e) => setCredValue(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-gray-100 focus:outline-none focus:border-cyan-500 font-mono text-xs"
                    rows={8}
                    required
                  />
                )}
              </div>
              <button
                type="submit"
                className="px-4 py-2 bg-cyan-600 hover:bg-cyan-700 rounded text-sm transition-colors"
              >
                Enregistrer
              </button>
            </form>
          )}
        </div>
      ) : (
        <div className="flex-1 bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
          <Terminal
            targetId={targetId!}
            credentialId={selectedCred}
            token={token!}
            onDisconnect={() => setConnected(false)}
          />
        </div>
      )}
    </div>
  );
}
