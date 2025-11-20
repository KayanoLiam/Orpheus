// app/login/page.tsx

"use client";

import { useState } from 'react';
import Link from 'next/link';
import { Github, Lock, Eye, EyeOff, BookOpen, ArrowLeft } from 'lucide-react';
import { motion } from "framer-motion";
import HeroBackground from "@/components/ui/3d/HeroBackground";
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
    <div className="min-h-screen bg-background font-sans text-foreground overflow-hidden flex">

      {/* Left Column: The Form */}
      <div className="w-full lg:w-1/2 flex flex-col justify-between p-8 md:p-12 z-10 relative">
        {/* Header */}
        <header className="flex justify-between items-center">
          <Link href="/" className="flex items-center gap-2 group">
            <ArrowLeft className="w-5 h-5 text-muted-foreground group-hover:text-primary transition-colors" />
            <span className="font-bold text-lg tracking-tight">Orpheus</span>
          </Link>
          <Link href="/docs" className="hidden md:flex items-center gap-2 text-sm border border-border rounded-full px-4 py-1.5 hover:bg-white/5 hover:border-primary/50 transition-all">
            <BookOpen size={14} />
            <span>Documentation</span>
          </Link>
        </header>

        {/* Form Container */}
        <main className="flex items-center justify-center w-full flex-grow my-10">
          <div className="w-full max-w-sm space-y-8">
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
            >
              <h1 className="text-4xl font-bold tracking-tight">Welcome back</h1>
              <p className="text-muted-foreground mt-2">Sign in to your account to continue</p>
            </motion.div>

            {/* Social Logins */}
            <motion.div
              className="space-y-3"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.1 }}
            >
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <button className="w-full flex items-center justify-center gap-2 p-3 border border-border rounded-lg hover:bg-white/5 hover:border-primary/50 transition-all font-medium">
                    <Github size={20} />
                    Continue with GitHub
                  </button>
                </AlertDialogTrigger>
                <AlertDialogContent className="glass border-white/10 text-white">
                  <AlertDialogHeader>
                    <AlertDialogTitle>GitHub Auth is under development</AlertDialogTitle>
                    <AlertDialogDescription className="text-gray-400">
                      This feature is not yet available.
                      <br /><br />
                      Clicking "Continue" will take you to the GitHub repository.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel className="bg-transparent border-white/10 text-white hover:bg-white/10">Cancel</AlertDialogCancel>
                    <AlertDialogAction
                      className="bg-primary text-black hover:bg-primary/90"
                      onClick={() => window.location.href = "https://github.com/KayanoLiam/Orpheus.git"}
                    >Continue</AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>

              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <button className="w-full flex items-center justify-center gap-2 p-3 border border-border rounded-lg hover:bg-white/5 hover:border-primary/50 transition-all font-medium">
                    <Lock size={20} />
                    Continue with SSO
                  </button>
                </AlertDialogTrigger>
                <AlertDialogContent className="glass border-white/10 text-white">
                  <AlertDialogHeader>
                    <AlertDialogTitle>SSO Auth is under development</AlertDialogTitle>
                    <AlertDialogDescription className="text-gray-400">
                      This feature is currently being built. Please check back later.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel className="bg-transparent border-white/10 text-white hover:bg-white/10">Cancel</AlertDialogCancel>
                    <AlertDialogAction className="bg-primary text-black hover:bg-primary/90">Close</AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </motion.div>

            {/* Separator */}
            <div className="flex items-center">
              <hr className="flex-grow border-t border-border" />
              <span className="mx-4 text-muted-foreground text-xs uppercase tracking-widest">Or</span>
              <hr className="flex-grow border-t border-border" />
            </div>

            {/* Email & Password Form */}
            <motion.form
              className="space-y-4"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.2 }}
            >
              <div className="space-y-1">
                <label htmlFor="email" className="block text-sm font-medium text-muted-foreground">Email address</label>
                <input
                  type="email"
                  id="email"
                  defaultValue="sparkbyte's web"
                  className="w-full p-3 rounded-lg bg-white/5 border border-border focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary transition-all text-foreground placeholder:text-muted-foreground/50"
                />
              </div>
              <div className="space-y-1">
                <div className="flex justify-between items-center">
                  <label htmlFor="password" className="block text-sm font-medium text-muted-foreground">Password</label>
                  <Link href="/forgot-password" className="text-xs text-primary hover:text-primary/80 transition-colors">
                    Forgot password?
                  </Link>
                </div>
                <div className="relative">
                  <input
                    type={showPassword ? "text" : "password"}
                    id="password"
                    defaultValue="••••••••••••"
                    className="w-full p-3 rounded-lg bg-white/5 border border-border focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary transition-all text-foreground placeholder:text-muted-foreground/50"
                  />
                  <button
                    type="button"
                    className="absolute inset-y-0 right-0 px-3 flex items-center text-muted-foreground hover:text-foreground transition-colors"
                    onClick={() => setShowPassword(!showPassword)}
                  >
                    {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                  </button>
                </div>
              </div>
              <button type="submit" className="w-full bg-primary text-black p-3 rounded-lg hover:bg-primary/90 font-bold transition-all shadow-[0_0_20px_-5px_rgba(var(--primary),0.5)] hover:shadow-[0_0_30px_-5px_rgba(var(--primary),0.6)]">
                Sign In
              </button>
            </motion.form>

            <motion.p
              className="text-center text-sm text-muted-foreground"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.5, delay: 0.3 }}
            >
              Don't have an account? <Link href="/signup" className="font-semibold text-primary hover:underline">Sign up now</Link>
            </motion.p>
          </div>
        </main>

        {/* Footer */}
        <footer className="text-center text-xs text-muted-foreground/50">
          By continuing, you agree to Orpheus's <Link href="/terms" className="underline hover:text-foreground">Terms of Service</Link> and <Link href="/privacy" className="underline hover:text-foreground">Privacy Policy</Link>.
        </footer>
      </div>

      {/* Right Column: The Visuals */}
      <div className="hidden lg:block w-1/2 relative bg-black overflow-hidden">
        <div className="absolute inset-0 z-0">
          <HeroBackground />
        </div>
        <div className="absolute inset-0 bg-gradient-to-l from-transparent to-background z-10" />

        <div className="relative z-20 h-full flex items-center justify-center p-12">
          <div className="max-w-md text-center">
            <motion.blockquote
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.8, delay: 0.4 }}
              className="text-3xl font-medium text-white leading-relaxed"
            >
              "The future of development is here. Orpheus gives you the power to build without limits."
            </motion.blockquote>
            <motion.footer
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: 0.6 }}
              className="mt-8 flex items-center justify-center gap-4"
            >
              <div className="w-12 h-12 bg-primary rounded-full flex items-center justify-center text-black font-bold text-xl">
                K
              </div>
              <div className="text-left">
                <p className="font-semibold text-white">Kayano</p>
                <p className="text-sm text-gray-400">Lead Developer</p>
              </div>
            </motion.footer>
          </div>
        </div>
      </div>

    </div>
  );
};

export default LoginPage;