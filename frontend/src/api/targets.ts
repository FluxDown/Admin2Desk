import api from './client';
import type { SshTarget, SshCredential } from '../types';

export async function listTargets(): Promise<SshTarget[]> {
  const res = await api.get<SshTarget[]>('/api/targets');
  return res.data;
}

export async function getTarget(id: string): Promise<SshTarget> {
  const res = await api.get<SshTarget>(`/api/targets/${id}`);
  return res.data;
}

export async function createTarget(data: {
  name: string;
  hostname: string;
  port?: number;
  description?: string;
}): Promise<SshTarget> {
  const res = await api.post<SshTarget>('/api/targets', data);
  return res.data;
}

export async function updateTarget(
  id: string,
  data: { name?: string; hostname?: string; port?: number; description?: string }
): Promise<SshTarget> {
  const res = await api.put<SshTarget>(`/api/targets/${id}`, data);
  return res.data;
}

export async function deleteTarget(id: string): Promise<void> {
  await api.delete(`/api/targets/${id}`);
}

export async function listCredentials(targetId: string): Promise<SshCredential[]> {
  const res = await api.get<SshCredential[]>(`/api/targets/${targetId}/credentials`);
  return res.data;
}

export async function createCredential(
  targetId: string,
  data: { username: string; auth_type: string; credential: string }
): Promise<SshCredential> {
  const res = await api.post<SshCredential>(`/api/targets/${targetId}/credentials`, data);
  return res.data;
}

export async function deleteCredential(targetId: string, credId: string): Promise<void> {
  await api.delete(`/api/targets/${targetId}/credentials/${credId}`);
}
