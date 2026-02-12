import api from './client';
import type { LoginResponse } from '../types';

export async function login(username: string, password: string): Promise<LoginResponse> {
  const res = await api.post<LoginResponse>('/api/auth/login', { username, password });
  return res.data;
}

export async function register(username: string, email: string, password: string): Promise<LoginResponse> {
  const res = await api.post<LoginResponse>('/api/auth/register', { username, email, password });
  return res.data;
}
