// app/login/page.tsx

"use client";

import { useState } from 'react';
import Link from 'next/link';
import { Github, Lock, Eye, EyeOff, BookOpen } from 'lucide-react';
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
} from "@/components/ui/alert-dialog";

const LoginPage = () => {
  const [showPassword, setShowPassword] = useState(false);
  return (
    <div className="min-h-screen bg-gray-50 font-sans">
      {/* The main grid layout */}
      <div className="grid grid-cols-1 md:grid-cols-2 min-h-screen">
        
        {/* Left Column: The Form */}
        <div className="flex flex-col justify-between p-8 md:p-12">
          {/* Header */}
          <header className="flex justify-between items-center">
            <Link href="/" className="flex items-center gap-2">
              <svg width="24" height="24" viewBox="0 0 76 65" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M38 0L0.240387 64.5H75.7596L38 0Z" fill="#3ECF8E"/>
              </svg>
              <span className="font-bold text-lg">supabase</span>
            </Link>
            <Link href="/docs" className="hidden md:flex items-center gap-2 text-sm border rounded-md px-3 py-1.5 hover:bg-gray-100">
              <BookOpen size={14} />
              ドキュメント
            </Link>
          </header>

          {/* Form Container */}
          <main className="flex items-center justify-center w-full">
            <div className="w-full max-w-sm space-y-6">
              <div>
                <h1 className="text-3xl font-bold">おかえりなさい</h1>
                <p className="text-gray-500 mt-2">アカウントにサインインしてください</p>
              </div>

              {/* Social Logins */}
              <div className="space-y-3">
                <AlertDialog>
                  <AlertDialogTrigger asChild>
                    <button className="w-full flex items-center justify-center gap-2 p-3 border border-green-400 rounded-md hover:bg-gray-50 font-semibold">
                      <Github size={20} />
                      GitHubで続行
                    </button>
                  </AlertDialogTrigger>
                  <AlertDialogContent>
                    <AlertDialogHeader>
                      <AlertDialogTitle>GitHub認証は現在開発中です。</AlertDialogTitle>
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
                
                <AlertDialog>
                  <AlertDialogTrigger asChild>
                    <button className="w-full flex items-center justify-center gap-2 p-3 border rounded-md hover:bg-gray-50 font-semibold">
                      <Lock size={20} />
                      SSOで続行
                    </button>
                  </AlertDialogTrigger>
                  <AlertDialogContent>
                    <AlertDialogHeader>
                      <AlertDialogTitle>SSO認証は現在開発中です。</AlertDialogTitle>
                      <AlertDialogDescription>
                        まだご利用いただけませんが、準備が整い次第アクセスできるようになります。

                        この機能は現在開発中です。しばらくお待ちください。
                      </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                      <AlertDialogCancel>キャンセル</AlertDialogCancel>
                      <AlertDialogAction>閉じる</AlertDialogAction>
                    </AlertDialogFooter>
                  </AlertDialogContent>
                </AlertDialog>
              </div>

              {/* Separator */}
              <div className="flex items-center">
                <hr className="flex-grow border-t" />
                <span className="mx-4 text-gray-400 text-sm">または</span>
                <hr className="flex-grow border-t" />
              </div>

              {/* Email & Password Form */}
              <form className="space-y-4">
                <div>
                  <label htmlFor="email" className="block text-sm font-medium mb-1">メールアドレス</label>
                  <input
                    type="email"
                    id="email"
                    defaultValue="sparkbyte's web"
                    className="w-full p-3 border rounded-md bg-blue-50 border-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-400"
                  />
                </div>
                <div>
                  <div className="flex justify-between items-center mb-1">
                    <label htmlFor="password"className="block text-sm font-medium">パスワード</label>
                    <Link href="/forgot-password"className="text-sm text-gray-600 hover:underline">
                      パスワードを忘れた場合
                    </Link>
                  </div>
                  <div className="relative">
                    <input
                      type={showPassword ? "text" : "password"}
                      id="password"
                      defaultValue="••••••••••••"
                      className="w-full p-3 border rounded-md bg-blue-50 border-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-400"
                    />
                    <button 
                      type="button" 
                      className="absolute inset-y-0 right-0 px-3 flex items-center text-gray-500 hover:text-gray-700"
                      onClick={() => setShowPassword(!showPassword)}
                    >
                      {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                    </button>
                  </div>
                </div>
                <button type="submit" className="w-full bg-green-500 text-white p-3 rounded-md hover:bg-green-600 font-semibold transition-colors">
                  サインイン
                </button>
              </form>
              
              <p className="text-center text-sm text-gray-600">
                アカウントをお持ちでない場合？ <Link href="/signup" className="font-semibold hover:underline">今すぐサインアップ</Link>
              </p>
            </div>
          </main>

          {/* Footer */}
          <footer className="text-center text-xs text-gray-400">
            続行することで、Supabaseの<Link href="/terms" className="underline">利用規約</Link>と<Link href="/privacy" className="underline">プライバシーポリシー</Link>に同意し、更新情報の定期的なメールを受け取ることに同意します。
          </footer>
        </div>

        {/* Right Column: The Testimonial */}
        <div className="hidden md:flex items-center justify-center bg-white p-12">
          <div className="max-w-md text-center relative">
            <span className="absolute -top-12 -left-12 text-9xl text-gray-100 font-serif opacity-80 select-none">“</span>
            <blockquote className="text-2xl font-medium text-gray-800 leading-relaxed">
              このプロジェクトは現在開発中です。ご期待ください。
            </blockquote>
            <footer className="mt-8 flex items-center justify-center">
              <div className="flex items-center justify-center w-12 h-12 bg-green-500 text-white rounded-full font-bold text-xl">
                茅
              </div>
              <div className="ml-4 text-left">
                <p className="font-semibold text-gray-800">茅野</p>
                <p className="text-sm text-gray-600">プロジェクト開発者</p>
              </div>
            </footer>
          </div>
        </div>

      </div>
    </div>
  );
};

export default LoginPage;