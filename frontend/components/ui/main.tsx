"use client";
import React from 'react';
// 在 Next.js 项目中，你通常不需要导入 React
// 假设你已经配置好了 Tailwind CSS

// 这是一个模拟的 Logo 组件，用于展示“Trusted by”部分
// 在真实项目中，你可能会使用 next/image 或 <img> 来加载 SVG



import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"

const LogoPlaceholder = ({ name }) => (
  <div className="text-gray-500 text-lg font-medium">
    {name}
  </div>
);

export default function App() {
  return (
    <div className="bg-black min-h-screen text-white p-4">
      {/* HeroSection 组件
        这是你询问的中间区域的实现。
        我们使用 Tailwind CSS 来实现布局和样式。
      */}
      <HeroSection />
    </div>
  );
}

function HeroSection() {
  return (
    <section className="flex flex-col items-center justify-center py-20 md:py-32">
      <div className="container mx-auto px-6 max-w-4xl text-center">

        {/* 主要标题 */}
        <h1 className="text-5xl md:text-7xl font-bold tracking-tight text-gray-100">
          週末で構築
        </h1>
        <h1 className="text-5xl md:text-7xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-green-400 to-green-500 mt-2">
          数百万ユーザーまで拡張可能
        </h1>

        {/* 副标题 */}
        <p className="mt-8 text-xl md:text-2xl text-gray-300 max-w-2xl mx-auto">
          OrpheusはPostgresベースの開発プラットフォームです
        </p>

        {/* 描述文本 */}
        <p className="mt-6 text-lg text-gray-400 max-w-2xl mx-auto">
          Postgresデータベース、認証、即時API、Edge Functions、リアルタイム購読、ストレージ、ベクトル埋め込みを使ってプロジェクトを開始できます。        </p>

        {/* 按钮组 */}
        <div className="mt-10 flex flex-col sm:flex-row justify-center gap-4">
          <a
            href="#"
            className="px-8 py-3 rounded-lg bg-green-500 text-black font-semibold shadow-lg hover:bg-green-400 transition-colors duration-200"
          >
            プロジェクトを始める
          </a>
          {/* <a
            href="#"
            className="px-8 py-3 rounded-lg bg-gray-800 text-white font-semibold border border-gray-700 hover:bg-gray-700 transition-colors duration-200"
          >
            デモを依頼する
          </a> */}
          <AlertDialog>
            <AlertDialogTrigger asChild>
              <button className="px-8 py-3 rounded-lg bg-gray-800 text-white font-semibold border border-gray-700 hover:bg-gray-700 transition-colors duration-200">
                デモを依頼する
              </button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>このデモ機能は現在開発中です。</AlertDialogTitle>
                <AlertDialogDescription>
                  まだご利用いただけませんが、準備が整い次第アクセスできるようになります。

                  「続行」をクリックすると、このプロジェクトの GitHub リポジトリへ移動します。
                  しばらくお待ちください。
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>キャンセル</AlertDialogCancel>
                <AlertDialogAction
                  onClick={() => window.location.href = "https://github.com/KayanoLiam/Orpheus.git"}
                >続行</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>

        {/* “Trusted by” 部分 */}
        <div className="mt-20">
          <p className="text-sm text-gray-500">
            世界中の急成長企業に信頼されています
          </p>
          {/* Logo 列表 */}
          <div className="mt-8 flex justify-center items-center gap-8 md:gap-12 flex-wrap">
            {/* 在真实的应用中，这里会是 Logo 图片或 SVG */}
            <LogoPlaceholder name="LangChain" />
            <LogoPlaceholder name="Resend" />
            <LogoPlaceholder name="Loops" />
            <LogoPlaceholder name="Mobbin" />
            <LogoPlaceholder name="gopuff" />
          </div>
        </div>

      </div>
    </section>
  );
}