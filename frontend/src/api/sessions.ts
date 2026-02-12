import api from './client';
import type { Session } from '../types';

export async function listSessions(): Promise<Session[]> {
  const res = await api.get<Session[]>('/api/sessions');
  return res.data;
}
