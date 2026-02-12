import api from './client';
import type { AuditLog } from '../types';

export async function listAuditLogs(): Promise<AuditLog[]> {
  const res = await api.get<AuditLog[]>('/api/audit-logs');
  return res.data;
}
