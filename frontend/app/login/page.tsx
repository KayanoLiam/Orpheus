"use client";

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';

const Login = () => {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');

    try {
      // 调用后端登录 API
      const response = await axios.post('http://127.0.0.1:8080/login', {
        email,
        password
      });

      if (response.data.success) {
        // 登录成功，保存 session_id 到 localStorage
        // 假设后端返回的 session_id 在 response.data.data.session_id 中
        if (response.data.data?.session_id) {
          localStorage.setItem('session_id', response.data.data.session_id);
        }
        
        // 跳转到 dashboard
        router.push('/dashboard');
      } else {
        setError('登录失败：' + (response.data.message || '未知错误'));
      }
    } catch (error: any) {
      console.error('登录错误:', error);
      setError('登录失败：' + (error.response?.data?.message || error.message || '网络错误'));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 dark:bg-black">
      <div className="max-w-md w-full p-8 bg-white dark:bg-black rounded-lg shadow-lg">
        <h1 className="text-2xl font-bold mb-6 text-center">登录</h1>
        
        <form onSubmit={handleLogin} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">邮箱</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-600"
              placeholder="请输入邮箱"
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium mb-2">密码</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-600"
              placeholder="请输入密码"
              required
            />
          </div>
          
          {error && (
            <div className="text-red-600 text-sm">{error}</div>
          )}
          
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition disabled:opacity-50"
          >
            {loading ? '登录中...' : '登录'}
          </button>
        </form>
        
        <div className="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          还没有账号？
          <a href="/signup" className="text-blue-600 hover:underline ml-1">
            立即注册
          </a>
        </div>
      </div>
    </div>
  );
};

export default Login;