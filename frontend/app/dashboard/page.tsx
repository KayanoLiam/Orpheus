"use client";

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';

const Dashboard = () => {
  console.log('Dashboard 组件已加载');
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<any>(null);

  useEffect(() => {
    const checkAuthStatus = async () => {
      try {
        // 从 localStorage 获取 session_id
        const sessionId = localStorage.getItem('session_id');
        console.log('检查 session_id:', sessionId);
        
        if (!sessionId) {
          console.log('没有 session_id，跳转到登录页');
          // 没有 session_id，跳转到登录页
          router.push('/login');
          return;
        }

        // 调用后端 /auth/status API 检查登录状态
        console.log('发送认证请求...');
        const response = await axios.get('http://127.0.0.1:8080/auth/status', {
          headers: {
            'Authorization': sessionId
          },
          timeout: 5000 // 5秒超时
        });

        console.log('API 响应:', response.data);

        if (response.data.success) {
          // 已登录，设置用户信息
          setUser(response.data.data);
          setLoading(false);
        } else {
          // API 返回失败，清除无效 session 并跳转登录页
          console.log('API 返回失败:', response.data.message);
          localStorage.removeItem('session_id');
          router.push('/login');
        }
      } catch (error: any) {
        console.error('认证检查失败:', error);
        
        // 显示具体错误信息
        if (error.response) {
          console.error('响应错误:', error.response.status, error.response.data);
        } else if (error.request) {
          console.error('请求错误:', error.message);
        }
        
        // 请求失败，清除可能的无效 session 并跳转登录页
        localStorage.removeItem('session_id');
        router.push('/login');
      }
    };

    // 添加一个延时，确保页面完全加载
    const timer = setTimeout(checkAuthStatus, 100);
    
    return () => clearTimeout(timer);
  }, [router]);

  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="text-lg">加载中...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-zinc-50 dark:bg-black">
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold mb-8 text-center">Dashboard</h1>
        
        {user && (
          <div className="max-w-4xl mx-auto space-y-6">
            {/* 用户信息卡片 */}
            <div className="bg-white dark:bg-black p-6 rounded-lg shadow-lg">
              <h2 className="text-xl font-semibold mb-4">用户信息</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="p-4 bg-gray-100 dark:bg-gray-800 rounded">
                  <p className="text-sm text-gray-600 dark:text-gray-400">用户 ID:</p>
                  <p className="font-mono">{user.user_id}</p>
                </div>
                
                <div className="p-4 bg-gray-100 dark:bg-gray-800 rounded">
                  <p className="text-sm text-gray-600 dark:text-gray-400">登录状态:</p>
                  <p className="text-green-600 font-medium">✓ 已登录</p>
                </div>
              </div>
            </div>
            
            {/* 功能卡片 */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-white dark:bg-black p-6 rounded-lg shadow-lg">
                <h3 className="text-lg font-semibold mb-2">数据分析</h3>
                <p className="text-gray-600 dark:text-gray-400 mb-4">查看您的数据统计和分析报告</p>
                <button className="w-full bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition">
                  查看报告
                </button>
              </div>
              
              <div className="bg-white dark:bg-black p-6 rounded-lg shadow-lg">
                <h3 className="text-lg font-semibold mb-2">账户设置</h3>
                <p className="text-gray-600 dark:text-gray-400 mb-4">管理您的账户信息和偏好设置</p>
                <button className="w-full bg-green-600 text-white py-2 px-4 rounded hover:bg-green-700 transition">
                  设置账户
                </button>
              </div>
              
              <div className="bg-white dark:bg-black p-6 rounded-lg shadow-lg">
                <h3 className="text-lg font-semibold mb-2">安全中心</h3>
                <p className="text-gray-600 dark:text-gray-400 mb-4">管理密码和两步验证等安全设置</p>
                <button className="w-full bg-purple-600 text-white py-2 px-4 rounded hover:bg-purple-700 transition">
                  安全设置
                </button>
              </div>
            </div>
            
            {/* 最近活动 */}
            <div className="bg-white dark:bg-black p-6 rounded-lg shadow-lg">
              <h2 className="text-xl font-semibold mb-4">最近活动</h2>
              <div className="space-y-3">
                <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded">
                  <span className="text-sm">登录成功</span>
                  <span className="text-xs text-gray-500">刚刚</span>
                </div>
                <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded">
                  <span className="text-sm">账户创建</span>
                  <span className="text-xs text-gray-500">2小时前</span>
                </div>
                <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded">
                  <span className="text-sm">邮箱验证</span>
                  <span className="text-xs text-gray-500">昨天</span>
                </div>
              </div>
            </div>
            
            {/* 退出登录按钮 */}
            <div className="text-center">
              <button 
                onClick={() => {
                  localStorage.removeItem('session_id');
                  router.push('/login');
                }}
                className="bg-red-600 text-white py-2 px-6 rounded hover:bg-red-700 transition"
              >
                退出登录
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Dashboard;