import { Link, Outlet, useLocation } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

const navItems = [
  { path: '/', label: 'Dashboard' },
  { path: '/targets', label: 'Cibles SSH' },
  { path: '/sessions', label: 'Sessions' },
];

const adminItems = [
  { path: '/audit-logs', label: 'Audit Logs' },
  { path: '/users', label: 'Utilisateurs' },
];

export default function Layout() {
  const { user, logout, isAdmin } = useAuth();
  const location = useLocation();

  return (
    <div className="min-h-screen flex bg-gray-900 text-gray-100">
      {/* Sidebar */}
      <aside className="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
        <div className="p-6 border-b border-gray-700">
          <h1 className="text-xl font-bold text-cyan-400">Bastion SSH</h1>
          <p className="text-sm text-gray-400 mt-1">{user?.username}</p>
        </div>

        <nav className="flex-1 p-4 space-y-1">
          {navItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={`block px-4 py-2 rounded transition-colors ${
                location.pathname === item.path
                  ? 'bg-cyan-600 text-white'
                  : 'text-gray-300 hover:bg-gray-700'
              }`}
            >
              {item.label}
            </Link>
          ))}

          {isAdmin && (
            <>
              <div className="pt-4 pb-2 px-4 text-xs uppercase text-gray-500 tracking-wider">
                Administration
              </div>
              {adminItems.map((item) => (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`block px-4 py-2 rounded transition-colors ${
                    location.pathname === item.path
                      ? 'bg-cyan-600 text-white'
                      : 'text-gray-300 hover:bg-gray-700'
                  }`}
                >
                  {item.label}
                </Link>
              ))}
            </>
          )}
        </nav>

        <div className="p-4 border-t border-gray-700">
          <button
            onClick={logout}
            className="w-full px-4 py-2 text-sm bg-red-600 hover:bg-red-700 rounded transition-colors"
          >
            Deconnexion
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 p-8 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}
