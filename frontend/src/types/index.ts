export interface User {
  id: string;
  username: string;
  email: string;
  role: 'admin' | 'user';
  created_at: string;
}

export interface SshTarget {
  id: string;
  name: string;
  hostname: string;
  port: number;
  description: string | null;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface SshCredential {
  id: string;
  target_id: string;
  username: string;
  auth_type: 'password' | 'key';
  created_at: string;
}

export interface Session {
  id: string;
  user_id: string;
  target_id: string;
  credential_id: string;
  started_at: string;
  ended_at: string | null;
  status: 'active' | 'completed' | 'failed' | 'terminated';
  recording_path: string | null;
}

export interface AuditLog {
  id: string;
  user_id: string | null;
  action: string;
  details: Record<string, unknown> | null;
  ip_address: string;
  created_at: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}
