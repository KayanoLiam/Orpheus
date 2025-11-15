"use client";

import Link from 'next/link';
import { ChevronDown, Github } from 'lucide-react';
import React, { useState, useEffect } from 'react';
import axios from 'axios';
// import { Button } from '@/components/ui/button';



/**
 * 这是一个从后端获取 GitHub 仓库星标数量的组件
 * Props:
 * - owner: 仓库所有者的用户名
 * - repo: 仓库名称
 * 返回值:
 * 一个显示星标数量的链接按钮
 */
function GitHubStarButton({ owner, repo }: { owner: string; repo: string }) {
  // 保存星标数量的状态，初始值为 null
  const [stars, setStars] = useState<number | null>(null);
  // 保存加载状态的状态，初始值为 false
  const [loading, setLoading] = useState(false);

  // 使用 useEffect 在组件挂载时获取星标数量
  useEffect(() => {
    const fetchStars = async () => {
      setLoading(true);
      try {
        // 调用后端 API 获取星标数量
        const response = await axios.get(`http://127.0.0.1:8080/github/stars/${owner}/${repo}`);
        // 后端返回格式：
        //   code: 200,
        //   success: true,
        //   data: { stars: 0 },
        //   message: "Repository stars fetched successfully"
        // }
        if (response.data && response.data.success) {
          setStars(response.data.data.stars);
        } else {
          console.error('Failed to fetch stars:', response.data.message);
        }
      } catch (error) {
        console.error('Error fetching stars:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchStars();
  }, [owner, repo]);

  // 显示加载状态或星标数量
  const displayStars = loading ? '...' : stars !== null ? stars.toString() : '?';

  return (
    <Link href={`https://github.com/${owner}/${repo}`} target="_blank" className="flex items-center gap-2 border border-gray-600 rounded-md px-3 py-1.5 hover:border-gray-400">
      <Github size={16} />
      <span>{displayStars}</span>
    </Link>
  );
}



const Navbar = () => {
  return (
    <header className="sticky top-0 z-50 bg-black text-white p-4 border-b border-white/10">
      <nav className="container mx-auto flex justify-between items-center">
        {/* 左侧 Logo 和导航链接 */}
        <div className="flex items-center gap-8">
          <Link href="/" className="flex items-center gap-2">
            {/* 你可以在 public 文件夹下放置你的 SVG logo */}
            {/* <img src="/logo.svg" alt="Logo" className="h-8" /> */}
            <span className="font-bold text-xl">Orpheus</span>
          </Link>
          <ul className="hidden md:flex items-center gap-6">
            <li>
              <Link href="/product" className="flex items-center gap-1 hover:text-gray-300">
                製品 <ChevronDown size={16} />
              </Link>
            </li>
            <li>
              <Link href="/developers" className="flex items-center gap-1 hover:text-gray-300">
                開発者 <ChevronDown size={16} />
              </Link>
            </li>
            <li>
              <Link href="/solutions" className="flex items-center gap-1 hover:text-gray-300">
                ソリューション <ChevronDown size={16} />
              </Link>
            </li>
            <li><Link href="/pricing" className="hover:text-gray-300">料金</Link></li>
            <li><Link href="/docs" className="hover:text-gray-300">ドキュメント</Link></li>
            <li><Link href="/blog" className="hover:text-gray-300">ブログ</Link></li>
          </ul>
        </div>

        {/* 右侧部分 */}
        <div className="hidden md:flex items-center gap-4">
          <GitHubStarButton owner="KayanoLiam" repo="Orpheus" />
          <Link href="/dashboard">
            <button className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700">
              ダッシュボード
            </button>

          </Link>
          {/* <img src="/avatar.jpg" alt="User Avatar" className="h-8 w-8 rounded-full" /> */}
        </div>

        {/* 移动端菜单按钮 (可选) */}
        <div className="md:hidden">
          {/* 这里可以放置一个汉堡菜单按钮 */}
        </div>
      </nav>
    </header>
  );
};

export default Navbar;