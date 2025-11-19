"use client";
import React from 'react';
import { motion } from "framer-motion";
import HeroBackground from "@/components/ui/3d/HeroBackground";
import { ArrowRight, CheckCircle2 } from "lucide-react";
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

const LogoPlaceholder = ({ name }: { name: string }) => (
  <div className="text-muted-foreground/50 text-lg font-medium hover:text-primary transition-colors duration-300 cursor-default">
    {name}
  </div>
);

export default function App() {
  return (
    <div className="relative min-h-screen text-foreground overflow-hidden selection:bg-primary/20">
      <HeroBackground />

      <HeroSection />
    </div>
  );
}

function HeroSection() {
  return (
    <section className="relative z-10 flex flex-col items-center justify-center min-h-screen pt-20">
      <div className="container mx-auto px-6 max-w-5xl text-center">

        {/* Badge */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 border border-primary/20 text-primary text-xs font-medium mb-8"
        >
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-primary"></span>
          </span>
          v1.0 Public Beta is Live
        </motion.div>

        {/* Main Title */}
        <motion.h1
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.1 }}
          className="text-6xl md:text-8xl font-bold tracking-tighter text-white mb-6"
        >
          Build in a <span className="text-transparent bg-clip-text bg-gradient-to-r from-primary to-emerald-200">Weekend.</span>
          <br />
          Scale to <span className="text-transparent bg-clip-text bg-gradient-to-r from-emerald-200 to-primary">Millions.</span>
        </motion.h1>

        {/* Subtitle */}
        <motion.p
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.2 }}
          className="mt-6 text-xl md:text-2xl text-muted-foreground max-w-2xl mx-auto leading-relaxed"
        >
          Orpheus is the open source Supabase alternative.
          <br className="hidden md:block" />
          The power of Postgres with the ease of a BaaS.
        </motion.p>

        {/* Buttons */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.3 }}
          className="mt-10 flex flex-col sm:flex-row justify-center gap-4"
        >
          <a
            href="#"
            className="group relative px-8 py-4 rounded-full bg-primary text-black font-bold text-lg shadow-[0_0_40px_-10px_rgba(var(--primary),0.5)] hover:shadow-[0_0_60px_-10px_rgba(var(--primary),0.7)] hover:scale-105 transition-all duration-300"
          >
            Start Project
            <ArrowRight className="inline-block ml-2 w-5 h-5 group-hover:translate-x-1 transition-transform" />
          </a>

          <AlertDialog>
            <AlertDialogTrigger asChild>
              <button className="px-8 py-4 rounded-full glass text-white font-semibold border border-white/10 hover:bg-white/10 hover:scale-105 transition-all duration-300">
                Request Demo
              </button>
            </AlertDialogTrigger>
            <AlertDialogContent className="glass border-white/10 text-white">
              <AlertDialogHeader>
                <AlertDialogTitle>Demo Unavailable</AlertDialogTitle>
                <AlertDialogDescription className="text-gray-400">
                  The demo environment is currently being provisioned.
                  <br /><br />
                  Click "Continue" to visit our GitHub repository in the meantime.
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
        </motion.div>

        {/* Feature List (Mini) */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 1, delay: 0.5 }}
          className="mt-12 flex flex-wrap justify-center gap-6 text-sm text-gray-500"
        >
          {['Postgres Database', 'Authentication', 'Edge Functions', 'Realtime Subscriptions'].map((feature) => (
            <div key={feature} className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-primary" />
              <span>{feature}</span>
            </div>
          ))}
        </motion.div>

        {/* Trusted By */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 1, delay: 0.8 }}
          className="mt-24 border-t border-white/5 pt-10 w-full"
        >
          <p className="text-xs uppercase tracking-widest text-gray-600 mb-8">
            Trusted by high-growth teams
          </p>
          <div className="flex justify-center items-center gap-12 flex-wrap opacity-50 grayscale hover:grayscale-0 transition-all duration-500">
            <LogoPlaceholder name="LangChain" />
            <LogoPlaceholder name="Resend" />
            <LogoPlaceholder name="Loops" />
            <LogoPlaceholder name="Mobbin" />
            <LogoPlaceholder name="Gopuff" />
          </div>
        </motion.div>

      </div>
    </section>
  );
}